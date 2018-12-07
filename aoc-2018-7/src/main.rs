#[macro_use]
extern crate intrusive_collections;
use intrusive_collections::{LinkedList, LinkedListLink};

struct Step {
    part: &str,
    link: LinkedListLink
}

intrusive_adapter!(ValueAdapter<'a> = &'a Step: Step { link: LinkedListLink });

fn part1(input: &str) -> Option<String> {
    let output = String::new();

    Some(output)
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_part1() {
    let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";
    assert_eq!(part1(input), Some("CABDFE".to_owned()));
}