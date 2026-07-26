//! Background worker.
//!
//! Mirrors `Services/WorkerService.cs` in the sibling .NET repository and `worker.go` in the
//! sibling Go repository: it periodically logs runtime, configuration and build provenance
//! information, and shuts down cleanly on SIGINT/SIGTERM.

use std::time::Duration;

use tokio::sync::watch::Receiver;
use tracing::info;

use crate::config::Settings;

/// Run the worker loop until `shutdown` is signalled.
pub async fn run(settings: &Settings, mut shutdown: Receiver<bool>) {
    let app = &settings.app;

    info!(
        greeting = %app.greeting,
        interval_seconds = app.interval_seconds,
        log_format = ?app.log_format,
        "worker started"
    );

    let interval = Duration::from_secs(app.interval_seconds);

    loop {
        info!(
            app_name = %app_name(),
            process_architecture = std::env::consts::ARCH,
            os_type = %sys_info::os_type().unwrap_or_else(|_| "unknown".to_owned()),
            os_release = %sys_info::os_release().unwrap_or_else(|_| "unknown".to_owned()),
            "{}",
            app.greeting
        );

        info!(
            git_repository = %settings.git_repository,
            git_branch = %settings.git_branch,
            git_commit = %settings.git_commit,
            git_tag = %settings.git_tag,
            "git provenance"
        );

        info!(
            github_workflow = %settings.github_workflow,
            github_run_id = %settings.github_run_id,
            github_run_number = %settings.github_run_number,
            "github provenance"
        );

        tokio::select! {
            _ = tokio::time::sleep(interval) => {}
            _ = shutdown.changed() => break,
        }
    }

    info!("worker stopping");
}

/// File name of the running executable.
fn app_name() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .unwrap_or_else(|| crate::config::UNKNOWN.to_owned())
}
