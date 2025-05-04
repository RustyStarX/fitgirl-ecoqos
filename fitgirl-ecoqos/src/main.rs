use ahash::AHashSet;
use fitgirl_ecoqos::{
    Error,
    config::Config,
    listen::{Process, listen_process_creation},
};
use tracing::{error, info, level_filters::LevelFilter, warn};
use tracing_subscriber::EnvFilter;
use win32_ecoqos::process::toggle_efficiency_mode;

#[cfg(feature = "regex")]
use regex::RegexSet;

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
    assert!(os_version >= 21359, "EcoQoS not supported!");

    if os_version < 22621 {
        warn!("you may not get the best result before Windows 11 22H2!");
    }

    let config = Config::from_default_path()?;
    info!("startup with config: {config:?}");
    let Config {
        blacklist,
        #[cfg(feature = "regex")]
        blacklist_regex,
    } = config;
    let blacklist: AHashSet<String> = blacklist.into_iter().collect();
    #[cfg(feature = "regex")]
    let regexset = RegexSet::new(blacklist_regex)?;

    listen_process_creation(move |Process { process_id, name }| {
        if !blacklist.contains(&name) {
            return;
        }

        #[cfg(feature = "regex")]
        if !regexset.is_match(&name) {
            return;
        }

        info!("found process: {process_id}, name: {name}, throtting...");
        if let Err(e) = toggle_efficiency_mode(process_id, Some(true)) {
            error!("failed to throttle {process_id}: {e}");
        }
    })
    .await?;

    Ok(())
}
