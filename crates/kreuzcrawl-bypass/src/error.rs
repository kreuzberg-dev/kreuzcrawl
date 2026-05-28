//! Error types for the kreuzcrawl-bypass crate.

use thiserror::Error;

/// Errors that can occur during config loading or validation.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// The config file could not be read from disk.
    #[error("config read failed: {0}")]
    Io(#[from] std::io::Error),

    /// The YAML content could not be parsed.
    #[error("config parse failed: {0}")]
    Parse(String),

    /// A required field is absent in the YAML document.
    #[error("config missing required field: {0}")]
    MissingField(String),

    /// A field value is not of the expected type.
    #[error("config field '{field}' has unexpected type: expected {expected}")]
    WrongType { field: String, expected: String },

    /// A field value is not a recognized option.
    #[error("config field '{field}' has unknown value '{value}'")]
    UnknownValue { field: String, value: String },

    /// An environment variable referenced via `${{VAR_NAME}}` is not set.
    #[error("env var '{0}' not set (referenced in config)")]
    MissingEnvVar(String),
}

/// Errors that can occur at runtime inside `SimpleHttpProvider::fetch`.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// The HTTP client could not be built.
    #[error("http client build failed: {0}")]
    ClientBuild(String),

    /// The HTTP request could not be sent.
    #[error("request send failed for vendor '{vendor}': {message}")]
    Send { vendor: String, message: String },

    /// The response body could not be read.
    #[error("response body read failed for vendor '{vendor}': {message}")]
    BodyRead { vendor: String, message: String },

    /// The response JSON could not be parsed or the expected field was missing.
    #[error("response parse failed for vendor '{vendor}': {message}")]
    ResponseParse { vendor: String, message: String },
}
