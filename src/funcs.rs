use std::{fs, net::{SocketAddr, TcpListener}, path::Path};

use crate::consts;

pub fn date_parse(ts_str: &str) -> i64 {
    chrono::DateTime::parse_from_str(ts_str, consts::DATE_FMT)
    .unwrap()
    .timestamp_millis()
}

pub fn parse_date(mut ts: i64) -> String{
    if ts == 0{
        ts = chrono::Local::now().timestamp_millis();
    }
    let local_dt = chrono::Local::now();
    let offset = local_dt.offset();
    let dt = chrono::DateTime::from_timestamp_millis(ts).unwrap().with_timezone(offset);
    dt.format(consts::DATE_FMT).to_string()
}

pub fn write_json_file<T: serde::Serialize>(filename: &str, data: &T) -> Result<(), String> {
    let json = serde_json::to_string_pretty(data).unwrap();
    write_file(filename, &json)

}

pub fn write_file(filename: &str, data: &str) -> Result<(), String>{
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

pub fn calc_perc(b: f64, a: f64) -> f64{
    let perc = (b - a) / a * 100.0;
    format!("{perc:.2}").parse().unwrap()
}

