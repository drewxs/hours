use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn fmt(time: f32) -> String {
    let hours = time as u32;
    let minutes = ((time - hours as f32) * 60.0) as u32;
    let seconds = (((time - hours as f32) * 60.0 - minutes as f32) * 60.0) as u32;
    String::from(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
}
