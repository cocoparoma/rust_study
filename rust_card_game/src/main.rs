mod card;
mod config;
mod file_io;
mod ui;
mod player;
mod user_db;

use config::load_config;

fn main() {
    println!("build Test");
    
    //config Test
    match load_config(){
        Ok(config) =>{
            println!("{:#?}", config);
        }
        Err(e) =>{
            println!("Can't load Config");
            println!("[Err] : {}", e);
        }
    
    }
    
}
