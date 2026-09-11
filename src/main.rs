//! Multi-architecture container demonstrator.
//!
//! A trivial worker process, implemented identically in four languages:
//!   - <https://github.com/f2calv/multi-arch-container-dotnet>
//!   - <https://github.com/f2calv/multi-arch-container-go>
//!   - <https://github.com/f2calv/multi-arch-container-rust> (this one)
//!   - <https://github.com/f2calv/multi-arch-container-python>

mod config;
mod telemetry;
mod worker;

use std::error::Error;

use tokio::sync::watch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // 1) Configuration: base JSON -> environment-specific JSON -> environment variables.
    let settings = config::load()?;

    // 2) Structured logging. Application code only ever calls the `tracing` macros, so the
    //    subscriber (text vs JSON, filtering, exporters) can be swapped without touching it.
    let telemetry = telemetry::init(&settings)?;

    // 3) Shutdown signalling. `ctrlc` traps SIGINT and, with the `termination` feature, the
    //    SIGTERM that `docker stop` and `kubectl delete pod` send. The handler runs on its own
    //    thread, so it publishes to a watch channel that the async worker can select on.
    let (tx, rx) = watch::channel(false);
    ctrlc::set_handler(move || {
        let _ = tx.send(true);
    })?;

    tracing::info!("Hit Ctrl-C to exit....");

    // 4) The worker itself.
    worker::run(&settings, rx).await;

    if let Some(telemetry) = telemetry {
        tokio::task::spawn_blocking(move || telemetry.shutdown()).await??;
    }

    Ok(())
}
