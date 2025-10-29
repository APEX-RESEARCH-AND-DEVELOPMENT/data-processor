use chrono::DateTime;
use color_eyre::eyre::{Result, eyre};

use crate::{
    arg::{
        ApplicationCommands, DiscordCommands, PlatformCommands, TelegramCommands,
        const_get_application_arguments,
    },
    error::DateTimeParseError,
    platforms::{
        discord::methods::dump_messages_for_channel,
        telegram::methods::{dump_messages, resolve_usernames},
    },
    utils::{file_exists, file_extension_matches},
};

pub async fn program_loader() -> Result<()> {
    let args = const_get_application_arguments().await?;

    match &args.command {
        ApplicationCommands::Data { platform } => match platform {
            PlatformCommands::Telegram { method } => match method {
                TelegramCommands::ResolveUsers { usernames } => {
                    file_exists(usernames).await?;
                    file_extension_matches(usernames, "txt").await?;

                    resolve_usernames(usernames.to_path_buf()).await?;
                }
                TelegramCommands::DumpMessages {
                    users_file,
                    limit,
                    date_point,
                    reverse,
                } => {
                    if *reverse {
                        todo!(
                            "Reverse operations via the Telegram library is not possible at this moment."
                        )
                    }

                    file_exists(users_file).await?;
                    file_extension_matches(users_file, "json").await?;

                    let date = match DateTime::from_timestamp_secs(*date_point) {
                        Some(date) => date,
                        None => {
                            return Err(eyre!(DateTimeParseError::new(
                                date_point.to_string(),
                                "NO DATE_POINT FOUND AFTER PARSE".to_string()
                            )));
                        }
                    };

                    dump_messages(users_file.to_path_buf(), date, *limit, *reverse).await?;
                }
            },
            PlatformCommands::Discord { method } => match method {
                DiscordCommands::DumpMessages {
                    targets_file,
                    limit,
                    date_point,
                    reverse,
                } => {
                    file_exists(targets_file).await?;
                    file_extension_matches(targets_file, "txt").await?;

                    let date = match DateTime::from_timestamp_secs(*date_point) {
                        Some(date) => date,
                        None => {
                            return Err(eyre!(DateTimeParseError::new(
                                date_point.to_string(),
                                "NO DATE_POINT FOUND AFTER PARSE".to_string()
                            )));
                        }
                    };

                    dump_messages_for_channel(targets_file.to_path_buf(), date, *limit, *reverse)
                        .await?;
                }
            },
        },
    }

    Ok(())
}
