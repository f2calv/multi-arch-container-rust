use env_logger::Env;

fn main() {
    // Set INFO logging level as default
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("application started up: {}", 123);
}
