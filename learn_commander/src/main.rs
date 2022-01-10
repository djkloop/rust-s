
use std::{env, fs};

use chrono::Utc;
use serde_derive::{Deserialize, Serialize};
use sysinfo::{System, SystemExt};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    city: String,
    server: String,
    time: String,
    info: String,
}

fn get_all_info(seq: &str) -> String {
    // 获取所有的system info
    let sys = System::new_all();
    // 建立一个vec数组
    let mut items = vec![];
    // host name
    let host_name = sys.host_name().unwrap();
    // memory
    let memory = format!("memory: {}/{}", sys.used_memory(), sys.total_memory());
    let os_version = sys.os_version().unwrap();
    let cpu_number = num_cpus::get_physical().to_string();
    let process_number = sys.processors().len();
    // other_info
    let other_info = format!("os: {}, cpu: {}, process: {}", os_version, cpu_number, process_number);
    for item in [host_name, memory, other_info] {
        items.push(item.to_string())
    }
    items.join(seq)
}

fn main() {
    // 城市名称
    let input_city_name = env::args().nth(1).unwrap();
    let input_server_name = env::args().nth(2).unwrap();
    let output = env::args().nth(3).unwrap();
    let now = Utc::now().to_string();
    let out_json = Config {
        city: input_city_name,
        server: input_server_name,
        time: now,
        info: get_all_info(" && "),
    };

    // 写入文件
    fs::write(output, serde_json::to_string_pretty(&out_json).unwrap()).unwrap();
}
