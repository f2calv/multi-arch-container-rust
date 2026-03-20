use config::{Config, ConfigError};
use env_logger::Env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use sys_info::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Set INFO logging level as default
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    //Load app settings from env variables
    let app_settings = get_configuration().expect("configuration issue");

    // Set a watch on Ctrl-C, http://detegr.github.io/doc/ctrlc/
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    log::info!("Hit Ctrl-C to exit....");

    while running.load(Ordering::SeqCst) {
        log::info!(
            "App '{}' on [Process Architecture: {}, OSDescription: {} {}].",
            get_app_name(),
            get_arch(),
            os_type().unwrap(),
            os_release().unwrap(),
        );

        log::info!(
            "Git information; repository '{}', branch '{}', commit '{}', tag '{}'",
            DisplayOption(&app_settings.git_repository),
            DisplayOption(&app_settings.git_branch),
            DisplayOption(&app_settings.git_commit),
            DisplayOption(&app_settings.git_tag),
        );

        log::info!(
            "GitHub information; workflow '{}', run id '{}', run number '{}'",
            DisplayOption(&app_settings.github_workflow),
            DisplayOption(&app_settings.github_run_id),
            DisplayOption(&app_settings.github_run_number),
        );

        tokio::time::sleep(Duration::from_millis(3_000)).await;
    }

    Ok(())
}

fn get_configuration() -> Result<AppSettings, ConfigError> {
    let config = Config::builder()
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
    git_repository: Option<String>,
    git_branch: Option<String>,
    git_commit: Option<String>,
    git_tag: Option<String>,
    github_workflow: Option<String>,
    github_run_id: Option<u64>,
    github_run_number: Option<u32>,
}

impl std::fmt::Display for AppSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "git_repository='{}', git_branch='{}'",
            DisplayOption(&self.git_repository),
            DisplayOption(&self.git_branch),
        )
    }
}

struct DisplayOption<'a, T>(&'a Option<T>);

impl<T: std::fmt::Display> std::fmt::Display for DisplayOption<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(v) => v.fmt(f),
            None => f.write_str("N/A"),
        }
    }
}

fn get_arch() -> &'static str {
    std::env::consts::ARCH
}

fn get_app_name() -> String {
    std::env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned()
}
