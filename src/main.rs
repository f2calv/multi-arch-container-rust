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

    log::debug!("application started...");

    // Set a watch on Ctrl-C, http://detegr.github.io/doc/ctrlc/
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    //Load app settings from env variables
    let app_settings = get_configuration().expect("configuration issue");

    log::info!("Hit Ctrl-C to exit....");
    while running.load(Ordering::SeqCst) {
        // let appName = "AppDomain.CurrentDomain.FriendlyName"; //TODO
        // let ProcessArchitecture = "RuntimeInformation.ProcessArchitecture"; //TODO
        // let OSArchitecture = "RuntimeInformation.OSArchitecture"; //TODO
        // let OSDescription = "RuntimeInformation.OSDescription"; //TODO
        // log::info!(
        //     "App '{}' on [Process Architecture: {}, OSArchitecture: {}, OSDescription: {}].",
        //     appName,
        //     ProcessArchitecture,
        //     OSArchitecture,
        //     OSDescription
        // );

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

    print_sysinfo();

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

impl std:fmt::Display for AppSettings {
    fn fmt(&self, f: &mut std:fmt::Formatter<'_>) -> std:fmt::Result {
        write!(f, "({}, {})", self.git_repo, self.y)
    }
}

fn print_sysinfo() {
    log::debug!("os: {} {}", os_type().unwrap(), os_release().unwrap());
    log::debug!(
        "cpu: {} cores, {} MHz",
        cpu_num().unwrap(),
        cpu_speed().unwrap()
    );
    log::debug!("proc total: {}", proc_total().unwrap());
    let load = loadavg().unwrap();
    log::debug!("load: {} {} {}", load.one, load.five, load.fifteen);
    let mem = mem_info().unwrap();
    log::debug!(
        "mem: total {} KB, free {} KB, avail {} KB, buffers {} KB, cached {} KB",
        mem.total,
        mem.free,
        mem.avail,
        mem.buffers,
        mem.cached
    );
    log::debug!(
        "swap: total {} KB, free {} KB",
        mem.swap_total,
        mem.swap_free
    );
    #[cfg(not(target_os = "solaris"))]
    {
        let disk = disk_info().unwrap();
        log::debug!("disk: total {} KB, free {} KB", disk.total, disk.free);
    }
    log::debug!("hostname: {}", hostname().unwrap());
    #[cfg(not(target_os = "windows"))]
    {
        let t = boottime().unwrap();
        log::debug!("boottime {} sec, {} usec", t.tv_sec, t.tv_usec);
    }
    #[cfg(target_os = "linux")]
    log::debug!("/etc/os-release: {:?}", linux_os_release().unwrap());
}
