use std::sync::Arc;

use color_eyre::eyre::{Result, eyre};
use tokio::sync::OnceCell;

use crate::{
    error::DiscordClientInitializationError, platforms::discord::structs::DiscordClient,
    utils::get_discord_headermap,
};

pub const DISCORD_CLIENT: OnceCell<Arc<DiscordClient>> = OnceCell::const_new();

async fn init_discord_client() -> Result<Arc<DiscordClient>> {
    let headers = get_discord_headermap().await?;

    let client = match DiscordClient::init(headers).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return Err(eyre!(DiscordClientInitializationError::new(e.to_string())));
        }
    };

    Ok(Arc::new(client))
}
pub async fn const_get_discord_client() -> Result<Arc<DiscordClient>> {
    Ok(DISCORD_CLIENT
        .get_or_try_init(async || init_discord_client().await)
        .await?
        .clone())
}
