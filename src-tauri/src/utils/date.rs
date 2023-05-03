use chrono::{ DateTime, Local };

pub fn get_date (offset: i32) -> String {
  // 获取当前时间
  let now: DateTime<Local> = Local::now();
  now.format("%Y-%m-%d").to_string()
}
