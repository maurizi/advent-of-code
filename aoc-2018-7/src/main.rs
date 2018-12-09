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
use itertools::Itertools;
use std::collections::HashSet;

lazy_static! {
    static ref REQUIREMENT: Regex = Regex::new(r"^Step (\w+) must be finished before step (\w+) can begin.$").unwrap();
}

fn parse_line(line: &str) -> Option<(&str, &str)> {
    let captures = REQUIREMENT.captures(&line)?;
    Some((captures.get(1)?.as_str(), captures.get(2)?.as_str()))
}

fn has_required(step_graph: &Graph<&str, &str>, node_id: NodeIndex, output: &str) -> bool {
    let mut neighbor_requires: Vec<_> = step_graph.neighbors_directed(node_id, Incoming).collect();
    let res = neighbor_requires.into_iter().all(|n| output.contains(step_graph[n]));
    res
}

fn walk(step_graph: &Graph<&str, &str>, id: NodeIndex) -> Option<String> {
    let mut neighbors: Vec<_> = step_graph.neighbors(id).collect();
    let mut output = String::from(step_graph[id]);

    while !neighbors.is_empty() {
        let min = *neighbors
            .iter()
            .filter(|&&n| has_required(&step_graph, n, &output))
            .min_by_key(|&&n| step_graph[n]).unwrap();
        output += step_graph[min];

        neighbors = neighbors.into_iter().filter(|&n| n != min).collect();
        neighbors.extend(step_graph.neighbors(min));
    }

    Some(output)
}

fn part1<'a>(input: &'a str) -> Option<String> {
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

    let sources: Vec<NodeIndex> = step_graph.externals(Incoming).collect();
    let root = *sources.last()?;

    walk(&step_graph, root)
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input).unwrap());
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