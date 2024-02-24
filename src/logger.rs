use tracing_subscriber::EnvFilter;

pub fn raw_jarust_init_logger() {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("jarust=trace".parse().unwrap()),
        )
        .compact()
        .init();

    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag("Jarust"),
        );
    }

    #[cfg(target_os = "ios")]
    {
        let logger = oslog::OsLogger::new("com.jarust")
            .level_filter(log::LevelFilter::Trace)
            .init();
        match logger {
            Ok(()) => {}
            Err(why) => tracing::error!("{why}"),
        };
    }

    tracing::info!("Jarust started logging");
    log_panics::init();
}
