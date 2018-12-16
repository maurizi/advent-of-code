extern crate arrayvec;
extern crate itertools;

use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::VecDeque;
use arrayvec::ArrayVec;
use std::str::Bytes;
use std::iter::Map;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;
use itertools::Itertools;

const THRESHHOLD: usize = 10;

#[derive(Clone)]
struct Plants {
    min: isize,
    max: isize,
    pots: HashMap<isize, bool>,
    rules: HashMap<ArrayVec<[bool; 5]>, bool>
}

impl Plants {
    fn pots_to_string(&self) -> String {
        let pots = self.pots.iter().sorted_by_key(|(&i, p)| i);
        pots.map(|(_, p)| if *p { '#' } else { '.' }).collect()
    }

    fn rules_to_string(&self) -> String {
        self.rules.iter().map(|(input, output)| {
            let in_str: String = input.iter().map(|p| if *p { '#' } else { '.' }).collect();
            let out_str = if *output { "#" } else { "." };
            in_str + " => " + out_str + "\n"
        }).collect()

    }
}

impl Debug for Plants {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_fmt(format_args!("initial state: {}\n\n{}", &self.pots_to_string(), self.rules_to_string()))
    }
}

fn to_bool_list(input: &str) -> Map<Bytes, fn(u8) -> bool> {
    input.bytes().map(|b| b == '#' as u8)
}

fn to_enumerated_bools(input: &str) -> HashMap<isize, bool> {
    to_bool_list(input)
        .enumerate()
        .map(|(i, p)| (i as isize, p))
        .collect()
}

impl Plants {
    fn from_input(input: &str) -> Option<Plants> {
        let lines: Vec<_> = input.lines().collect();
        let pots = to_enumerated_bools(&lines[0][15..]);
        let rules: HashMap<ArrayVec<[bool; 5]>, bool> = lines[2..].iter()
            .map(|&line| (ArrayVec::from_iter(to_bool_list(&line[0..5])), &line[9..10] == "#"))
            .collect();
        let (&min, _) = pots.iter().min_by_key(|(&i, _)| i)?;
        let (&max, _) = pots.iter().max_by_key(|(&i, _)| i)?;

        Some(Plants { pots, rules, min, max })
    }

    fn age(&mut self) {
        let mut new_pots = self.pots.clone();
        for i in 1..3 {
            self.pots.insert(self.min - i, false);
            self.pots.insert(self.max + i, false);
        }

        for i in self.min - 2 ..= self.max + 2 {
            let window = ArrayVec::from([
                self.pots.get(&(i - 2)).unwrap_or(&false).clone(),
                self.pots.get(&(i - 1)).unwrap_or(&false).clone(),
                self.pots.get(&(i)).unwrap_or(&false).clone(),
                self.pots.get(&(i + 1)).unwrap_or(&false).clone(),
                self.pots.get(&(i + 2)).unwrap_or(&false).clone(),
            ]);
            let rule_matches = self.rules.get(&window).map(|b| *b).unwrap_or(false);
            new_pots.insert(i, rule_matches);
        }

        for i in 1..3 {
            if !new_pots[&(self.min - i)] {
                new_pots.remove(&(self.min - i));
            }
            if !new_pots[&(self.max + i)] {
                new_pots.remove(&(self.max + i));
            }
        }
        let (&min, _) = new_pots.iter().min_by_key(|(&i, _)| i).unwrap();
        let (&max, _) = new_pots.iter().max_by_key(|(&i, _)| i).unwrap();
        self.min = min;
        self.max = max;
        self.pots = new_pots;
    }
}

fn sum_of_plant_nums(input: &str, generations: usize) -> Option<isize> {
    let mut plants = Plants::from_input(input)?;
    let mut diffs = HashMap::new();
    let mut last_sum = 0;
    for i in 1..=generations {
        plants.age();
        let sum: isize = plants.pots.iter().filter(|(_, p)| **p).map(|(i, _)| *i).sum();

        let diff = sum - last_sum;
        let diff_count = diffs.entry(diff).or_insert(0);
        if *diff_count > THRESHHOLD {
            return Some((generations - i) as isize * diff + sum);
        } else {
            *diff_count += 1;
        }
        last_sum = sum;
    }
    Some(plants.pots.iter().filter(|(_, p)| **p).map(|(i, _)| *i).sum())
}

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", sum_of_plant_nums(input, 20).unwrap());
    println!("{:?}", sum_of_plant_nums(input, 50000000000).unwrap());
}

#[test]
fn test_parse() {
    let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";
    let res = Plants::from_input(input);
    println!("{:?}", res);
    assert_eq!(res.is_some(), true);
    let plants = res.unwrap();
    assert_eq!(plants.pots.len(), 25);
    assert_eq!(plants.rules.len(), 14);
}

#[test]
fn test_generation() {
    let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";
    let mut plants = Plants::from_input(input).unwrap();
    println!("{:?}", plants.pots_to_string());

    let expected_output = to_enumerated_bools("#...#....#.....#..#..#..#");

    plants.age();
    println!("{:?}", plants.pots_to_string());
    assert_eq!(plants.pots, expected_output);

    let expected_output = to_enumerated_bools("##..##...##....#..#..#..##");

    plants.age();
    println!("{:?}", plants.pots_to_string());
    assert_eq!(plants.pots, expected_output);
}

#[test]
fn test_sum() {
    let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";
    let sum = sum_of_plant_nums(input, 20);
    assert_eq!(sum, Some(325));
}
