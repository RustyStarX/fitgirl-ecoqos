use fitgirl_ecoqos::{Error, config::Config, listen::listen_process_creation};
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let os_version = windows_version::OsVersion::current().build;
    assert!(os_version >= 22000, "EcoQoS not supported!");

    if os_version < 22621 {
        warn!("you may not get the best result before Windows 11 22H2!");
    }

    let config = Config::from_default_path()?;
    info!("startup with config: {config:?}");

    listen_process_creation().await?;

    Ok(())
}
