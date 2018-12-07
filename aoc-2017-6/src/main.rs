#[macro_use] extern crate maplit;

use std::collections::HashSet;

fn part1(input: &str) -> Option<usize> {
    let mut banks: Vec<i32> = input.split("\t").map(|num| num.parse()).flatten().collect();

    let mut seen:  HashSet<Vec<i32>> = HashSet::new();
    while !seen.contains(&banks) {
        seen.insert(banks.clone());
        let (max_index, &max_blocks) = banks.iter().enumerate().max_by_key(|&(index, val)| (*val, -(index as i128)))?;
        let mut bank_loop= if max_index == banks.len() - 1 {
            let range: Vec<_> = (0 .. banks.len()).collect();
            range.into_iter().cycle()
        } else {
            let range: Vec<_> = (max_index+1 .. banks.len()).chain(0 .. max_index+1).collect();
            range.into_iter().cycle()
        };
        let mut blocks = max_blocks;
        banks[max_index] = 0;
        while blocks > 0 {
            let next: usize = bank_loop.next()?;
            banks[next] += 1;
            blocks -= 1;
        }
    }

    Some(seen.len())
}

fn part2(input: &str) -> Option<i32> {
    let mut banks: Vec<i32> = input.split("\t").map(|num| num.parse()).flatten().collect();

    let mut steps = 0;
    let mut first_repeat_steps = -1;
    let mut seen:  HashSet<Vec<i32>> = HashSet::new();
    let mut first_repeat = vec![];
    while first_repeat != banks {
        steps += 1;
        if first_repeat.is_empty() && !seen.insert(banks.clone()) {
            first_repeat = banks.clone();
            first_repeat_steps = steps;
        }
        let (max_index, &max_blocks) = banks.iter().enumerate().max_by_key(|&(index, val)| (*val, -(index as i128)))?;
        let mut bank_loop= if max_index == banks.len() - 1 {
            let range: Vec<_> = (0 .. banks.len()).collect();
            range.into_iter().cycle()
        } else {
            let range: Vec<_> = (max_index+1 .. banks.len()).chain(0 .. max_index+1).collect();
            range.into_iter().cycle()
        };
        let mut blocks = max_blocks;
        banks[max_index] = 0;
        while blocks > 0 {
            let next: usize = bank_loop.next()?;
            banks[next] += 1;
            blocks -= 1;
        }
    }

    Some(steps - first_repeat_steps + 1)
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input).unwrap());
    println!("{}", part2(input).unwrap());
}

#[test]
fn test_part1() {
    let input = "0\t2\t7\t0";
    assert_eq!(part1(input), Some(5))
}

#[test]
fn test_part2() {
    let input = "0\t2\t7\t0";
    assert_eq!(part2(input), Some(4))
}
