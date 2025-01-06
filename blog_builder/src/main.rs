use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct Report {
    text: String,
    link: String,
}

fn main() {
    let path = Path::new("data/en/daily-report/list.json");

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create directories");
        }
    };

    let report = Report {
        text: "Daily Report".to_string(),
        link: "This is a sample daily report json file".to_string(),
    };

    let json_data = serde_json::to_string_pretty(&report).expect("Failed to serialize data");

    fs::write(path, json_data).expect("Failed to write data to file");

    println!("JSON file created at {:?}", path);
}
