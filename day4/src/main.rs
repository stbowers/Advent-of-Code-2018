extern crate aoc_utils;
#[macro_use]
extern crate text_io;

use aoc_utils::file_utils;
use std::cmp::Ordering;

#[derive(PartialEq)]
enum LogAction {
    SHIFT_START,
    FALL_ASLEEP,
    WAKE_UP,
}

struct Log {
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    action: LogAction,
    guard_id: u32,
}

impl Log {
    pub fn new(
        year: u32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        action: LogAction,
        guard_id: u32,
    ) -> Log {
        return Log {
            year: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
            action: action,
            guard_id: guard_id,
        };
    }

    pub fn get_year(&self) -> u32 {
        return self.year;
    }

    pub fn get_month(&self) -> u8 {
        return self.month;
    }

    pub fn get_day(&self) -> u8 {
        return self.day;
    }

    pub fn get_hour(&self) -> u8 {
        return self.hour;
    }

    pub fn get_minute(&self) -> u8 {
        return self.minute;
    }

    pub fn get_action(&self) -> &LogAction {
        return &self.action;
    }

    pub fn get_guard_id(&self) -> u32 {
        return self.guard_id;
    }

    pub fn set_guard_id(&mut self, new_id: u32) {
        self.guard_id = new_id;
    }
}

fn main() {
    let mut buffer: String = String::new();
    let input: Vec<&str> =
        file_utils::get_input("./input.txt", &mut buffer, true).expect("Error reading input file");

    let mut logs: Vec<Log> = get_logs(&input);
    fix_guard_ids(&mut logs);
    let guards: Vec<u32> = get_guard_ids(&logs);

    let mut most_minutes_asleep: u32 = 0;
    let mut buest_guard: u32 = 0;

    let mut best_guard_2: u32 = 0;
    let mut minute_asleep: u8 = 0;
    let mut times_asleep: u32 = 0;

    for guard in guards {
        let minutes_asleep: u32 = get_minutes_asleep(&logs, guard);
        if minutes_asleep > most_minutes_asleep {
            buest_guard = guard;
            most_minutes_asleep = minutes_asleep;
        }

        for min in 0..60 {
            let times_asleep_at_minute = get_times_asleep_on_minute(&logs, guard, min);
            if times_asleep_at_minute > times_asleep {
                best_guard_2 = guard;
                minute_asleep = min;
                times_asleep = times_asleep_at_minute;
            }
        }
    }

    let minute_most_asleep = get_minute_most_asleep(&logs, buest_guard);
    println!(
        "Part 1: Guard #{} spend the most time asleep ({}) at 00:{} = {}",
        buest_guard,
        most_minutes_asleep,
        minute_most_asleep,
        buest_guard * minute_most_asleep as u32
    );

    println!(
        "Part 2: Guard #{} spend minute {} asleep more than any other guard or minute at {} times = {}",
        best_guard_2,
        minute_asleep,
        times_asleep,
        best_guard_2 * minute_asleep as u32
    );
}

fn get_times_asleep_on_minute(logs: &Vec<Log>, guard_id: u32, minute: u8) -> u32 {
    let mut times_asleep: u32 = 0;
    let mut last_sleep: u8 = 0;

    for log in logs.iter() {
        if log.get_guard_id() != guard_id {
            continue;
        }

        if *log.get_action() == LogAction::FALL_ASLEEP {
            if log.get_hour() == 23 {
                last_sleep = 0;
            } else {
                last_sleep = log.get_minute();
            }
        } else if *log.get_action() == LogAction::WAKE_UP {
            // If we slept through the minute, add to times_asleep
            if minute >= last_sleep && minute < log.get_minute() {
                times_asleep += 1;
            }
        }
    }

    return times_asleep;
}

fn fix_guard_ids(logs: &mut Vec<Log>) {
    let mut current_id: u32 = 0;
    for log in logs {
        if *log.get_action() == LogAction::SHIFT_START {
            current_id = log.get_guard_id();
        } else {
            log.set_guard_id(current_id);
        }
    }
}

