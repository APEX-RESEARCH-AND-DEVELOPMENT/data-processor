use color_eyre::eyre::{Result, eyre};
use encoding_rs::UTF_8;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::{path::PathBuf, str::FromStr};
use tokio::fs::{read, try_exists};

use crate::{
    env::Environment,
    error::{DecodingError, FileExtensionError, FileNotFoundError},
};

pub async fn file_exists(filepath: &PathBuf) -> Result<()> {
    match try_exists(filepath).await {
        Ok(exists) => {
            if exists {
                return Ok(());
            } else {
                return Err(eyre!(FileNotFoundError::new(
                    filepath.to_string_lossy().to_string()
                )));
            }
        }
        Err(e) => {
            return Err(eyre!(
                format!("UNKNOWN ERROR OCCURRED: {:#?}", e).to_string()
            ));
        }
    }
}

pub async fn file_extension_matches(filepath: &PathBuf, expected_extension: &str) -> Result<()> {
    let actual_extension = filepath.extension();

    let found_extension_lossy = actual_extension
        .map(|ext| ext.to_string_lossy().to_string())
        .unwrap_or_else(|| "UNKNOWN_EXTENSION".to_string());

    match actual_extension {
        Some(ext) if ext.eq_ignore_ascii_case(expected_extension) => Ok(()),
        _ => Err(eyre!(FileExtensionError::new(
            expected_extension.to_string(),
            found_extension_lossy
        ))),
    }
}

pub async fn get_discord_headermap() -> Result<HeaderMap> {
    let env = Environment::read()?;

    let skip = ["authority", "method", "path", "scheme"];
    let mut headers = HeaderMap::new();

    let file = {
        let buf = read(env.discord.auth_file.clone()).await?;

        let (text, _, error) = UTF_8.decode(&buf);

        if error {
            return Err(eyre!(DecodingError::new(env.discord.auth_file.clone())));
        }

        text.to_string()
    };

    for line in file.lines() {
        let line = line.trim();

        if line.is_empty()
            || line.contains("$session")
            || line.contains("Invoke-WebRequest")
            || !line.contains("=")
        {
            continue;
        }

        let mut parts = line.splitn(2, "=");
        let key = parts.next().unwrap().trim().trim_matches('"');
        if skip.iter().any(|&k| k.eq_ignore_ascii_case(key)) {
            continue;
        }

        let raw_value = parts.next().unwrap().trim();

        let value = raw_value.trim_matches('"').trim_matches('`');

        headers.insert(
            HeaderName::from_str(key)
                .map_err(|e| eyre!(format!("Issue with key '{}': {}", key, e)))?,
            HeaderValue::from_str(value)?,
        );
    }

    Ok(headers)
}
