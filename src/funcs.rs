use std::{
    fs,
    net::{SocketAddr, TcpListener},
    path::Path,
};

use chrono::Local;

use crate::consts;

pub fn date_parse(ts_str: &str) -> i64 {
    if ts_str == "" {
        return Local::now().timestamp_millis();
    }
    
   let ts = chrono::DateTime::parse_from_str(ts_str, consts::DATE_FMT)
        .unwrap()
        .timestamp_millis();
    ts
}

pub fn parse_date(mut ts: i64) -> String {
    if ts == 0 {
        ts = chrono::Local::now().timestamp_millis();
    }
    let local_dt = chrono::Local::now();
    let offset = local_dt.offset();
    let dt = chrono::DateTime::from_timestamp_millis(ts)
        .unwrap()
        .with_timezone(offset);
    dt.format(consts::DATE_FMT).to_string()
}

pub fn write_json_file<T: serde::Serialize>(filename: &str, data: &T) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data).unwrap();
    write_file(filename, &json)
}

pub fn write_file(filename: &str, data: &str) -> Result<(), String> {
    let dir = Path::new(filename).parent().unwrap();
    fs::create_dir_all(dir).expect("Failed to create dirs");
    fs::write(filename, data).expect("Unable to write file");
    Ok(())
}

pub fn find_available_port(min_port: u16, max_port: u16) -> Option<u16> {
    for port in min_port..=max_port {
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        if TcpListener::bind(addr).is_ok() {
            return Some(port);
        }
    }
    None
}

pub fn calc_perc(b: f64, a: f64) -> f64 {
    let perc = (b - a) / a * 100.0;
    format!("{perc:.2}").parse().unwrap()
}

pub fn ts() -> String{
    Local::now().to_rfc3339().split(".").nth(0).unwrap().to_string()
}

pub fn now() -> i64{
    Local::now().timestamp_millis()
}

pub fn fix_precision(n: f64, precision: usize) -> f64 {
    let mut s = format!("{:.100}", n); // Use a high precision to avoid rounding errors
    let decimal_pos = s.find('.').unwrap_or(0);
    let end_pos = decimal_pos + precision + 1;
    if end_pos < s.len() {
        s = s[..end_pos].to_string();
    }

    s.parse().unwrap()
}
fn get_precision(s: String) -> usize {
    let decimal_pos = s.find('.').unwrap_or(0);
    let precision = s.len() - decimal_pos - 1;
    precision
}