pub mod log {
    use std::fmt;
    use std::time::SystemTime;

    static mut PREFIX: &'static str = "default";

    pub fn set_prefix(prefix: &'static str) {
        unsafe {
            PREFIX = prefix;
        }
    }

    pub fn info(fmt: fmt::Arguments) {
        log("info", fmt);
    }

    pub fn error(fmt: fmt::Arguments) {
        log("error", fmt);
    }

    pub fn warning(fmt: fmt::Arguments) {
        log("warning", fmt);
    }

    pub fn debug(fmt: fmt::Arguments) {
        log("debug", fmt);
    }

    fn log(level: &str, fmt: fmt::Arguments) {
        let now = SystemTime::now();
        let time = now.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        unsafe {
            println!(
                "{} [{}-{}] {}",
                chrono::NaiveDateTime::from_timestamp(time as i64, 0),
                PREFIX,
                level,
                fmt
            );
        }
    }
}
