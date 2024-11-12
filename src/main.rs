use home::home_dir;
use serde_derive::Deserialize;
use std::{env, fs, io};

#[derive(Deserialize, Debug)]
struct Config {
    general: Option<General>,
    dailyNotes: Option<DailyNotes>,
}

#[derive(Deserialize, Debug)]
struct General {
    editor_command: Option<String>,
}

#[derive(Deserialize, Debug)]
struct DailyNotes {
    daily_notes_dir: Option<String>,
}

#[derive(Debug)]
enum RunningMode {
    Config,
    Other,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let running_mode = handle_args(args);
    dbg!(&running_mode);

    match running_mode {
        RunningMode::Config => {
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
        _ => {}
    }
    // begin config process
    let config = load_config()?;

    println!("{:?}", config);

    // load data depending on config

    // analyse input args to determine what to do

    Ok(())
}

fn handle_args(args: Vec<String>) -> RunningMode {
    dbg!(&args);
    match args.get(1) {
        Some(val) => {
            if val == "config" {
                return RunningMode::Config;
            }
        }
        None => {}
    }
    return RunningMode::Other;
}

fn load_config() -> Result<Config, io::Error> {
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
