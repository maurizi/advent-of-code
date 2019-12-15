#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate petgraph;
extern crate itertools;

use petgraph::Direction::Incoming;
use petgraph::graph::NodeIndex;
use petgraph::graph::Graph;
use regex::Regex;
use std::collections::HashMap;
use petgraph::Directed;
use itertools::Itertools;

lazy_static! {
    static ref REQUIREMENT: Regex = Regex::new(r"^Step (\w+) must be finished before step (\w+) can begin.$").unwrap();
}

fn parse_line(line: &str) -> Option<(&str, &str)> {
    let captures = REQUIREMENT.captures(&line)?;
    Some((captures.get(1)?.as_str(), captures.get(2)?.as_str()))
}

fn has_required(step_graph: &Graph<&str, &str>, node_id: NodeIndex, output: &str) -> bool {
    let neighbor_requires: Vec<_> = step_graph.neighbors_directed(node_id, Incoming).collect();
    let res = neighbor_requires.into_iter().all(|n| output.contains(step_graph[n]));
    res
}

fn walk(step_graph: &Graph<&str, &str>) -> Option<String> {
    let mut available: Vec<NodeIndex> = step_graph.externals(Incoming).collect();
    let mut output = String::new();

    while !available.is_empty() {
        let min = *available
            .iter()
            .filter(|&&n| has_required(&step_graph, n, &output))
            .min_by_key(|&&n| step_graph[n]).unwrap();
        output += step_graph[min];

        available = available.into_iter().filter(|&n| n != min).collect();
        available.extend(step_graph.neighbors(min));
    }

    Some(output)
}

fn part1(input: &str) -> Option<String> {
    let step_graph = get_step_graph(input)?;
    walk(&step_graph)
}

fn get_step_graph(input: &str) -> Option<Graph<&str, &str>> {
    let _requirements: Option<_> = input.lines().map(parse_line).collect();
    let requirements: Vec<(&str, &str)> = _requirements?;
    let mut step_graph = Graph::<&str, &str>::new();
    let mut steps: HashMap<&str, NodeIndex> = HashMap::new();
    for (before, after) in requirements.iter() {
        steps.entry(before).or_insert_with(|| step_graph.add_node(before));
        steps.entry(after).or_insert_with(|| step_graph.add_node(after));
    }
    step_graph.extend_with_edges(requirements.iter().map(|(before, after): &(&str, &str)| {
        (steps[before], steps[after])
    }));
    Some(step_graph)
}

fn get_step_time(step: &str, delay: u8) -> Option<usize> {
    Some(step.bytes().next()? as usize - 'A' as usize + 1 + delay as usize)
}

fn build_time(step_graph: &Graph<&str, &str>, workers: u8, delay: u8) -> Option<usize> {
    let mut available: Vec<NodeIndex> = step_graph.externals(Incoming).collect();
    let mut done = String::new();
    let mut in_progress = vec![None; workers as usize];
    let mut time = 0;

    while !available.is_empty() || in_progress.iter().any(Option::is_some) {
        let open_workers: Vec<_> = in_progress.iter().positions(Option::is_none).collect();
        for worker in open_workers {
            let next = available
                .iter()
                .cloned()
                .filter(|&n| has_required(&step_graph, n, &done))
                .min_by_key(|&n| step_graph[n]);
            if let Some(next_step) = next {
                in_progress[worker] = Some((next_step, time + get_step_time(step_graph[next_step], delay)?));
                available = available.into_iter().filter(|&n| n != next_step).collect();
            }
        }
        time += 1;
        let complete: Vec<_> = in_progress.iter().positions(|&work| {
            if let Some((step, end_time)) = work {
                end_time == time
            } else {
                false
            }
        }).collect();
        for index in complete {
            let node = in_progress[index]?.0;
            done += step_graph[node];
            available.extend(step_graph.neighbors(node));
            in_progress[index] = None;
        }
    }

    Some(time)
}

fn part2(input: &str, workers: u8, delay: u8) -> Option<usize> {
    let step_graph = get_step_graph(input)?;
    build_time(&step_graph, workers, delay)
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input).unwrap());
    println!("{}", part2(input, 5, 60).unwrap());
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

#[test]
fn test_part2() {
    let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";
    assert_eq!(part2(input, 2, 0), Some(15));
}