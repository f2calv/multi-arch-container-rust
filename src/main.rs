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
            app_settings.GIT_REPO,
            app_settings.GIT_BRANCH,
            app_settings.GIT_COMMIT,
            app_settings.GIT_TAG,
        );

        log::info!(
            "CI/CD information; GitHub Workflow '{}', run id '{}', run number '{}'",
            app_settings.GITHUB_WORKFLOW,
            app_settings.GITHUB_RUN_ID,
            app_settings.GITHUB_RUN_NUMBER,
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
    GIT_REPO: String,
    GIT_BRANCH: String,
    GIT_COMMIT: String,
    GIT_TAG: String,
    GITHUB_WORKFLOW: String,
    GITHUB_RUN_ID: String,
    GITHUB_RUN_NUMBER: String,
}
