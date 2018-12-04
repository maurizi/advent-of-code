extern crate itertools;

use std::collections::HashMap;
use itertools::Itertools;

enum Event {
    Start { time: i32, id: i32 },
    Sleep { time: i32 },
    Wake { time: i32 }
}

impl Event {
    fn from_input(line: &str) -> Option<Event> {
        let time_sep = line.find(":")?;
        let msg_sep = line.find("] ")?;

        let time = line[time_sep + 1..time_sep + 3].parse().ok()?;
        let message = &line[msg_sep + 2..];
        if message.starts_with("Guard") {
            let id_start = line.find("#")?;
            let id_end = line[id_start..].find(" ")?;
            Some(Event::Start {
                time,
                id: line[id_start+1..id_start + id_end].parse().ok()?
            })
        } else if message.starts_with("falls asleep") {
            Some(Event:: Sleep { time })
        } else {
            Some(Event:: Wake { time })
        }
    }
}

fn parse_events(input: &str) -> Vec<Event> {
    let lines = input.lines().sorted_by_key(|f| *f);
    lines.iter().map(|l| Event::from_input(*l)).flat_map(|i| i).collect()
}


fn get_sleep_times(events: &mut Vec<Event>) -> HashMap<i32, HashMap<i32, i32>> {
    let mut guards: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    let mut guard = 0;
    let mut sleep = 0;
    for event in events {
        match event {
            Event::Start { time: _, id } => guard = *id,
            Event::Sleep { time } => sleep = *time,
            Event::Wake { time } => {
                let times = guards.entry(guard).or_insert_with(|| HashMap::new());
                for i in sleep..*time {
                    *times.entry(i).or_insert(0) += 1;
                }
            }
        }
    }
    guards
}

fn part1(input: &str) -> Option<(i32, i32)> {
    let mut events = parse_events(input);
    let guards = get_sleep_times(&mut events);

    let (&sleepiest, times): (_, &HashMap<i32, i32>) = guards.iter().max_by_key(|(_, times)| {
        let sum: i32 = times.values().sum();
        sum
    })?;

    let &sleepiest_time = times.iter().max_by_key(|(_, count)| **count)?.0;

    Some((sleepiest, sleepiest_time))
}

fn part2(input: &str) -> Option<(i32, i32)> {
    let mut events = parse_events(input);
    let guards = get_sleep_times(&mut events);

    let (&sleepiest, times): (_, &HashMap<i32, i32>) = guards.iter().max_by_key(|(_, times)| {
        times.iter().max_by_key(|(_, count)| **count).unwrap().1
    })?;

    let &sleepiest_time = times.iter().max_by_key(|(_, count)| **count)?.0;

    Some((sleepiest, sleepiest_time))
}

fn main() {
    let input = include_str!("input.txt").trim();
    println!("{:?}", part1(input).unwrap());
    println!("{:?}", part2(input).unwrap());
}

#[test]
fn test_part1() {
    let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
";
    assert_eq!(part1(input), Some((10, 24)));
}

#[test]
fn test_part2() {
    let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
";
    assert_eq!(part2(input), Some((99, 45)));
}
