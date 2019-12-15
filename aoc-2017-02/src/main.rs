fn part1(input: &str) -> Option<i32> {
    let mut checksum = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line.split("\t").map(|num| num.parse()).flatten().collect();
        let max = nums.iter().max()?;
        let min = nums.iter().min()?;
        checksum += max - min;
    }
    Some(checksum)
}

fn part2(input: &str) -> Option<i32> {
    let mut checksum = 0;
    for line in input.lines() {
        let nums: Vec<i32> = line.split("\t").map(|num| num.parse()).flatten().collect();

        for (i, &num) in nums.iter().enumerate() {
            for &other_num in nums[..i].iter().chain(nums[i+1..].iter()) {
                if num % other_num == 0 && num > other_num {
                    checksum += num / other_num;
                }
            }
        }
    }
    Some(checksum)
}

fn main() {
    let input = include_str!("input.txt").trim();
    println!("{}", part1(input).unwrap());
    println!("{}", part2(input).unwrap());
}

#[test]
fn test_part1() {
    let input = "5\t1\t9\t5
7\t5\t3
2\t4\t6\t8
";
    assert_eq!(part1(input), Some(18));
}

#[test]
fn test_part2() {
    let input = "5\t9\t2\t8
9\t4\t7\t3
3\t8\t6\t5
";
    assert_eq!(part2(input), Some(9));
}
