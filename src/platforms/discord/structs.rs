use std::time::Duration;

use color_eyre::eyre::Result;
use indicatif::ProgressBar;
use reqwest::{Client, header::HeaderMap};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use tokio::time::sleep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub owner_id: String,
    pub permissions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialGuild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub owner: Option<bool>,
    pub permissions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: User,
    pub content: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub guild_id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub kind: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMChannel {
    pub id: String,
    pub last_message_id: Option<String>,
    pub recipients: Vec<User>,
    #[serde(rename = "type")]
    pub kind: u8,
}

#[derive(Debug, Clone)]
pub enum DiscordAPIRoutes {
    Users(String),
    Guilds(String),
    GuildChannels(String),
    Messages {
        id: String,
        limit: usize,
        before: Option<String>,
        after: Option<String>,
        around: Option<String>,
    },
    JoinedGuilds,
    JoinedDMChannels,
}

impl DiscordAPIRoutes {
    fn build(&self) -> String {
        match self {
            DiscordAPIRoutes::Guilds(id) => format!("https://discord.com/api/v10/guilds/{}", id),
            DiscordAPIRoutes::Users(id) => format!("https://discord.com/api/v10/users/{}", id),
            DiscordAPIRoutes::GuildChannels(id) => {
                format!("https://discord.com/api/v10/guilds/{}/channels", id)
            }
            DiscordAPIRoutes::Messages {
                id,
                limit,
                before,
                after,
                around,
            } => {
                let inital = format!(
                    "https://discord.com/api/v10/channels/{}/messages?limit={}",
                    id, limit
                );

                if let Some(bval) = before {
                    let final_string = format!("{}&before={}", inital, bval);
                    return final_string;
                }
                if let Some(aval) = after {
                    let final_string = format!("{}&after={}", inital, aval);
                    return final_string;
                }
                if let Some(arval) = around {
                    let final_string = format!("{}&around={}", inital, arval);
                    return final_string;
                }

                return inital;
            }
            DiscordAPIRoutes::JoinedGuilds => {
                "https://discord.com/api/v10/users/@me/guilds".to_string()
            }
            DiscordAPIRoutes::JoinedDMChannels => {
                "https://discord.com/api/v10/users/@me/channels".to_string()
            }
        }
    }

    async fn fetch(&self, headers: HeaderMap) -> Result<String> {
        let url = self.build();
        let client = Client::new();

        let res = client
            .get(url.clone())
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }
}

#[derive(Debug)]
pub struct DiscordClient {
    _headers: HeaderMap,
}

impl DiscordClient {
    pub async fn init(headers: HeaderMap) -> Result<Self> {
        Ok(Self { _headers: headers })
    }

    pub async fn get_messages(
        &self,
        channel: String,
        limit: usize,
        progress: Option<ProgressBar>,
    ) -> Result<Vec<Message>> {
        let mut total_list: Vec<Message> = vec![];

        if limit <= 100 {
            let mut messages = from_str::<Vec<Message>>(
                &DiscordAPIRoutes::Messages {
                    id: channel.clone(),
                    limit,
                    before: None,
                    after: None,
                    around: None,
                }
                .fetch(self._headers.clone())
                .await?,
            )?;

            if let Some(ref prog) = progress {
                prog.inc(messages.len() as u64);
            }

            total_list.append(&mut messages);
        } else {
            let mut before = None;

            while total_list.len() < limit {
                let msg_str = DiscordAPIRoutes::Messages {
                    id: channel.clone(),
                    limit: 100usize,
                    before: before,
                    after: None,
                    around: None,
                }
                .fetch(self._headers.clone())
                .await?;

                let mut messages = from_str::<Vec<Message>>(&msg_str)?;

                if let Some(ref prog) = progress {
                    prog.inc(messages.len() as u64);
                }

                if messages.len() < 100usize {
                    total_list.append(&mut messages);
                    break;
                }

                before = Some(messages[messages.len() - 1usize].id.clone());
                total_list.append(&mut messages);

                sleep(Duration::from_millis(250)).await;
            }
        }

        if let Some(ref prog) = progress {
            prog.finish_with_message(format!("{} - Dumped", channel.clone()));
        }

        Ok(total_list)
    }
}
