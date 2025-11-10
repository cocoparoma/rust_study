// 1. 모듈 선언
mod user_db;
use user_db::{load_db, save_db, User, UserDatabase};

// 2. crossterm 사용
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{self, ClearType},
};
use std::io::{self, stdout, Write};

// 3. bcrypt 임포트
use bcrypt;

// --- get_user_input 함수 (사용자 입력 처리) ---
fn get_user_input(
    stdout: &mut io::Stdout,
    prompt: &str,
    masked: bool,
) -> io::Result<String> {
    let mut input = String::new();
    execute!(stdout, Print(prompt))?;
    stdout.flush()?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Enter => {
                    execute!(stdout, Print("\r\n"))?;
                    break;
                }
                KeyCode::Backspace => {
                    if !input.is_empty() {
                        input.pop();
                        execute!(
                            stdout,
                            cursor::MoveLeft(1),
                            Print(" "),
                            cursor::MoveLeft(1)
                        )?;
                        stdout.flush()?;
                    }
                }
                KeyCode::Char(c) => {
                    input.push(c);
                    if masked {
                        execute!(stdout, Print("*"))?;
                    } else {
                        execute!(stdout, Print(c))?;
                    }
                    stdout.flush()?;
                }
                _ => {}
            }
        }
    }
    Ok(input)
}

// --- press_enter_to_continue 함수 (메시지 후 대기) ---
fn press_enter_to_continue(stdout: &mut io::Stdout, message: &str) -> io::Result<()> {
    execute!(
        stdout,
        Print(format!("\r\n{} (계속하려면 Enter를 누르세요...)\r\n", message))
    )?;
    stdout.flush()?;
    loop {
        if let Event::Key(key_event) = event::read()? {
            if key_event.code == KeyCode::Enter {
                break;
            }
        }
    }
    Ok(())
}

// --- handle_signup 함수 (회원가입 로직) ---
fn handle_signup(stdout: &mut io::Stdout) -> io::Result<()> {
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    execute!(stdout, Print("--- 회원가입 ---\r\n"))?;
    
    let mut db = load_db()?;

    let username = get_user_input(stdout, "사용할 아이디: ", false)?;
    if username.is_empty() {
        press_enter_to_continue(stdout, "아이디는 비워둘 수 없습니다.")?;
        return Ok(());
    }

    if db.contains_key(&username) {
        press_enter_to_continue(stdout, "이미 사용 중인 아이디입니다.")?;
        return Ok(());
    }

    let password = get_user_input(stdout, "비밀번호: ", true)?;
    if password.is_empty() {
        press_enter_to_continue(stdout, "비밀번호는 비워둘 수 없습니다.")?;
        return Ok(());
    }
    let password_confirm = get_user_input(stdout, "비밀번호 확인: ", true)?;

    if password != password_confirm {
        press_enter_to_continue(stdout, "비밀번호가 일치하지 않습니다.")?;
        return Ok(());
    }

    // (보안) 비밀번호 해싱
    let hashed_password = match bcrypt::hash(&password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        Err(e) => {
            press_enter_to_continue(stdout, &format!("오류 발생: {}", e))?;
            return Ok(());
        }
    };

    // (보안) 해시 값을 저장
    let new_user = User { 
        password_hash: hashed_password 
    };
    db.insert(username.clone(), new_user);
    save_db(&db)?; 

    press_enter_to_continue(stdout, &format!("'{}'님, 회원가입이 완료되었습니다!", username))?;
    Ok(())
}

// --- handle_login 함수 (로그인 로직) ---
fn handle_login(stdout: &mut io::Stdout) -> io::Result<()> {
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    execute!(stdout, Print("--- 로그인 ---\r\n"))?;

    let db = load_db()?;

    let username = get_user_input(stdout, "아이디: ", false)?;
    let password = get_user_input(stdout, "비밀번호: ", true)?;

    match db.get(&username) {
        Some(user) => {
            // (보안) 비밀번호 검증
            let valid = match bcrypt::verify(&password, &user.password_hash) {
                Ok(v) => v,
                Err(_) => false, 
            };

            if valid {
                press_enter_to_continue(stdout, &format!("'{}'님, 환영합니다!", username))?;
            } else {
                press_enter_to_continue(stdout, "비밀번호가 틀렸습니다.")?;
            }
        }
        None => {
            press_enter_to_continue(stdout, "존재하지 않는 아이디입니다.")?;
        }
    }
    Ok(())
}


// --- main 함수 (메인 메뉴 루프) ---
fn main() -> io::Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?; 

    loop {
        execute!(
            stdout,
            terminal::Clear(ClearType::All), 
            cursor::MoveTo(0, 0),            
            Print("--- Rust 로그인 시스템 (보안 강화) ---\r\n\r\n"),
            Print("1. 로그인\r\n"),
            Print("2. 회원가입\r\n"),
            Print("3. 종료\r\n\r\n"),
            Print("선택: ")
        )?;
        stdout.flush()?; 

        let choice = get_user_input(&mut stdout, "", false)?;

        match choice.trim() {
            "1" => {
                handle_login(&mut stdout)?;
            }
            "2" => {
                handle_signup(&mut stdout)?;
            }
            "3" | "q" | "Q" => {
                break; 
            }
            _ => {
                press_enter_to_continue(&mut stdout, "잘못된 선택입니다. (1, 2, 3 중 하나)")?;
            }
        }
    }

    // 프로그램 종료 전 정리
    terminal::disable_raw_mode()?; 
    execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?; 
    Ok(())
}
