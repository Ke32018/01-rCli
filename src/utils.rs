use anyhow::Result;
use chrono::{Duration as ChronoDuration, Utc};
use std::{fs::File, io::Read, time::Duration};

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn get_content(input: &str) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn parse_duration(s: &str) -> Result<Duration, &'static str> {
    let s = s.trim();
    let now = Utc::now();
    let duration = if let Some(value) = s.strip_suffix('m') {
        // Minutes
        let minutes: i64 = value.parse().map_err(|_| "invalid number of minutes")?;
        ChronoDuration::minutes(minutes)
    } else if let Some(value) = s.strip_suffix('h') {
        // Hours
        let hours: i64 = value.parse().map_err(|_| "invalid number of hours")?;
        ChronoDuration::hours(hours)
    } else if let Some(value) = s.strip_suffix('d') {
        // Days
        let days: i64 = value.parse().map_err(|_| "invalid number of days")?;
        ChronoDuration::days(days)
    } else {
        return Err("invalid duration format, must end with `m`, `h`, or `d`");
    };
    let expiration = now + duration;
    let exp_duration = Duration::from_secs(expiration.timestamp() as u64);
    Ok(exp_duration)
}