fn get_minute_most_asleep(logs: &Vec<Log>, guard_id: u32) -> u8 {
    // (minute, times asleep)
    let mut minutes: Vec<u32> = Vec::new();
    minutes.resize(60, 0);

    let mut last_sleep: u8 = 0;

    for log in logs.iter() {
        if log.get_guard_id() != guard_id {
            continue;
        }

        if *log.get_action() == LogAction::FALL_ASLEEP {
            if log.get_hour() == 23 {
                last_sleep = 0;
            } else {
                last_sleep = log.get_minute();
            }
        } else if *log.get_action() == LogAction::WAKE_UP {
            for minute in last_sleep..log.get_minute() {
                minutes[minute as usize] += 1;
            }
        }
    }

    let mut best_minute: u8 = 0;
    let mut best_value: u32 = 0;
    for minute in 0..60 {
        if minutes[minute] > best_value {
            best_minute = minute as u8;
            best_value = minutes[minute] as u32;
        }
    }

    return best_minute;
}

fn get_minutes_asleep(logs: &Vec<Log>, guard_id: u32) -> u32 {
    let mut minutes_asleep: u32 = 0;
    let mut last_sleep: u8 = 0;

    for log in logs.iter() {
        if log.get_guard_id() != guard_id {
            continue;
        }

        if *log.get_action() == LogAction::FALL_ASLEEP {
            if log.get_hour() == 23 {
                last_sleep = 0;
            } else {
                last_sleep = log.get_minute();
            }
        } else if *log.get_action() == LogAction::WAKE_UP {
            minutes_asleep += (log.get_minute() - last_sleep) as u32;
        }
    }

    return minutes_asleep;
}

fn get_guard_ids(logs: &Vec<Log>) -> Vec<u32> {
    let mut guard_ids: Vec<u32> = logs.iter().map(|log| log.get_guard_id()).collect();
    guard_ids.sort_unstable();
    guard_ids.dedup();
    return guard_ids;
}

fn print_logs(logs: &Vec<Log>) {
    for log in logs.iter() {
        println!(
            "{}-{}-{} {}:{} - #{}",
            log.get_year(),
            log.get_month(),
            log.get_day(),
            log.get_hour(),
            log.get_minute(),
            log.get_guard_id()
        );
    }
}

fn get_logs(input: &Vec<&str>) -> Vec<Log> {
    let mut logs: Vec<Log> = Vec::new();
    let mut current_guard: u32 = 0;

    for &line in input.iter() {
        let (year, month, day, hour, minute, info_1, info_2): (
            u32,
            u8,
            u8,
            u8,
            u8,
            String,
            String,
        );
        scan!(line.bytes() => "[{}-{}-{} {}:{}] {} {}", year, month, day, hour, minute, info_1, info_2);

        let (new_guard, action) = parse_info(&info_1, &info_2, current_guard);
        current_guard = new_guard;

        let new_log: Log = Log::new(year, month, day, hour, minute, action, current_guard);

        // search by the time
        let search_result: Result<usize, usize> = logs.binary_search_by(|log| {
            if log.year > new_log.year {
                Ordering::Greater
            } else if log.year < new_log.year {
                Ordering::Less
            } else if log.month > new_log.month {
                Ordering::Greater
            } else if log.month < new_log.month {
                Ordering::Less
            } else if log.day > new_log.day {
                Ordering::Greater
            } else if log.day < new_log.day {
                Ordering::Less
            } else if log.hour > new_log.hour {
                Ordering::Greater
            } else if log.hour < new_log.hour {
                Ordering::Less
            } else if log.minute > new_log.minute {
                Ordering::Greater
            } else if log.minute < new_log.minute {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        if search_result.is_err() {
            logs.insert(search_result.unwrap_err(), new_log);
        } else {
            println!(
                "Duplicate timestamp: {}-{}-{} {}:{}",
                year, month, day, hour, minute
            );
        }
    }

    return logs;
}

fn parse_info(info_1: &str, info_2: &str, current_guard: u32) -> (u32, LogAction) {
    if info_1.contains("falls") {
        return (current_guard, LogAction::FALL_ASLEEP);
    } else if info_1.contains("wakes") {
        return (current_guard, LogAction::WAKE_UP);
    } else if info_1.contains("Guard") {
        let guard_id: u32;
        scan!(info_2.bytes() => "#{}", guard_id);
        return (guard_id, LogAction::SHIFT_START);
    } else {
        println!("Unknown action: {}", info_1);
        return (current_guard, LogAction::SHIFT_START);
    }
}
