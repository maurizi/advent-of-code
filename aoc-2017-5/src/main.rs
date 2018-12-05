fn part1(input: &str) -> u32 {
    let mut jumps: Vec<i32> = input.lines().map(|line| line.parse()).flatten().collect();

    let mut pos = 0i32;
    let mut steps = 0;
    while pos >= 0 && (pos as usize) < jumps.len() {
        let offset = jumps[pos as usize];
        jumps[pos as usize] += 1;
        pos += offset;
        steps += 1;
    }
    steps
}

fn part2(input: &str) -> u32 {
    let mut jumps: Vec<i32> = input.lines().map(|line| line.parse()).flatten().collect();

    let mut pos = 0i32;
    let mut steps = 0;
    while pos >= 0 && (pos as usize) < jumps.len() {
        let offset = jumps[pos as usize];
        jumps[pos as usize] += if offset >= 3 {
            -1
        } else {
            1
        };
        pos += offset;
        steps += 1;
    }
    steps
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[test]
fn test_part1() {
    let input = "0
3
0
1
-3
";
    assert_eq!(part1(input), 5);
}

#[test]
fn test_part2() {
    let input = "0
3
0
1
-3
";
    assert_eq!(part2(input), 10);
}
