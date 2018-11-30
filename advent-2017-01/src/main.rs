fn sum_1(input: &str) -> Option<u32> {
    let mut res: u32 = 0;
    let mut prev = 11;
    let first_digit = input.chars().next()?;

    for character in input.chars() {
        let digit = character.to_digit(10)?;
        if prev == digit {
            res += prev
        }
        prev = digit;
    }
    if prev == first_digit.to_digit(10)? {
        res += prev
    }

    Some(res)
}

fn sum_2(input: &str) -> Option<u32> {
    let mut res: u32 = 0;
    let digits: Vec<Option<u32>> = input.chars().map(|s| s.to_digit(10)).collect();
    let len = digits.len();

    for i in 0..len {
        let digit = digits[i]?;
        let next_digit = digits[(i + (len / 2)) % len]?;
        if digit == next_digit {
            res += digit
        }
    }

    Some(res)
}

fn main() {
    let input = include_str!("input.txt").trim();
    println!("{}", sum_1(input).unwrap());
    println!("{}", sum_2(input).unwrap());
}

#[test]
fn test_sum_1() {
    assert_eq!(sum_1(""), None);
    assert_eq!(sum_1("112A2"), None);
    assert_eq!(sum_1("1122"), Some(3));
    assert_eq!(sum_1("1111"), Some(4));
    assert_eq!(sum_1("1234"), Some(0));
    assert_eq!(sum_1("91212129"), Some(9));
}

#[test]
fn test_sum_2() {
    assert_eq!(sum_2(""), Some(0));
    assert_eq!(sum_2("112A2"), None);
    assert_eq!(sum_2("1212"), Some(6));
    assert_eq!(sum_2("1221"), Some(0));
    assert_eq!(sum_2("123425"), Some(4));
    assert_eq!(sum_2("123123"), Some(12));
    assert_eq!(sum_2("12131415"), Some(4));
}
