use serde::{Serialize, Deserialize};
use std::collections::HashMap; // <--- 유저 이름을 Key로 사용
use std::fs;
use std::io; // <--- 에러 처리를 위해 io 모듈 사용

// 1. 개별 사용자 정보
// (보안) password_raw -> password_hash로 변경되었습니다.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub password_hash: String,
}

// 2. 전체 데이터베이스의 구조 (HashMap)
pub type UserDatabase = HashMap<String, User>;

// 3. TOML 파일 경로 (상수)
const DB_PATH: &str = "users.toml"; // 프로젝트 루트에 users.toml로 저장


// 4. 데이터베이스 파일 불러오기 (Load)
pub fn load_db() -> io::Result<UserDatabase> {
    // 1. users.toml 파일을 읽으려 시도합니다.
    match fs::read_to_string(DB_PATH) {
        Ok(content) => {
            // 2. 파일이 있으면, TOML 내용을 파싱해서 UserDatabase(HashMap)로 변환
            toml::from_str(&content)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            // 3. (중요!) 파일이 없으면 (예: 첫 실행),
            //    에러가 아니라 "빈 HashMap"을 반환합니다.
            Ok(UserDatabase::new())
        }
        Err(e) => {
            // 4. 그 외의 에러 (예: 권한 없음)는 그대로 반환
            Err(e)
        }
    }
}

// 5. 데이터베이스 파일 저장하기 (Save)
pub fn save_db(db: &UserDatabase) -> io::Result<()> {
    // 1. UserDatabase(HashMap)를 TOML 문자열로 자동 변환
    let content = toml::to_string(db)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    
    // 2. 변환된 문자열을 users.toml 파일에 쓰기
    fs::write(DB_PATH, content)
}
