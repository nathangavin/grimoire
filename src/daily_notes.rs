use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct DailyNotes {
    pub dir: Option<String>,
}

#[derive(Clone, Debug)]
enum DailyNotesMode {
    Yesterday,
    Tomorrow,
    NextWeek,
    LastWeek,
    Today,
}

impl DailyNotes {
    pub fn handle(&self, daily_notes_config: DailyNotes, args: Vec<String>) {
        println!("handling daily notes arg");
        let arg = self.handle_arg(args.get(0));
        println!("detected arg: {:?}", arg);

        match daily_notes_config.dir {
            Some(d) => {
                println!("config dir is: {:?}", d);
            }
            None => {
                println!("config dir not found");
            }
        }
    }

    fn handle_arg(&self, arg: Option<&String>) -> Option<DailyNotesMode> {
        let args_map = HashMap::from([
            ("-y", DailyNotesMode::Yesterday),
            ("-tw", DailyNotesMode::Tomorrow),
            ("-nw", DailyNotesMode::NextWeek),
            ("-lw", DailyNotesMode::LastWeek),
            ("-t", DailyNotesMode::Today),
            ("--yesterday", DailyNotesMode::Yesterday),
            ("--tomorrow", DailyNotesMode::Tomorrow),
            ("--nextWeek", DailyNotesMode::NextWeek),
            ("--lastWeek", DailyNotesMode::LastWeek),
            ("--today", DailyNotesMode::Today),
            ("yesterday", DailyNotesMode::Yesterday),
            ("tomorrow", DailyNotesMode::Tomorrow),
            ("nextWeek", DailyNotesMode::NextWeek),
            ("lastWeek", DailyNotesMode::LastWeek),
            ("today", DailyNotesMode::Today),
        ]);

        match arg {
            Some(a) => {
                match args_map.get(&a[..]) {
                    Some(e) => return Some(e.clone()),
                    None => return None,
                };
            }
            None => {
                return Some(DailyNotesMode::Today);
            }
        };
    }
}
