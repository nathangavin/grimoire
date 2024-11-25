use std::{env, io};

mod config;
mod daily_notes;

use config::Config;
use daily_notes::DailyNotes;

#[derive(Debug)]
enum RunningMode {
    Config,
    DailyNotes,
    Other,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let running_mode = handle_arg(args.get(1));
    dbg!(&running_mode);

    match running_mode {
        RunningMode::Config => {}
        RunningMode::DailyNotes => {
            let dnc = DailyNotes {
                dir: Some("~/hub/notes/06-Daily/".to_string()),
            };
            DailyNotes::handle(dnc, args[1..].to_vec());
        }
        _ => {}
    }
    // begin config process
    return Ok(());
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
