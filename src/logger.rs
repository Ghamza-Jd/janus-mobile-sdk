pub fn raw_jarust_init_logger() {
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
            Err(why) => log::error!("{why}"),
        };
    }

    log::info!("Jarust started logging");
    log_panics::init();
}
