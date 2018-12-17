fn part1(num: usize) -> String {
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut elves = [0, 1];
    while recipes.len() < num + 10 {
        let sum: u8 = recipes[elves[0]] + recipes[elves[1]];
        let digits: Vec<u8> = sum.to_string().chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        recipes.extend(digits);


        for elf in elves.iter_mut() {
            *elf = (1 + recipes[*elf] as usize + *elf) % recipes.len();
        }
    }
    recipes[num..num+10].into_iter().map(u8::to_string).collect()
}

fn part2(sequence: &str) -> usize {
    let sequence: Vec<u8> = sequence.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut elves = [0, 1];
    let mut last_digits = 0;
    while !matches(&recipes, &sequence, last_digits)  {
        let sum: u8 = recipes[elves[0]] + recipes[elves[1]];
        let digits: Vec<u8> = sum.to_string().chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        last_digits = digits.len();
        recipes.extend(digits);


        for elf in elves.iter_mut() {
            *elf = (1 + recipes[*elf] as usize + *elf) % recipes.len();
        }
    }
    recipes.len() - sequence.len() - get_match(&recipes, &sequence, last_digits).unwrap()
}

fn matches(recipes: &Vec<u8>, sequence: &Vec<u8>, additions: usize) -> bool {
    if recipes.len() < sequence.len() {
        return false;
    } else {
        get_match(recipes, sequence, additions).is_some()
    }
}

fn get_match(recipes: &Vec<u8>, sequence: &Vec<u8>, additions: usize) -> Option<usize> {
    (0..additions).find(|&i| {
        if recipes.len() - sequence.len() < i {
            false
        } else {
            recipes[recipes.len() - sequence.len() - i..recipes.len() - i] == sequence[..]
        }
    })

}

fn main() {
    println!("{}", part1(864801));
    println!("{}", part2("864801"));
}

#[test]
fn test_part1() {
    assert_eq!(part1(9), "5158916779");
    assert_eq!(part1(5), "0124515891");
    assert_eq!(part1(18), "9251071085");
    assert_eq!(part1(2018), "5941429882");
}

#[test]
fn test_part2() {
    assert_eq!(part2("51589"), 9);
    assert_eq!(part2("01245"), 5);
    assert_eq!(part2("92510"), 18);
    assert_eq!(part2("59414"), 2018);
}