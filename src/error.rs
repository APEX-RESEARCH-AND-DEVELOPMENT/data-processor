use std::fmt::Display;

use color_eyre::owo_colors::OwoColorize;

#[derive(Debug)]
pub struct DecodingError {
    pub filepath: String,
}

impl DecodingError {
    pub fn new(filepath: String) -> Self {
        Self { filepath: filepath }
    }
}

impl Display for DecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            "[DECODING_ERROR]".black().on_red(),
            format!("Error trying to decode file: {}", self.filepath).red()
        )
    }
}

#[derive(Debug)]
pub struct DeserializationError {
    pub filepath: String,
    pub error_object: String,
}

impl DeserializationError {
    pub fn new(filepath: String, error_object: String) -> Self {
        Self {
            filepath: filepath,
            error_object: error_object,
        }
    }
}

impl Display for DeserializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            "[DESERIALIZATION_ERROR]".black().on_red(),
            format!(
                "Error trying to deserialize JSON file '{}': {}",
                self.filepath, self.error_object
            )
            .red()
        )
    }
}

#[derive(Debug)]
pub struct UsernameResolveError {
    pub username: String,
    pub message: String,
}

impl UsernameResolveError {
    pub fn new(username: String, message: String) -> Self {
        Self {
            username: username,
            message: message,
        }
    }
}

impl Display for UsernameResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            "[USERNAME_RESOLVE_ERROR]".black().on_red(),
            format!("{}: {}", self.message, self.username).red()
        )
    }
}

#[derive(Debug)]
pub struct FileExtensionError {
    pub expected: String,
    pub found: String,
}

impl FileExtensionError {
    pub fn new(expected: String, found: String) -> Self {
        Self {
            expected: expected,
            found: found,
        }
    }
}

impl Display for FileExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            "[FILE_EXTENSION_ERROR]".black().on_red(),
            format!(
                "Mismatched filetype, expected '{}', got '{}'",
                self.expected, self.found
            )
            .red()
        )
    }
}

#[derive(Debug)]
pub struct FileNotFoundError {
    pub expected_path: String,
}

impl FileNotFoundError {
    pub fn new(expected_path: String) -> Self {
        Self {
            expected_path: expected_path,
        }
    }
}

impl Display for FileNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            "[FILE_NOT_FOUND_ERROR]".black().on_red(),
            format!("File expected at path: {}", self.expected_path).red()
        )
    }
}

#[derive(Debug)]
pub struct DateTimeParseError {
    pub datestring: String,
    pub error_object: String,
}

impl DateTimeParseError {
    pub fn new(datestring: String, error_object: String) -> Self {
        Self {
            datestring: datestring,
            error_object: error_object,
        }
    }
}

impl Display for DateTimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            "[DATETIME_PARSE_ERROR]".black().on_red(),
            format!(
                "Unable to parse datetime string '{}': {}",
                self.datestring, self.error_object
            )
            .red()
        )
    }
}

#[derive(Debug)]
pub struct DiscordClientInitializationError {
    pub message: String,
}

impl DiscordClientInitializationError {
    pub fn new(message: String) -> Self {
        Self { message: message }
    }
}

impl Display for DiscordClientInitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            "[DISCORD_CLIENT_INITIALIZATION_ERROR]".black().on_red(),
            format!("Unable to init discord client: {}", self.message).red()
        )
    }
}

#[derive(Debug)]
pub struct InvalidIDError {
    pub ids: Vec<String>,
}

impl InvalidIDError {
    pub fn new(ids: Vec<String>) -> Self {
        Self { ids: ids }
    }
}

impl Display for InvalidIDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{}",
            "[INVALID_ID_ERROR]".black().on_red(),
            "Invalid IDs were supplied:\n[\n".red()
        )?;

        for id in self.ids.iter() {
            write!(f, "   {}\n", id.red())?;
        }

        write!(f, "{}", "]".red())
    }
}
