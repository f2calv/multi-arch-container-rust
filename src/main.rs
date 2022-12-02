#![allow(dead_code, non_snake_case)]
use config::{Config, ConfigError, File};
use env_logger::Env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Set INFO logging level as default
    //env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    //env_logger::init();

    // Set a watch on Ctrl-C, http://detegr.github.io/doc/ctrlc/
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let app_settings = get_configuration().expect("configuration issue");

    log::info!("Hit Ctrl-C to exit...");
    while running.load(Ordering::SeqCst) {
        let appName = "AppDomain.CurrentDomain.FriendlyName"; //TODO
        let ProcessArchitecture = "RuntimeInformation.ProcessArchitecture"; //TODO
        let OSArchitecture = "RuntimeInformation.OSArchitecture"; //TODO
        let OSDescription = "RuntimeInformation.OSDescription"; //TODO
        log::info!(
            "App '{}' on [Process Architecture: {}, OSArchitecture: {}, OSDescription: {}].",
            appName,
            ProcessArchitecture,
            OSArchitecture,
            OSDescription
        );

        log::info!(
            "Repository information; name '{}', branch '{}', commit '{}', tag '{}'",
            app_settings.git_repo,
            app_settings.git_branch,
            app_settings.git_commit,
            app_settings.git_tag,
        );

        log::info!(
            "CI/CD information; GitHub Workflow '{}', run id '{}', run number '{}'",
            app_settings.github_workflow,
            app_settings.github_run_id,
            app_settings.github_run_number,
        );

        tokio::time::sleep(Duration::from_millis(2_000)).await;
    }

    Ok(())
}

fn get_configuration() -> Result<AppSettings, ConfigError> {
    let config = Config::builder()
        .add_source(File::with_name("appsettings.toml"))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;
    config.try_deserialize()
}

#[derive(Debug, serde::Deserialize)]
struct AppSettings {
    git_repo: String,
    git_branch: String,
    git_commit: String,
    git_tag: String,
    github_workflow: String,
    github_run_id: u32,
    github_run_number: u32,
}
