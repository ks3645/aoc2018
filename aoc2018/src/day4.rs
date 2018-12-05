use std::cmp::Ordering;
use std::collections::HashMap;
use utils;
use utils::Part;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 4).unwrap();

    do_the_thing(input, part)
}

fn do_the_thing(input: String, part: Part) -> i32 {
    let mut guard_map = HashMap::new();

    let mut records: Vec<Record> = input.lines().map(|line| Record::new(line)).collect();

    records.sort();

    let guard_indices: Vec<usize> = records
        .iter()
        .enumerate()
        .filter_map(|(i, rec)| {
            if let LineType::Guard(_) = rec.line_type {
                Some(i)
            } else {
                None
            }
        }).collect();

    for i in 0..guard_indices.len() - 1 as usize {
        let index_start = guard_indices[i];
        let index_end = guard_indices[i + 1];

        let guard = Guard::from(&records[index_start]);
        let mut guard_entry = guard_map.entry(guard.id).or_insert(guard);

        let mut ptr = index_start + 1;
        while ptr + 1 < index_end {
            guard_entry.apply_nap(&records[ptr].time_stamp, &records[ptr + 1].time_stamp);

            ptr += 2;
        }
    }

    let chosen_guard = match part {
        Part::One => guard_map
            .values()
            .max_by_key(|guard| guard.schedule.iter().sum::<i32>())
            .unwrap(),
        Part::Two => guard_map
            .values()
            .max_by_key(|guard| guard.schedule.iter().max())
            .unwrap(),
    };

    let mut max = -1;
    let mut best_minute = 0;
    for (i, val) in chosen_guard.schedule.iter().enumerate() {
        if *val > max {
            best_minute = i;
            max = *val;
        }
    }

    chosen_guard.id * best_minute as i32
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum LineType {
    Guard(i32),
    Asleep,
    Wake,
    None,
}

impl LineType {
    fn from(input: &str) -> LineType {
        let mut result = LineType::None;
        if input.contains("Guard") {
            let cleaned = input.replace("Guard #", "");
            let tokens: Vec<&str> = cleaned.split_whitespace().collect();
            result = LineType::Guard(tokens[0].parse().unwrap());
        } else if input.contains("falls") {
            result = LineType::Asleep;
        } else if input.contains("wakes") {
            result = LineType::Wake;
        }
        result
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Record {
    time_stamp: TimeStamp,
    line_type: LineType,
}

impl Record {
    fn new(input: &str) -> Record {
        let tokens: Vec<&str> = input.split("]").collect();

        Record {
            time_stamp: TimeStamp::from(tokens[0]),
            line_type: LineType::from(tokens[1]),
        }
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Record) -> Ordering {
        match self.time_stamp.month.cmp(&other.time_stamp.month) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match self.time_stamp.date.cmp(&other.time_stamp.date) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => match self.time_stamp.hour.cmp(&other.time_stamp.hour) {
                    Ordering::Less => Ordering::Less,
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => match self.time_stamp.minute.cmp(&other.time_stamp.minute) {
                        Ordering::Less => Ordering::Less,
                        Ordering::Greater => Ordering::Greater,
                        Ordering::Equal => Ordering::Equal,
                    },
                },
            },
        }
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct TimeStamp {
    month: i32,
    date: i32,
    hour: i32,
    minute: i32,
}

impl TimeStamp {
    fn from(input: &str) -> TimeStamp {
        let cleaned = input
            .replace("[1518-", "")
            .replace("-", " ")
            .replace(":", " ")
            .replace("]", "");

        let tokens: Vec<&str> = cleaned.split_whitespace().collect();

        let mut ts = TimeStamp {
            month: tokens[0].parse().unwrap(),
            date: tokens[1].parse().unwrap(),
            hour: tokens[2].parse().unwrap(),
            minute: tokens[3].parse().unwrap(),
        };

        if ts.hour != 0 {
            ts.date += 1;
            ts.hour = 0;
            ts.minute = 0;
        }

        ts
    }
}

struct Guard {
    id: i32,
    schedule: [i32; 60],
}

impl Guard {
    fn from(rec: &Record) -> Guard {
        if let LineType::Guard(id) = rec.line_type {
            Guard {
                id,
                schedule: [0; 60],
            }
        } else {
            panic!("Can't create guard from wrong record linetype")
        }
    }

    fn apply_nap(&mut self, start: &TimeStamp, end: &TimeStamp) {
        for i in start.minute as usize..end.minute as usize {
            self.schedule[i] += 1;
        }
    }
}
