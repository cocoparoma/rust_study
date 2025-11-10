// src/file_io.rs

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use chrono::prelude::*;
use crate::game::{Player, MAP_WIDTH, MAP_HEIGHT};
use crossterm::event::KeyEvent;
use toml;

// --- 1. 로그 설정 ---
// ↓↓↓ 'pub'가 있는지 확인하세요
pub fn setup_logging() -> io::Result<File> {
    // ... (내용 동일)
    let log_dir = "my_folder/log";
    fs::create_dir_all(log_dir)?;
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    let log_file_path = format!("{}/log_{}.toml", log_dir, timestamp);
    let mut log_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file_path)?;
    writeln!(log_file, "# Log file created at {}\n", now.to_rfc3339())?;
    Ok(log_file)
}

// --- 2. 로그 쓰기 ---
// ↓↓↓ 'pub'가 있는지 확인하세요
pub fn log_key_press(log_file: &mut File, key_event: &KeyEvent) -> io::Result<()> {
    // ... (내용 동일)
    let log_time = Local::now();
    let key_str = match key_event.code {
        crossterm::event::KeyCode::Char(c) => c.to_string(),
        _ => format!("{:?}", key_event.code),
    };
    let log_entry = format!(
        "[[log_event]]\ntimestamp = \"{}\"\nkey_pressed = \"{}\"\n\n",
        log_time.to_rfc3339(),
        key_str
    );
    log_file.write_all(log_entry.as_bytes())
}

// --- 3. 저장 (Save) ---
// ↓↓↓ 'pub'가 있는지 확인하세요
pub fn save_coordinates(player: &Player) -> String {
    // ... (내용 동일, Serde 버전)
    let dir_name = "my_folder";
    let file_path = "my_folder/test.toml";
    let content = match toml::to_string(player) {
        Ok(toml_string) => toml_string,
        Err(e) => return format!("TOML 직렬화 실패: {}", e),
    };
    match fs::create_dir_all(dir_name) {
        Ok(_) => match fs::write(file_path, content) {
            Ok(_) => format!("좌표 저장 완료: ({}, {})", player.x, player.y),
            Err(e) => format!("파일 쓰기 실패: {}", e),
        },
        Err(e) => format!("폴더 생성 실패: {}", e),
    }
}

// --- 4. 불러오기 (Load) ---
// ↓↓↓ 'pub'가 있는지 확인하세요
pub fn load_coordinates() -> Result<Player, String> {
    // ... (내용 동일, Serde 버전)
    let file_path = "my_folder/test.toml";
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return Err("저장된 파일이 없습니다. (t로 저장)".to_string()),
    };
    let player: Player = match toml::from_str(&content) {
        Ok(p) => p,
        Err(e) => return Err(format!("파일 형식 오류 (TOML 파싱 실패): {}", e)),
    };
    if player.x < MAP_WIDTH && player.y < MAP_HEIGHT {
        Ok(player)
    } else {
        Err("저장된 좌표가 맵을 벗어납니다.".to_string())
    }
}
