use std::{
    io::{self, Write},
    time::{SystemTime, UNIX_EPOCH},
};

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn fmt_time(time: f32) -> String {
    let hours = time as u32;
    let minutes = ((time - hours as f32) * 60.0) as u32;
    let seconds = (((time - hours as f32) * 60.0 - minutes as f32) * 60.0) as u32;
    String::from(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
}

pub fn input() -> String {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}
