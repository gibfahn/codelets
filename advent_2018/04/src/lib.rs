#![feature(external_doc)]
#![doc(include = "../Question.md")]

use std::collections::HashMap;
use std::str::FromStr;

// use chrono::prelude::*;
use chrono::{offset::TimeZone, DateTime, Duration, Timelike, Utc};
use failure::{bail, format_err, Error};
use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../input");

pub fn answer() -> (String, String) {
    let sleep_data = Schedule::from(INPUT).unwrap().sleep_data();
    (
        sleep_data.most_asleep_by_guard().to_string(),
        sleep_data.most_asleep_by_minute().to_string(),
    )
}

#[derive(Debug, PartialEq)]
struct Schedule {
    events: Vec<Event>,
}

#[derive(Debug, PartialEq)]
struct Event {
    id: Option<u32>,
    action: Action,
    date_time: DateTime<Utc>,
}

#[derive(Debug, PartialEq)]
enum Action {
    BeginShift,
    Wake,
    Sleep,
}

#[derive(Debug)]
struct SleepData {
    sleep_count: HashMap<u32, i64>,
    sleep_minutes: HashMap<u32, HashMap<u32, u32>>,
}

impl Schedule {
    fn from(s: &str) -> Result<Self, Error> {
        let mut events = s.lines().collect::<Vec<_>>();
        events.sort();
        let (results, errors): (Vec<_>, Vec<_>) = events
            .iter()
            .map(|l| l.parse::<Event>())
            .partition(Result::is_ok);
        if !errors.is_empty() {
            bail!(
                "{:#?}",
                errors
                    .into_iter()
                    .map(Result::unwrap_err)
                    .collect::<Vec<_>>()
            );
        }
        let mut events: Vec<_> = results.into_iter().map(Result::unwrap).collect();

        let mut last_id = None;
        for mut event in events.iter_mut() {
            if event.id.is_some() {
                last_id = event.id;
            } else {
                event.id = last_id;
            }
        }
        Ok(Schedule { events })
    }

    fn sleep_data(&self) -> SleepData {
        let mut sleep_count = HashMap::new();
        let mut sleep_minutes = HashMap::new();
        let mut sleep_date = None;
        for event in &self.events {
            match event.action {
                Action::Sleep => {
                    sleep_date = Some(event.date_time);
                }
                Action::Wake => {
                    let sleep_length = event
                        .date_time
                        .signed_duration_since(sleep_date.unwrap())
                        .num_minutes();
                    *sleep_count.entry(event.id.unwrap()).or_insert(0) += sleep_length;
                    let guard_sleep_minutes = sleep_minutes
                        .entry(event.id.unwrap())
                        .or_insert_with(HashMap::new);
                    let mut date = sleep_date.unwrap();
                    while date != event.date_time {
                        *guard_sleep_minutes.entry(date.minute()).or_insert(0) += 1;
                        date = date.checked_add_signed(Duration::minutes(1)).unwrap();
                    }
                    sleep_date = None;
                }
                _ => {}
            }
        }
        SleepData {
            sleep_count,
            sleep_minutes,
        }
    }
}

impl SleepData {
    fn most_asleep_by_guard(&self) -> u32 {
        let most_asleep_guard = &self.sleep_count.iter().max_by_key(|x| x.1).unwrap().0;
        let most_asleep_minute = &self.sleep_minutes[most_asleep_guard]
            .iter()
            .max_by_key(|x| x.1)
            .unwrap()
            .0;

        **most_asleep_guard * **most_asleep_minute
    }
    fn most_asleep_by_minute(&self) -> u32 {
        let mut max_id = None;
        let mut max_minute = None;
        let mut max_count = 0;
        for (id, guard_map) in self.sleep_minutes.iter() {
            let (minute, count) = guard_map.iter().max_by_key(|x| x.1).unwrap();
            if *count > max_count {
                max_count = *count;
                max_id = Some(id);
                max_minute = Some(minute);
            }
        }
        max_id.unwrap() * max_minute.unwrap()
    }
}

impl FromStr for Event {
    type Err = Error;

    // [1518-11-01 00:00] Guard #10 begins shift
    // [1518-11-01 00:05] falls asleep
    // [1518-11-01 00:25] wakes up
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref re: Regex = Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\](?: Guard #)?(\d+)?( begins shift| falls asleep| wakes up)$").unwrap();
        }

        let matches = re
            .captures(s)
            .ok_or_else(|| format_err!("Failed to parse into an Event: {:?}", s))?;
        let matches_len = matches.len();

        if matches_len != 7 && matches_len != 8 {
            bail!(
                "Wrong number of matches found when parsing Event:\ninput: {:?}, matches: {:?}",
                s,
                matches
            );
        }

        let match_id = matches.get(6);
        let id = if match_id.is_some() {
            Some(match_id.unwrap().as_str().parse::<u32>()?)
        } else {
            None
        };
        let action = matches[7].parse::<Action>()?;

        let date_time = Utc
            .ymd(
                matches[1].parse::<i32>()?,
                matches[2].parse::<u32>()?,
                matches[3].parse::<u32>()?,
            )
            .and_hms(matches[4].parse::<u32>()?, matches[5].parse::<u32>()?, 0);

        Ok(Event {
            id,
            action,
            date_time,
        })
    }
}

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            " begins shift" => Ok(Action::BeginShift),
            " falls asleep" => Ok(Action::Sleep),
            " wakes up" => Ok(Action::Wake),
            _ => Err(format_err!("Unknown action type {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = "[1518-11-01 00:25] wakes up\n\
                     [1518-11-04 00:46] wakes up\n\
                     [1518-11-01 00:30] falls asleep\n\
                     [1518-11-01 00:55] wakes up\n\
                     [1518-11-01 23:58] Guard #99 begins shift\n\
                     [1518-11-02 00:40] falls asleep\n\
                     [1518-11-05 00:45] falls asleep\n\
                     [1518-11-02 00:50] wakes up\n\
                     [1518-11-01 00:05] falls asleep\n\
                     [1518-11-03 00:05] Guard #10 begins shift\n\
                     [1518-11-03 00:29] wakes up\n\
                     [1518-11-04 00:02] Guard #99 begins shift\n\
                     [1518-11-04 00:36] falls asleep\n\
                     [1518-11-05 00:03] Guard #99 begins shift\n\
                     [1518-11-03 00:24] falls asleep\n\
                     [1518-11-05 00:55] wakes up\n\
                     [1518-11-01 00:00] Guard #10 begins shift\n";

        assert_eq!(
            Schedule::from(input)
                .unwrap()
                .sleep_data()
                .most_asleep_by_guard(),
            240
        );
        assert_eq!(
            Schedule::from(input)
                .unwrap()
                .sleep_data()
                .most_asleep_by_minute(),
            4455
        );
    }

    #[test]
    fn test_answer() {
        assert_eq!(answer(), (String::from("8421"), String::from("83359")));
    }
}
