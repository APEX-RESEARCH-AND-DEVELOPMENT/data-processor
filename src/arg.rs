use std::{path::PathBuf, sync::Arc};

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use tokio::sync::OnceCell;

//* TYPEDEF */
#[derive(Debug, Subcommand)]
pub enum TelegramCommands {
    ResolveUsers {
        /// .txt file containing usernames, separated by newlines
        #[arg(short, long, value_name = "TEXT_FILE")]
        usernames: PathBuf,
    },
    DumpMessages {
        /// .json file containing resolved usernames, outputted by the resolve-users command
        #[arg(short, long, value_name = "JSON_FILE")]
        users_file: PathBuf,

        /// Arbitrary limit to number of messages to be dumped starting from date_point in either direction.
        #[arg(short, long)]
        limit: Option<u32>,

        /// Unix timestamp to serve as a start point, default behaviour is to get the most recent messages starting from this date.
        #[arg(short, long)]
        date_point: i64,

        /// Reverse default behaviour. If date_point is set whilst this is true, then all messages before the date_point will be retrieved. Ignored when limit is set
        #[arg(short, long, default_value_t = false)]
        reverse: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum DiscordCommands {
    DumpMessages {
        /// .txt file containing channel ids
        #[arg(short, long, value_name = "TEXT_FILE")]
        targets_file: PathBuf,

        /// Arbitrary limit to number of messages to be dumped starting from date_point in either direction.
        #[arg(short, long)]
        limit: Option<u32>,

        /// Unix timestamp to serve as a start point, default behaviour is to get the most recent messages starting from this date.
        #[arg(short, long)]
        date_point: i64,

        /// Reverse default behaviour. If date_point is set whilst this is true, then all messages before the date_point will be retrieved. Ignored when limit is set
        #[arg(short, long, default_value_t = false)]
        reverse: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum PlatformCommands {
    Telegram {
        #[command(subcommand)]
        method: TelegramCommands,
    },
    Discord {
        #[command(subcommand)]
        method: DiscordCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum ApplicationCommands {
    Data {
        #[command(subcommand)]
        platform: PlatformCommands,
    },
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct ApplicationArguments {
    #[command(subcommand)]
    pub command: ApplicationCommands,
}
//* END TYPEDEF */
pub const APPLICATION_ARGUMENTS: OnceCell<Arc<ApplicationArguments>> = OnceCell::const_new();

fn init_application_argument() -> Result<Arc<ApplicationArguments>> {
    Ok(Arc::new(ApplicationArguments::try_parse()?))
}
pub async fn const_get_application_arguments() -> Result<Arc<ApplicationArguments>> {
    Ok(APPLICATION_ARGUMENTS
        .get_or_try_init(async || init_application_argument())
        .await?
        .clone())
}
