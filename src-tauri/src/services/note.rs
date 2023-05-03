
use std::fs::{ metadata, create_dir_all };
use std::fs::OpenOptions;
use std::io::prelude::*;
use serde::{ Serialize, Deserialize };
use serde_json;
use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
struct MemoItem {
  name: String,
  complete: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct MemoData {
  date: String,
  list: Vec<MemoItem>,
}

#[tauri::command]
pub fn get_memo_by_day (offset: i32, app_handle: tauri::AppHandle) -> String {
  let date = utils::date::get_date(offset);

  let file_name = format!("{}{}", date, ".json");

  let mut file_path = app_handle.path_resolver().app_dir().unwrap();

  file_path.push("data");

  if !file_path.exists() {
    create_dir_all(&file_path).unwrap();
  }

  file_path.push(&file_name);

  let mut file = match OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(&file_path) {
    Ok(file) => file,
    Err(_e) => panic!("get_memo_by_day failed to open file"),
  };

  let file_length = metadata(&file_path).unwrap().len();

  if file_length == 0 {
    let memo_data = MemoData {
      date: String::from(&date),
      list: Vec::new(),
    };

    return serde_json::to_string(&memo_data).unwrap();
  }

  let mut memo_contents: String = String::new();
  file.read_to_string(&mut memo_contents).expect("get_memo_by_day failed to write file");

  memo_contents
}

#[tauri::command]
pub fn set_memo (date: &str, content: &str, app_handle: tauri::AppHandle) -> bool {

  let file_name = format!("{}{}", date, ".json");

  let mut file_path = app_handle.path_resolver().app_dir().unwrap();

  file_path.push("data");

  if !file_path.exists() {
    create_dir_all(&file_path).unwrap();
  }

  file_path.push(&file_name);

  let mut file = match OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .truncate(true)
    .open(&file_path) {
    Ok(file) => file,
    Err(_e) => panic!("set_memo failed to open file"),
  };

  file.write_all(content.as_bytes()).expect("set_memo failed to write file");
  true
}
