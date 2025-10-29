use std::time::Duration;

use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, eyre};
use grammers_client::{
    Client, Config, InitParams,
    session::Session,
    types::{Chat, Message},
};
use indicatif::ProgressBar;
use inquire::Text;
use tokio::time::sleep;

use crate::{env::Environment, error::UsernameResolveError, platforms::structs::PeerMessage};

#[derive(Debug)]
pub struct TelegramClient(pub Client);

impl TelegramClient {
    pub async fn init() -> Result<Self> {
        let env = Environment::read()?;

        let client = Client::connect(Config {
            session: Session::load_file_or_create(env.telegram.session_path.clone())?,
            api_id: env.telegram.api_id.clone() as i32,
            api_hash: env.telegram.api_hash.clone(),
            params: InitParams::default(),
        })
        .await?;

        if !client.is_authorized().await? {
            let phone = Text::new("Enter Phone (INTL): ").prompt()?;
            let token = client.request_login_code(&phone).await?;
            let code = Text::new("Enter Code Sent: ").prompt()?;

            match client.sign_in(&token, &code).await {
                Ok(_) => {}
                Err(e) => match e {
                    grammers_client::SignInError::PasswordRequired(ptoken) => {
                        let pass = Text::new(&format!(
                            "Please enter 2FA password [Hint: {}] [ECHO]: ",
                            &ptoken.hint().unwrap_or_default()
                        ))
                        .prompt()?;
                        client.check_password(ptoken, pass).await?;
                    }
                    e => {
                        eprintln!("Unknown Sign In Error: {}", e);
                        client.sign_out().await?;
                    }
                },
            }

            client
                .session()
                .save_to_file(env.telegram.session_path.clone())?;
        }

        Ok(TelegramClient(client))
    }

    pub async fn resolve_username(
        &self,
        username: String,
        progress: Option<ProgressBar>,
    ) -> Result<Chat> {
        let chat = match self.0.resolve_username(&username).await {
            Ok(op) => match op {
                Some(c) => {
                    if let Some(ref prog) = progress {
                        prog.inc(1);
                    }
                    c
                }
                None => {
                    if let Some(ref prog) = progress {
                        prog.finish_with_message(format!("{} - Failed", username.clone()));
                    }
                    return Err(eyre!(UsernameResolveError::new(
                        username,
                        "No viable chat found".to_string()
                    )));
                }
            },
            Err(e) => {
                if let Some(ref prog) = progress {
                    prog.finish_with_message(format!("{} - Failed", username.clone()));
                }
                return Err(eyre!(UsernameResolveError::new(
                    username,
                    format!("Unable to resolve username ({})", e).to_string()
                )));
            }
        };

        if let Some(ref prog) = progress {
            prog.finish_with_message(format!("{} - Resolved", username.clone()));
        }

        Ok(chat)
    }

    pub async fn dump_username(
        &self,
        username: String,
        limit: Option<u32>,
        date_point: DateTime<Utc>,
        reverse: bool,
        progress: Option<ProgressBar>,
    ) -> Result<Vec<PeerMessage>> {
        let user = self.resolve_username(username.clone(), None).await?;

        let mut messages: Vec<PeerMessage> = vec![];

        let date_point_as_utimestamp = date_point.timestamp();

        let mut chunks = match limit {
            Some(lim) => self.0.iter_messages(&user).limit(lim as usize),
            None => self.0.iter_messages(&user),
        };

        'message_loop: loop {
            let fetch = chunks.next().await;
            match fetch {
                Ok(Some(msg)) => {
                    let message: Message = msg;

                    let id = message.id().to_string();
                    let user_id = match message.sender() {
                        Some(sender) => format!("user{}", sender.id()).to_string(),
                        None => "userXXX".to_string(),
                    };
                    let content = message.text().to_string();
                    let date = message.date();

                    messages.push(PeerMessage::new(id, user_id, content, date.clone()));
                    if let Some(ref prog) = progress {
                        prog.inc(1);
                    }

                    if date.timestamp() <= date_point_as_utimestamp {
                        break 'message_loop;
                    }
                }
                Ok(None) => {
                    break 'message_loop;
                }
                Err(e) => match e {
                    grammers_client::InvocationError::Rpc(rpc_error) => {
                        if rpc_error.code == 420 {
                            let time = match rpc_error.value {
                                Some(val) => val.to_string(),
                                None => "DEFAULT_30".to_string(),
                            };

                            if let Some(ref prog) = progress {
                                prog.set_message(format!(
                                    "{} - Flood Wait For {} Seconds",
                                    &username, &time
                                ))
                            };

                            sleep(Duration::from_secs(rpc_error.value.unwrap_or(30) as u64)).await;

                            if let Some(ref prog) = progress {
                                prog.set_message(username.clone())
                            };
                        }
                    }
                    e => {
                        if let Some(ref prog) = progress {
                            prog.finish_and_clear()
                        };
                        return Err(eyre!(e));
                    }
                },
            }
        }

        if let Some(ref prog) = progress {
            prog.finish_with_message(format!("{} - Dumped", &username));
        }

        Ok(messages)
    }
}
