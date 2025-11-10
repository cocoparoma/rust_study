// src/main.rs

// (1) 모듈 선언: Rust 컴파일러에게 이 파일들을 사용하라고 알림
mod game;
mod file_io;

use std::io::{self, stdout, Write};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self, ClearType},
    cursor,
    style,
    queue,
};

// (2) use 선언: 모듈 안의 특정 항목들을 가져옴
use game::{Player, MAP_WIDTH, MAP_HEIGHT};
use file_io::{setup_logging, log_key_press, save_coordinates, load_coordinates};


// --- (신규) 렌더링 로직을 별도 함수로 분리 ---
// 화면, 플레이어 상태를 받아서 그리기만 담당
fn draw_map(stdout: &mut io::Stdout, player: &Player, status_message: &String) -> io::Result<()> {
    queue!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            if x == player.x && y == player.y {
                queue!(stdout, style::Print("@"))?;
            } else {
                queue!(stdout, style::Print("."))?;
            }
        }
        queue!(stdout, style::Print("\r\n"))?;
    }
    
    // 상태 메시지 출력
    queue!(stdout, style::Print(format!("Status: {}\r\n", status_message)))?;

    // 한 번에 flush
    stdout.flush()
}


// --- 메인 함수 ---
fn main() -> io::Result<()> {
    // --- 1. 설정 ---
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;

    // (수정) file_io 모듈의 함수 호출
    let mut log_file = file_io::setup_logging()?;

    // (수정) game 모듈의 생성자 호출
    let mut player = Player::new();
    
    let help_message = "(wasd: 이동, t: 저장, y: 불러오기, q: 종료)";
    let mut status_message = String::from(help_message);

    // --- 2. 메인 루프 ---
    loop {
        // (수정) 렌더링 함수 호출
        draw_map(&mut stdout, &player, &status_message)?;

        // --- 3. 입력 받기 ---
        if let Event::Key(key_event) = event::read()? {
            // (수정) 상태 메시지 초기화
            status_message = String::from(help_message); 
            
            // (수정) 로그 함수 호출
            if let Err(e) = file_io::log_key_press(&mut log_file, &key_event) {
                status_message = format!("로그 쓰기 실패: {}", e);
            }

            // --- 4. 상태 업데이트 ---
            match key_event.code {
                // (수정) game 모듈의 메서드 호출
                KeyCode::Char('w') => { player.move_up(); status_message = "Moved Up".into(); }
                KeyCode::Char('a') => { player.move_left(); status_message = "Moved Left".into(); }
                KeyCode::Char('s') => { player.move_down(); status_message = "Moved Down".into(); }
                KeyCode::Char('d') => { player.move_right(); status_message = "Moved Right".into(); }
                
                // (수정) file_io 모듈의 함수 호출
                KeyCode::Char('t') => {
                    status_message = file_io::save_coordinates(&player);
                }
                
                // (수정) file_io 모듈의 함수 호출
                KeyCode::Char('y') => {
                    match file_io::load_coordinates() {
                        Ok(loaded_player) => {
                            player = loaded_player; // 불러온 Player로 교체
                            status_message = format!("불러오기 완료! ({}, {})", player.x, player.y);
                        }
                        Err(e_msg) => {
                            status_message = e_msg; // 실패 메시지 표시
                        }
                    }
                }

                KeyCode::Char('q') => {
                    break;
                }
                _ => { /* 무시 */ }
            }
        }
    }

    // --- 5. 정리 ---
    terminal::disable_raw_mode()?;
    Ok(())
}
