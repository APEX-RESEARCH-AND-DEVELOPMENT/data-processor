use std::{path::PathBuf, sync::Arc};

use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, eyre};
use encoding_rs::UTF_8;
use futures::future::try_join_all;
use indicatif::ProgressBar;
use serde_json::to_string;
use tokio::fs::{read, write};

use crate::{
    error::DecodingError,
    platforms::{
        discord::{client::const_get_discord_client, structs::DiscordClient},
        structs::{DumpedPeer, PeerMessage, ResolvedPeer},
    },
    visual::new_multi_progress,
};

async fn dump_for_single_channel(
    client: Arc<DiscordClient>,
    channel: String,
    date_point: DateTime<Utc>,
    limit: u64,
    reverse: bool,
    progress: Option<ProgressBar>,
) -> Result<DumpedPeer> {
    let messages = client
        .get_messages(channel.clone(), limit as usize, progress)
        .await?
        .iter()
        .map(|m| {
            PeerMessage::new(
                m.id.clone(),
                format!("user{}", m.author.id.clone()),
                m.content.clone(),
                DateTime::parse_from_rfc3339(&m.timestamp)
                    .unwrap()
                    .with_timezone(&Utc),
            )
        })
        .collect::<Vec<PeerMessage>>();

    Ok(DumpedPeer::new(
        ResolvedPeer::new(channel.clone(), channel.clone()),
        messages,
    ))
}

pub async fn dump_messages_for_channel(
    targets_file: PathBuf,
    date_point: DateTime<Utc>,
    limit: Option<u32>,
    reverse: bool,
) -> Result<()> {
    let execution_time = Utc::now();

    let target_channels = {
        let buf = read(&targets_file).await?;
        let (string, _, error) = UTF_8.decode(&buf);

        if error {
            return Err(eyre!(DecodingError::new(
                targets_file.to_string_lossy().to_string()
            )));
        }

        string
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    };

    let client = const_get_discord_client().await?;

    let (multiprog, style) = new_multi_progress()?;

    let actual_limit = match limit {
        Some(lim) => lim as u64,
        None => 1e+8 as u64,
    };

    let futures = target_channels
        .iter()
        .map(|c| {
            let progress = multiprog.add(ProgressBar::new(actual_limit));
            progress.set_style(style.clone());
            progress.set_message(c.clone());
            dump_for_single_channel(
                client.clone(),
                c.to_string(),
                date_point.clone(),
                actual_limit.clone(),
                reverse,
                Some(progress),
            )
        })
        .collect::<Vec<_>>();

    let results = try_join_all(futures).await?;

    write(
        format!(
            "discord_dumped_peers_{}_{}.json",
            results.len(),
            &&execution_time
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                .replace("+", "_")
                .replace(":", "_")
        ),
        to_string(&results)?,
    )
    .await?;

    Ok(())
}
