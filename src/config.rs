//! Application configuration.
//!
//! Mirrors `Models/_AppConfig.cs` + `Models/_BuildInfo.cs` in the sibling .NET repository and
//! `config.go` in the sibling Go repository.

use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

/// Placeholder used when a build provenance variable is absent, i.e. outside a container.
pub const UNKNOWN: &str = "n/a";

/// Console log output format.
#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    /// Human-readable console output. Best for local development.
    #[default]
    Text,
    /// Newline-delimited JSON. Best for log shipping (Loki, Elasticsearch, OpenTelemetry).
    Json,
}

/// Application configuration, bound from the `app` section of `appsettings.json`.
///
/// Every value can be overridden by an environment variable using the same double-underscore
/// section separator as the sibling repositories, e.g. `APP__INTERVAL_SECONDS=10`.
///
/// Note: keys are snake_case because the `config` crate lower-cases environment keys but preserves
/// file keys verbatim - snake_case is the only casing where both sources resolve to the same key.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Message written on every iteration of the worker loop.
    pub greeting: String,

    /// Delay between worker loop iterations, in seconds.
    pub interval_seconds: u64,

    /// Console log output format.
    pub log_format: LogFormat,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            greeting: "Hello from a multi-architecture container".to_owned(),
            interval_seconds: 3,
            log_format: LogFormat::Text,
        }
    }
}

/// Root settings object: application configuration plus the flat build provenance variables
/// baked into the container image by the `ARG`/`ENV` block of the Dockerfile.
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    /// Application configuration.
    #[serde(default)]
    pub app: AppConfig,

    /// Git repository name, e.g. `f2calv/multi-arch-container-rust`.
    #[serde(default = "unknown")]
    pub git_repository: String,

    /// Git branch reference, e.g. `refs/heads/main`.
    #[serde(default = "unknown")]
    pub git_branch: String,

    /// Git commit SHA.
    #[serde(default = "unknown")]
    pub git_commit: String,

    /// Git tag, i.e. the semantic version of the image.
    #[serde(default = "unknown")]
    pub git_tag: String,

    /// GitHub Actions workflow name.
    #[serde(default = "unknown")]
    pub github_workflow: String,

    /// GitHub Actions run identifier.
    #[serde(default = "unknown")]
    pub github_run_id: String,

    /// GitHub Actions run number.
    #[serde(default = "unknown")]
    pub github_run_number: String,
}

fn unknown() -> String {
    UNKNOWN.to_owned()
}

/// Build the configuration from `appsettings.json` (optional) then the environment, with the
/// environment taking precedence - the same layering order as the sibling repositories.
///
/// Note: `try_parsing` is deliberately left OFF. It coerces every environment value that *looks*
/// numeric into an integer, which mangles values that only happen to be digits - an all-numeric
/// `GIT_COMMIT` SHA would be logged as `0`. Values stay as strings and serde converts them to the
/// declared field type on deserialisation instead.
pub fn load() -> Result<Settings, ConfigError> {
    Config::builder()
        .add_source(File::new("appsettings", FileFormat::Json).required(false))
        .add_source(Environment::default().separator("__"))
        .build()?
        .try_deserialize()
}
