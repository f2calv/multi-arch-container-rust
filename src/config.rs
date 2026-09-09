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
///
/// `#[serde(default)]` is applied at container level so that an `appsettings.json` supplying only
/// *some* of the keys still deserialises, with the remainder falling back to [`AppConfig::default`]
/// - matching the per-property defaults of the sibling .NET and Go repositories.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
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

impl AppConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if self.greeting.trim().is_empty() {
            return Err(ConfigError::Message(
                "app.greeting must not be empty".to_owned(),
            ));
        }
        if !(1..=3600).contains(&self.interval_seconds) {
            return Err(ConfigError::Message(
                "app.interval_seconds must be between 1 and 3600".to_owned(),
            ));
        }

        Ok(())
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
/// The sibling .NET repository layers one extra source, an optional
/// `appsettings.{DOTNET_ENVIRONMENT}.json`, because `Host.CreateApplicationBuilder` provides it
/// for free. It is deliberately not reimplemented here.
///
/// Note: `try_parsing` is deliberately left OFF. It coerces every environment value that *looks*
/// numeric into an integer, which mangles values that only happen to be digits - an all-numeric
/// `GIT_COMMIT` SHA would be logged as `0`. Values stay as strings and serde converts them to the
/// declared field type on deserialisation instead.
pub fn load() -> Result<Settings, ConfigError> {
    let settings: Settings = Config::builder()
        .add_source(File::new("appsettings", FileFormat::Json).required(false))
        .add_source(Environment::default().separator("__"))
        .build()?
        .try_deserialize()?;
    settings.app.validate()?;

    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mirrors `TestDefaultSettingsArePopulated` in the sibling Go repository.
    #[test]
    fn defaults_are_populated() {
        let app = AppConfig::default();

        assert_eq!(app.interval_seconds, 3);
        assert_eq!(app.log_format, LogFormat::Text);
        assert!(!app.greeting.is_empty());
    }

    /// An `appsettings.json` supplying only some keys must still deserialise, with the remainder
    /// falling back to the defaults - the behaviour of the sibling .NET and Go repositories.
    #[test]
    fn partial_configuration_falls_back_to_defaults() {
        let settings: Settings = Config::builder()
            .add_source(File::from_str(
                r#"{ "app": { "greeting": "hello from a test" } }"#,
                FileFormat::Json,
            ))
            .build()
            .expect("configuration should build")
            .try_deserialize()
            .expect("configuration should deserialise");

        assert_eq!(settings.app.greeting, "hello from a test");
        assert_eq!(settings.app.interval_seconds, 3);
        assert_eq!(settings.app.log_format, LogFormat::Text);
        assert_eq!(settings.git_tag, UNKNOWN);
    }

    /// `log_format` is matched case-insensitively against the lower-cased enum variants.
    #[test]
    fn log_format_deserialises_from_json_value() {
        let settings: Settings = Config::builder()
            .add_source(File::from_str(
                r#"{ "app": { "log_format": "json" } }"#,
                FileFormat::Json,
            ))
            .build()
            .expect("configuration should build")
            .try_deserialize()
            .expect("configuration should deserialise");

        assert_eq!(settings.app.log_format, LogFormat::Json);
    }

    #[test]
    fn validation_rejects_invalid_app_settings() {
        let invalid_configs = [
            AppConfig {
                greeting: " ".to_owned(),
                ..AppConfig::default()
            },
            AppConfig {
                interval_seconds: 0,
                ..AppConfig::default()
            },
            AppConfig {
                interval_seconds: 3601,
                ..AppConfig::default()
            },
        ];

        for app in invalid_configs {
            assert!(app.validate().is_err(), "configuration should be invalid");
        }
    }
}
