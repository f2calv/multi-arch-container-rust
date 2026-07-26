//! Structured logging setup.
//!
//! `tracing` + `tracing-subscriber` is the Rust analogue of Serilog in the sibling .NET repository
//! and `log/slog` in the sibling Go repository.

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

use crate::config::{AppConfig, LogFormat};

/// Install the global `tracing` subscriber.
///
/// Verbosity is controlled by the conventional `RUST_LOG` environment variable
/// (e.g. `RUST_LOG=debug`, `RUST_LOG=multi_arch_container_rust=trace`) and defaults to `info`.
pub fn init(app: &AppConfig) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let registry = tracing_subscriber::registry().with(filter);

    match app.log_format {
        LogFormat::Json => registry
            .with(fmt::layer().json().flatten_event(true))
            .init(),
        LogFormat::Text => registry.with(fmt::layer().compact()).init(),
    }
}
