use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
use home::home_dir;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

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
        let file_path = format!("{0}/{1}/WEEK{2}", year, m, weekNo);
        let actual_file_path = format!("{0}/{1}.md", file_path, filename);

        let home = home_dir().unwrap().to_str().unwrap().to_string();

        match daily_notes_config.dir {
            Some(d) => {
                println!("config dir is: {:?}", d);
                let true_path = d[..].replace("~", &home[..]);
                println!("{}", true_path);
                fs::create_dir_all(format!("{0}/{1}", true_path, file_path)).unwrap();
                fs::File::create(format!("{0}/{1}", true_path, actual_file_path)).unwrap();
                //let mut file = fs::File::create(format!())?;
            }
            None => {
                println!("config dir not found");
            }
        }
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

    fn last_work_day(today: &NaiveDate) -> NaiveDate {
        let days_to_subtract = match today.weekday() {
            Weekday::Mon => 3,
            Weekday::Tue => 1,
            Weekday::Wed => 1,
            Weekday::Thu => 1,
            Weekday::Fri => 1,
            Weekday::Sat => 2,
            Weekday::Sun => 3,
        };
        return today - Duration::days(days_to_subtract);
    }
}
