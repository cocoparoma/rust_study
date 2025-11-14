use serde::Deserialize;
use std::{default, fs};
use std::path::Path;
use std::io::ErrorKind;

#[derive(Debug, Deserialize)]
pub struct Config{
    pub game_title: String,
    pub number_of_decks: usize,
    pub user_db_path: String,
}

pub fn load_config() ->Result<Config, Box<dyn std::error::Error>> {
    let config_path = Path::new("config.toml");
    let config_string_result = fs::read_to_string(config_path);

    match config_string_result {
        Ok(config_string) => {
            let config: Config = toml::from_str(&config_string)?;
            Ok(config)
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound{
                println!("[ERR] NotFound::Can't find config.toml.");
                println!("[ERR] NotFound::Make it again.");
                fs::create_dir_all("data");
                let default_content = 
r#"
game_title = "My Baccarat Game"
number_of_decks = 8
user_db_path = "data/users.toml"
"#;
                fs::write(config_path, default_content)?;
                let config: Config = toml::from_str(default_content)?;
                Ok(config)  
            } else {
                Err(e.into())
            }
        }
    }

    
    
}