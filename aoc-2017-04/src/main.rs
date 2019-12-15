extern crate itertools;

use std::collections::HashSet;
use std::iter::Iterator;
use std::iter::FromIterator;

use itertools::Itertools;

fn validate1(input_line: &str) -> bool {
    let words: Vec<&str> = input_line.split(" ").collect();
    let words_set: HashSet<&str> = words.iter().cloned().collect();

    words.len() == words_set.len()
}

fn part1(input: &str) -> usize {
    input.lines().filter(|&line| validate1(&line)).count()
}

fn validate2(input_line: &str) -> bool {
    let words: Vec<&str> = input_line.split(" ").collect();
    let words_set: HashSet<String> = words.iter().cloned().map(|word: &str| String::from_iter(word.chars().sorted())).collect();

    words.len() == words_set.len()
}

fn part2(input: &str) -> usize {
    input.lines().filter(|&line| validate2(&line)).count()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[test]
fn test_validate1() {
    assert_eq!(validate1("aa bb cc dd ee"), true);
    assert_eq!(validate1("aa bb cc dd aa"), false);
    assert_eq!(validate1("aa bb cc dd aaa"), true);
}

#[test]
fn test_validate2() {
    assert_eq!(validate2("abcde fghij"), true);
    assert_eq!(validate2("abcde xyz ecdab"), false);
    assert_eq!(validate2("a ab abc abd abf abj"), true);
    assert_eq!(validate2("iiii oiii ooii oooi oooo"), true);
    assert_eq!(validate2("oiii ioii iioi iiio"), false);
}