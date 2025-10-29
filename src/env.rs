use std::{env, fmt::Display};

use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TelegramEnvironment {
    pub api_id: u32,
    pub api_hash: String,
    pub session_path: String,
}

impl Display for TelegramEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[-- TELEGRAM ENVIRONMENT --]\nAPI_ID: {}\nAPI_HASH: {}\n",
            self.api_id, self.api_hash
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordEnvironment {
    pub auth_file: String,
}

impl Display for DiscordEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[-- DISCORD ENVIRONMENT --]\nAUTH_FILE: {}\n",
            self.auth_file
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    pub telegram: TelegramEnvironment,
    pub discord: DiscordEnvironment,
}

impl Environment {
    pub fn read() -> Result<Self> {
        Ok(Self {
            telegram: TelegramEnvironment {
                api_id: env::var("API_ID")?.parse::<u32>()?,
                api_hash: env::var("API_HASH")?,
                session_path: env::var("SESSION_PATH")?,
            },
            discord: DiscordEnvironment {
                auth_file: env::var("AUTH_FILE")?,
            },
        })
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[--- ENVIRONMENT ---]\n\n{}\n\n{}",
            self.telegram, self.discord
        )
    }
}
