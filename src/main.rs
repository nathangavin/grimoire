use home::home_dir;
use std::io::Error;

struct Config {}

fn main() {
    println!("Hello, world!");

    // begin config process
    let config = match load_config() {
        Ok(c) => c,
        Err(a) => {
            panic!("{:?}", a);
        }
    };

    // load data depending on config

    // analyse input args to determine what to do
}

fn load_config() -> Result<Config, Error> {
    let mut err_str = String::new();
    match home_dir() {
        Some(home) => {
            let config_path = home.join(".config").join("grimoire").join("config.toml");
            if config_path.exists() {
                // yes
                todo!();
                return Ok(Config {});
            } else {
                err_str = format!("Config file not found at {:?}", config_path);
            }
        }
        None => {
            err_str = format!("Could not determine home directory");
        }
    }

    return Err(Error::other(err_str));
}
