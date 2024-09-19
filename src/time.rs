/// Pull this trait into scope and youll be able to use its methods on [std::time::SystemTime]
pub trait UTCTime {
    fn get_current_hour(&self) -> u64;
    fn get_current_hour_24(&self) -> u64;
    fn get_current_minute(&self) -> u64;
    fn get_current_second(&self) -> u64;
}

impl UTCTime for std::time::SystemTime {
    /// Panics if the call to [std::time::SystemTime::duration_since()] fails
    fn get_current_hour(&self) -> u64 {
        let duration_from_epoch = self.duration_since(std::time::UNIX_EPOCH).unwrap();

        let seconds_since_epoch = duration_from_epoch.as_secs();
        let hours_since_epoch = seconds_since_epoch / 3600;

        hours_since_epoch % 12
    }

    /// Panics if the call to [std::time::SystemTime::duration_since()] fails
    fn get_current_hour_24(&self) -> u64 {
        let duration_from_epoch = self.duration_since(std::time::UNIX_EPOCH).unwrap();

        let seconds_since_epoch = duration_from_epoch.as_secs();
        let hours_since_epoch = seconds_since_epoch / 3600;

        hours_since_epoch % 24
    }

    /// Panics if the call to [std::time::SystemTime::duration_since()] fails
    fn get_current_minute(&self) -> u64 {
        let duration_from_epoch = self.duration_since(std::time::UNIX_EPOCH).unwrap();

        let seconds_since_epoch = duration_from_epoch.as_secs();
        let minutes_since_epoch = seconds_since_epoch / 60;

        minutes_since_epoch % 60
    }

    /// Panics if the call to [std::time::SystemTime::duration_since()] fails
    fn get_current_second(&self) -> u64 {
        let duration_from_epoch = self.duration_since(std::time::UNIX_EPOCH).unwrap();

        let seconds_since_epoch = duration_from_epoch.as_secs();

        seconds_since_epoch % 60
    }
}
