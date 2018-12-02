use std::collections::HashMap;
use std::collections::HashSet;

fn intersect(sets: Vec<&HashSet<usize>>) -> HashSet<usize> {
    let mut res: HashSet<usize> = sets[0].iter().cloned().collect();
    for set in sets.iter().skip(1) {
        res = &res & set;
    }
    res
}

fn part1(input: &str) -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    for line in input.lines() {
        let mut counts = HashMap::new();
        for chr in line.chars() {
            *counts.entry(chr).or_insert(0) += 1;
        }

        if counts.values().any(|c| *c == 2) {
            twos += 1;
        }
        if counts.values().any(|c| *c == 3) {
            threes += 1;
        }
    }

    return twos * threes;
}

fn part2(input: &str) -> Option<(&str, &str, usize)> {
    let lines: Vec<_> = input.lines().collect();
    let line_length = lines[0].chars().count();
    let mut occurences: Vec<HashMap<char, HashSet<usize>>> = (0..line_length).map(|_| HashMap::new()).collect();

    for (line_index, line) in lines.iter().enumerate() {
        for (char_index, chr) in line.chars().enumerate() {
            let mut set = occurences[char_index].entry(chr).or_insert_with(HashSet::new);
            set.insert(line_index);
        }
    }

    for (line_index, line) in lines.iter().enumerate() {
        let chars: Vec<_> = line.chars().collect();
        let matching_sets: Vec<&HashSet<usize>> = occurences
            .iter()
            .enumerate()
            .map(|(i, map)| map.get(&chars[i]).unwrap())
            .collect();

        for i in 0..matching_sets.len() {
            let mut other_sets = vec!();
            other_sets.extend_from_slice(&matching_sets[0..i]);
            other_sets.extend_from_slice(&matching_sets[i+1..line_length]);
            let intersection = intersect(other_sets);
            if intersection.len() == 2 {
                let other_line = *intersection.iter().filter(|n| **n != line_index).next()?;
                return Some((lines[line_index], lines[other_line], i))
            }
        }
    }

    None
}

fn main() {
    let input = include_str!("input.txt").trim();
    println!("{:?}", part1(input));
    if let Some((id1, id2, differing_char)) = part2(input) {
        println!("{}_{}", &id1[0..differing_char], &id2[differing_char+1..]);
    }
}

#[test]
fn test_part1() {
    let input = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";
    assert_eq!(part1(input), 12);
}

#[test]
fn test_part2() {
    let input = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";
    assert_eq!(part2(input), Some(("fghij", "fguij", 2)));
}