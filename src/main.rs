use home::home_dir;
use std::{env, fs, io};

mod config;
mod daily_notes;

use config::Config;

#[derive(Debug)]
enum RunningMode {
    Config,
    DailyNotes,
    Other,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let running_mode = handle_arg(args.get(0));
    dbg!(&running_mode);

    match running_mode {
        RunningMode::Config => {}
        RunningMode::DailyNotes => {}
        _ => {}
    }
    // begin config process
    let config = Config::load()?;

    println!("{:?}", config);
    println!("{}", toml::to_string(&config).unwrap());

    // load data depending on config

    // analyse input args to determine what to do

    Ok(())
}

fn handle_arg(arg: Option<&String>) -> RunningMode {
    dbg!(arg);
    match arg {
        Some(val) => {
            return match &val[..] {
                "config" => RunningMode::Config,
                "dailyNotes" => RunningMode::DailyNotes,
                _ => RunningMode::Other,
            }
        }
        None => {}
    }
    return RunningMode::Other;
}
