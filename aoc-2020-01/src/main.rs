use itertools::Itertools;

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|c| c.parse::<i32>().unwrap()).collect()
}

fn solve(input: Vec<i32>, num: usize) -> Option<i32> {
    let combo = input
        .into_iter()
        .combinations(num)
        .find(|combo| combo.iter().sum::<i32>() == 2020)?;
    return Some(combo.iter().product());
}

fn main() {
    let input = parse(include_str!("input").trim());
    let output = solve(input, 2);
    println!("{:?}", output);

    let input = parse(include_str!("input").trim());
    let output = solve(input, 3);
    println!("{:?}", output);
}

#[test]
fn test_part1() {
    assert_eq!(solve(vec![1721, 979, 366, 299, 675, 1456], 2), Some(514579));
    assert_eq!(
        solve(vec![1721, 979, 366, 299, 675, 1456], 3),
        Some(241861950)
    );
}
