fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|c| c.parse::<i32>().unwrap()).collect()
}

fn solve(input: Vec<i32>) -> i32 {
    let mut sum = 0;
    for window in input.windows(2) {
        if window[0] < window[1] {
            sum += 1;
        }
    }
    sum
}

fn solve2(input: Vec<i32>) -> i32 {
    let window_sums: Vec<i32> = input.windows(3).map(|w| w.iter().sum()).collect();
    let mut sum = 0;
    for window in window_sums.windows(2) {
        if window[0] < window[1] {
            sum += 1;
        }
    }
    sum
}

fn main() {
    let input = parse(include_str!("input").trim());
    let output = solve(input);
    println!("{:?}", output);

    let input = parse(include_str!("input").trim());
    let output = solve2(input);
    println!("{:?}", output);
}

#[test]
fn test_part1() {
    assert_eq!(
        solve(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263,]),
        7
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        solve2(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263,]),
        5
    );
}
