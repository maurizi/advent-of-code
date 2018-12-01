#[macro_use] extern crate maplit;

fn part1(input: &str) -> i32 {
    input.lines().map(|c| c.parse::<i32>().unwrap()).sum()
}

fn part2(input: &str) -> Option<i32> {
    let mut freq = 0;
    let mut frequencies = hashset!{ freq };
    for chr in input.lines().cycle() {
        let num = chr.parse::<i32>().ok()?;
        freq += num;
        if frequencies.contains(&freq) {
            return Some(freq);
        }
        frequencies.insert(freq);
    }
    None
}

fn main() {
    let input = include_str!("input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input).unwrap());
}

#[test]
fn test_part1() {
    assert_eq!(part1("+1\n+1\n+1\n"), 3);
    assert_eq!(part1("+1\n+1\n-2"), 0);
    assert_eq!(part1("-1\n-2\n-3\n"), -6);
}

#[test]
fn test_part2() {
    assert_eq!(part2("+1\n-1\n").unwrap(), 0);
    assert_eq!(part2("+3\n+3\n+4\n-2\n-4\n").unwrap(), 10);
    assert_eq!(part2("-6\n+3\n+8\n+5\n-6\n").unwrap(), 5);
    assert_eq!(part2("+7\n+7\n-2\n-7\n-4\n").unwrap(), 14);
}
