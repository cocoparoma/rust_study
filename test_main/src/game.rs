// src/game.rs

use serde::{Serialize, Deserialize};
// 다른 파일에서 쓸 수 있도록 `pub` 키워드를 붙입니다.
pub const MAP_WIDTH: u16 = 16;
pub const MAP_HEIGHT: u16 = 16;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub x: u16,
    pub y: u16,
}

impl Player {
    // 플레이어 생성자
    pub fn new() -> Self {
        Player { x: 8, y: 8 }
    }

    // 플레이어 이동 로직 (이제 Player의 메서드가 됩니다)
    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.y < MAP_HEIGHT - 1 {
            self.y += 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < MAP_WIDTH - 1 {
            self.x += 1;
        }
    }
}
