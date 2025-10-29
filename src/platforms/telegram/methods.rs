use chrono::{DateTime, Utc};
use encoding_rs::UTF_8;
use futures::future::try_join_all;
use indicatif::ProgressBar;
use serde_json::{from_slice, to_string};
use std::{path::PathBuf, sync::Arc};
use tokio::{
    fs::{read, write},
    sync::Semaphore,
};

use color_eyre::eyre::{Result, eyre};

use crate::{
    error::{DecodingError, DeserializationError},
    platforms::structs::{DumpedPeer, ResolvedPeer},
    platforms::telegram::client::const_get_telegram_client,
    platforms::telegram::structs::TelegramClient,
    visual::new_multi_progress,
};

async fn resolve_single_username(
    client: Arc<TelegramClient>,
    username: String,
    progress: Option<ProgressBar>,
) -> Result<ResolvedPeer> {
    let user = client.resolve_username(username, progress).await?;

    Ok(ResolvedPeer::new(
        user.id().to_string(),
        user.username().unwrap_or("PRIVATE_USERNAME").to_string(),
    ))
}

async fn dump_for_single_username(
    semaphore: Arc<Semaphore>,
    client: Arc<TelegramClient>,
    peer: ResolvedPeer,
    date_point: DateTime<Utc>,
    limit: Option<u32>,
    reverse: bool,
    execution_time: DateTime<Utc>,
    progress: Option<ProgressBar>,
) -> Result<()> {
    if let Some(ref prog) = progress {
        prog.set_message(peer.peer_username.clone());

        if semaphore.available_permits() == 0usize {
            prog.set_message(format!(
                "{} - Awaiting permit...",
                peer.peer_username.clone()
            ));
        }
    }

    let permit = semaphore.acquire().await?;

    if let Some(ref prog) = progress {
        prog.set_message(peer.peer_username.clone());
    }

    let messages = client
        .dump_username(
            peer.peer_username.clone(),
            limit,
            date_point,
            reverse,
            progress,
        )
        .await?;

    drop(permit);

    let dumped_peer = DumpedPeer::new(peer.clone(), messages);

    let actual_limit = if let Some(lim) = limit {
        format!("_{}", lim.to_string())
    } else {
        "".to_string()
    };

    write(
        format!(
            "telegram_{}{}_{}.json",
            peer.peer_username.clone(),
            actual_limit,
            &&execution_time
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                .replace("+", "_")
                .replace(":", "_")
        ),
        to_string(&dumped_peer)?,
    )
    .await?;

    Ok(())
}

pub async fn resolve_usernames(filepath: PathBuf) -> Result<()> {
    let execution_time = Utc::now();

    let usernames = {
        let buf = read(&filepath).await?;
        let (string, _, error) = UTF_8.decode(&buf);

        if error {
            return Err(eyre!(DecodingError::new(
                filepath.to_string_lossy().to_string()
            )));
        }

        string
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    };

    let client = const_get_telegram_client().await?;

    let (multiprog, style) = new_multi_progress()?;

    let futures = usernames
        .iter()
        .map(|username| {
            let progress = multiprog.add(ProgressBar::new(1));
            progress.set_style(style.clone());
            progress.set_message(username.clone());
            resolve_single_username(client.clone(), username.clone(), Some(progress))
        })
        .collect::<Vec<_>>();

    let resolved = try_join_all(futures).await?;

    write(
        format!(
            "telegram_resolved_peers_{}_{}.json",
            resolved.len(),
            &&execution_time
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                .replace("+", "_")
                .replace(":", "_")
        ),
        to_string(&resolved)?,
    )
    .await?;

    Ok(())
}

pub async fn dump_messages(
    filepath: PathBuf,
    date_point: DateTime<Utc>,
    limit: Option<u32>,
    reverse: bool,
) -> Result<()> {
    let execution_time = Utc::now();

    let usernames = {
        let buf = read(&filepath).await?;
        match from_slice::<Vec<ResolvedPeer>>(&buf) {
            Ok(data) => data,
            Err(e) => {
                return Err(eyre!(DeserializationError::new(
                    filepath.to_string_lossy().to_string(),
                    e.to_string()
                )));
            }
        }
    };

    let client = const_get_telegram_client().await?;

    let (multiprog, style) = new_multi_progress()?;

    let implied_limit = match limit {
        Some(lim) => lim as u64,
        None => 1e+8 as u64,
    };

    let semaphore = Arc::new(Semaphore::new(3));

    let futures = usernames
        .iter()
        .map(|p| {
            let sclone = semaphore.clone();
            let progress = multiprog.add(ProgressBar::new(implied_limit));
            progress.set_style(style.clone());
            progress.set_message(format!("{} - Awaiting to start", p.peer_username.clone()));
            dump_for_single_username(
                sclone,
                client.clone(),
                p.clone(),
                date_point.clone(),
                limit.clone(),
                reverse,
                execution_time.clone(),
                Some(progress),
            )
        })
        .collect::<Vec<_>>();

    try_join_all(futures).await?;

    Ok(())
}
