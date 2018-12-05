use std::iter::FromIterator;
use std::collections::HashMap;

fn reduction_len<T: Into<String>>(input: T) -> usize {
    let mut data: Vec<char> = input.into().chars().collect();
    let mut i = 0;
    while i < data.len() - 1 {
        let chr = data[i];
        let next_char = data[i + 1];
        if next_char.to_lowercase().to_string() == chr.to_lowercase().to_string() && next_char != chr {
            data = Vec::from_iter(data[..i].into_iter().chain(data[i+2..].into_iter()).cloned());
            if i != 0 {
                i -= 1;
            }
            continue;
        }
        i += 1;
    }
    data.len()
}

fn best_reduction(input: &str) -> Option<usize> {
    let mut lengths = HashMap::new();
    for chr in char_iter::new('a', 'z') {
        let data: String = input.chars().filter(|c| {
            *c != chr && c.to_lowercase().to_string() != chr.to_string()
        }).collect();
        let len_with_removal = reduction_len(data);
        lengths.insert(chr, len_with_removal);
    }
    lengths.values().cloned().min()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", reduction_len(input));
    println!("{}", best_reduction(input).unwrap());
}

#[test]
fn test_part1() {
    assert_eq!(reduction_len("dabAcCaCBAcCcaDA"), 10);
    let input = include_str!("input.txt");
    assert_eq!(reduction_len(input), 10972);
}

#[test]
fn test_part2() {
    assert_eq!(best_reduction("dabAcCaCBAcCcaDA"), Some(4));
}