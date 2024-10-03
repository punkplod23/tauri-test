use chrono::{DateTime, Utc};
use csv::StringRecord;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use tauri_plugin_fs::FsExt;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
      .setup(|app| {
          // allowed the given directory
          let scope = app.fs_scope();
          scope.allow_directory("C:/github", false);
          dbg!(scope.allowed());

          Ok(())
       })
        .invoke_handler(tauri::generate_handler![greet, run_csv])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn run_csv(filepath: &str) -> String {
    if let Err(_e) = parse_csv(filepath) {}
    format!("Import completes <a href='file://C:/github/tauri-test/my-file.txt' target='_blank'>Download</a>")
}

fn parse_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    println!("{}", file_path);

    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("../../my-file.txt")
        .unwrap();

        if let Err(e) = write!(file, "[") {
            println!("Couldn't write to file: {}", e);
        }        
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;
    
        println!("UTC now is: {}", now);
    let mut ittr = 0;
    let mut header = StringRecord::new();



    if let Some(result) = rdr.records().next() {
        header = result?;
    }

    for result in rdr.records() {

        println!("Result {}",ittr);

        let json = create_json_record_map(header.clone(), result?, ittr).to_string();
        if let Err(e) = write!(file, "{}", json) {
            println!("Couldn't write to file: {}", e);
        }

        ittr = ittr + 1;
    }

    
    if let Err(e) = write!(file, "]") {
        println!("Couldn't write to file: {}", e);
    }

    let now: DateTime<Utc> = Utc::now();

    println!("End:: {}", now);
    Ok(())
}

fn create_json_record_map(
    mut _keys: StringRecord,
    mut values: StringRecord,
    record_no: i32,
) -> String {
    let record_length = values.len();
    let mut json = String::new();
    if record_no > 0 {
        json += ",";
    }
    json += "{";
    for n in 0..record_length {
        if n > 0 {
            json += ",";
        }
        json += "'";
        json += _keys
            .get(n)
            .as_ref()
            .map(|x| &**x)
            .unwrap_or("default string");
        json += "'";
        json += ":";
        json += "'";
        json += values
            .get(n)
            .as_ref()
            .map(|x| &**x)
            .unwrap_or("default string");
        json += "'";
    }
    values.clear();
    json += "}";
    return json;
}
