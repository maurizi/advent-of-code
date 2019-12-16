use std::cmp::max;


fn fuel(mass: i32) -> i32 {
    max(0, (mass / 3) - 2)
}

fn part1(input: &str) -> i32 {
    input.lines().map(|c| c.parse::<i32>().unwrap()).map(fuel).sum()
}

fn part2(input: &str) -> i32 {
    input.lines().map(|c| c.parse::<i32>().unwrap()).map(|mass| {
        let mut fuel_mass: i32 = fuel(mass);
        let mut total_mass = fuel_mass;

        while fuel(fuel_mass) > 0 {
            fuel_mass = fuel(fuel_mass);
            total_mass += fuel_mass;
        }

        total_mass
    }).sum()
}

fn main() {
    let input = include_str!("input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[test]
fn test_part1() {
    assert_eq!(part1("12\n"), 2);
    assert_eq!(part1("14\n"), 2);
    assert_eq!(part1("1969\n"), 654);
    assert_eq!(part1("100756\n"), 33583);
}

#[test]
fn test_part2() {
    assert_eq!(part2("14\n"), 2);
    assert_eq!(part2("1969\n"), 966);
    assert_eq!(part2("100756\n"), 50346);
}
