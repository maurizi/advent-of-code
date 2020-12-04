use regex::Regex;

fn part1(input: &str) -> Option<usize> {
    let mut num = 0;
    let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    for line in input.trim().lines() {
        let groups = re.captures(line).unwrap();
        let min = groups.get(1)?.as_str().parse::<usize>().ok()?;
        let max = groups.get(2)?.as_str().parse::<usize>().ok()?;
        let letter = groups.get(3)?.as_str();
        let pw = groups.get(4)?.as_str().trim();
        let num_letter = pw.matches(letter).count();
        if num_letter >= min && num_letter <= max {
            num += 1;
        }
    }
    return Some(num);
}

fn part2(input: &str) -> Option<usize> {
    let mut num = 0;
    let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    for line in input.trim().lines() {
        let groups = re.captures(line).unwrap();
        let pos1 = groups.get(1)?.as_str().parse::<usize>().ok()? - 1;
        let pos2 = groups.get(2)?.as_str().parse::<usize>().ok()? - 1;
        let letter = groups.get(3)?.as_str().chars().next()?;
        let pw: Vec<_> = groups.get(4)?.as_str().trim().chars().collect();

        if pw[pos1] != pw[pos2] && (pw[pos1] == letter || pw[pos2] == letter) {
            num += 1;
        }
    }
    return Some(num);
}

fn main() {
    let output = part1(include_str!("input"));
    println!("{:?}", output);
    let output = part2(include_str!("input"));
    println!("{:?}", output);
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(
            r"
    1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc
    "
        ),
        Some(2)
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(
            r"
    1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc
    "
        ),
        Some(1)
    );
}
