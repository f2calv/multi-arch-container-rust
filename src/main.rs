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
            "Git information; name '{:?}', branch '{:?}', commit '{:?}', tag '{:?}'",
            app_settings.git_repo,
            app_settings.git_branch,
            app_settings.git_commit,
            app_settings.git_tag,
        );

        log::info!(
            "GitHub information; workflow '{:?}', run id '{:?}', run number '{:?}'",
            app_settings.github_workflow,
            app_settings.github_run_id,
            app_settings.github_run_number,
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
    git_repo: Option<String>,
    git_branch: Option<String>,
    git_commit: Option<String>,
    git_tag: Option<String>,
    github_workflow: Option<String>,
    github_run_id: Option<u32>,
    github_run_number: Option<u32>,
}

impl std::fmt::Display for AppSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "git_repo='{:?}', git_branch='{:?}'",
            self.git_repo, self.git_branch
        )
    }
}

fn get_arch() -> String {
    #[cfg(target_arch = "x86")]
    let arch = String::from("x86");
    #[cfg(target_arch = "x86_64")]
    let arch = String::from("x86_64");
    #[cfg(target_arch = "arm")]
    let arch = String::from("arm");
    #[cfg(target_arch = "aarch64")]
    let arch = String::from("aarch64");
    arch
}

fn get_app_name() -> String {
    std::env::current_exe()
        .expect("Can't get the exec path")
        .file_name()
        .expect("Can't get the exec name")
        .to_string_lossy()
        .into_owned()
}
