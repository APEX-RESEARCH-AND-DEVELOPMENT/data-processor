use crate::platforms::telegram::structs::TelegramClient;
use color_eyre::eyre::Result;
use std::sync::Arc;
use tokio::sync::OnceCell;

pub const TELEGRAM_CLIENT: OnceCell<Arc<TelegramClient>> = OnceCell::const_new();

async fn init_telegram_client() -> Result<Arc<TelegramClient>> {
    Ok(Arc::new(TelegramClient::init().await?))
}

pub async fn const_get_telegram_client() -> Result<Arc<TelegramClient>> {
    Ok(TELEGRAM_CLIENT
        .get_or_try_init(async || init_telegram_client().await)
        .await?
        .clone())
}
