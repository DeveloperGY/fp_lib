use std::fmt::Display;

use crate::time::UTCTime;

pub enum Log {
    Info,
    Warning,
    Error,
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match *self {
            Log::Info => "INFO",
            Log::Warning => "WARN",
            Log::Error => "ERR",
        };

        write!(f, "{}", val)
    }
}

pub struct Logger;

impl Logger {
    pub fn log(dest: &mut impl std::io::Write, log_type: Log, thread_name: &str, msg: &str) {
        let time = std::time::SystemTime::now();
        let hour = time.get_current_hour_24();
        let minute = time.get_current_minute();
        let second = time.get_current_second();

        write!(
            dest,
            "[{}:{}:{}] [{}/{}]: {}",
            hour, minute, second, thread_name, log_type, msg
        )
        .unwrap();
    }
}
