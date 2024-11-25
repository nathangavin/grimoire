use chrono::{Datelike, Local, Timelike};
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
    pub fn handle(daily_notes_config: DailyNotes, args: Vec<String>) {
        println!("handling daily notes arg");
        let arg = Self::handle_arg(args.get(0));
        println!("detected arg: {:?}", arg);

        match daily_notes_config.dir {
            Some(d) => {
                println!("config dir is: {:?}", d);
            }
            None => {
                println!("config dir not found");
            }
        }

        let now = Local::now();

        let year = now.year();
        let month = now.month();
        let day_of_month = now.day();
        let weekday = now.weekday();

        let iso_week = now.iso_week();
        let iso_week_start = now.with_day(1).unwrap().iso_week();

        println!("{}", year);
        println!("{}", month);
        println!("{}", day_of_month);
        println!("{}", weekday);
        let weekNo = iso_week.week() - iso_week_start.week() + 1;
        let m = chrono::Month::try_from(u8::try_from(month).unwrap())
            .unwrap()
            .name();

        let filename = format!("{0}{1}{2}-{3}", year - 2000, month, day_of_month, weekday);
        println!("{0}/{1}/WEEK{2}/{3}", year, m, weekNo, filename);
        // calculate week in month
        //let first_day_of_month = now.with_day(1).unwrap().weekday();
        //let days_since_month_start = day_of_month - 1;
    }

    fn handle_arg(arg: Option<&String>) -> Option<DailyNotesMode> {
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
