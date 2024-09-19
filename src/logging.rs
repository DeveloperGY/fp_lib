use crate::time::UTCTime;

pub struct Logger;

impl Logger {
    pub fn info(mut dest: impl std::io::Write, thread_name: &str, msg: &str) {
        let time = std::time::SystemTime::now();
        let hour = time.get_current_hour_24();
        let minute = time.get_current_minute();
        let second = time.get_current_second();

        writeln!(
            dest,
            "[{:0>2}:{:0>2}:{:0>2}] \x1b[34m[{}/INFO]: {}\x1b[0m",
            hour, minute, second, thread_name, msg
        )
        .unwrap();

        dest.flush().unwrap();
    }

    pub fn warn(mut dest: impl std::io::Write, thread_name: &str, msg: &str) {
        let time = std::time::SystemTime::now();
        let hour = time.get_current_hour_24();
        let minute = time.get_current_minute();
        let second = time.get_current_second();

        writeln!(
            dest,
            "[{:0>2}:{:0>2}:{:0>2}] \x1b[33m[{}/WARN]: {}\x1b[0m",
            hour, minute, second, thread_name, msg
        )
        .unwrap();

        dest.flush().unwrap();
    }

    pub fn err(mut dest: impl std::io::Write, thread_name: &str, msg: &str) {
        let time = std::time::SystemTime::now();
        let hour = time.get_current_hour_24();
        let minute = time.get_current_minute();
        let second = time.get_current_second();

        writeln!(
            dest,
            "[{:0>2}:{:0>2}:{:0>2}] \x1b[31m[{}/ERR]: {}\x1b[0m",
            hour, minute, second, thread_name, msg
        )
        .unwrap();

        dest.flush().unwrap();
    }
}
