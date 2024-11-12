use home::home_dir;
use serde_derive::{Deserialize, Serialize};
use std::{fs, io};

use crate::daily_notes::DailyNotes;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub general: Option<General>,
    pub daily_notes: Option<DailyNotes>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct General {
    pub editor_command: Option<String>,
}

impl Config {
    pub fn handle(&self, args: Vec<String>) -> io::Result<()> {
        // load the file and prompt the user to populate required fields
        println!("starting config setting section");
        println!("Creating config file");
        match home_dir() {
            Some(home) => {
                let dir_path = home.join(".config").join("grimoire");
                let file_path = dir_path.join("config.toml");
                fs::create_dir_all(dir_path)?;
                fs::File::create(file_path)?;
            }
            None => {}
        }

        return Ok(());
    }

    pub fn load() -> Result<Config, io::Error> {
        let err_str;
        match home_dir() {
            Some(home) => {
                let config_path = home.join(".config").join("grimoire").join("config.toml");
                if config_path.exists() {
                    // yes
                    println!("config file found");
                    let contents = fs::read_to_string(config_path)?;

                    let config: Config = match toml::from_str(&contents) {
                        Ok(c) => c,
                        Err(e) => {
                            return Err(io::Error::other(e.to_string()));
                        }
                    };
                    return Ok(config);
                } else {
                    err_str = format!("Config file not found at {:?}", config_path);
                }
            }
            None => {
                err_str = format!("Could not determine home directory");
            }
        }

        return Err(io::Error::other(err_str));
    }

    pub fn save(&self) {
        /*
         * convert config into string via toml to_string
         * save to defined config file
         */
    }
}
