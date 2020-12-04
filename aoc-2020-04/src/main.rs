use std::collections::HashMap;

use regex::Regex;

fn part1(input: &str) -> Option<usize> {
    let mut num = 0usize;
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let passports = input.trim().split("\n\n");
    for passport in passports {
        let data: Vec<_> = passport.split_whitespace().collect();
        if required
            .iter()
            .all(|field| data.iter().any(|item| item.starts_with(field)))
        {
            num += 1;
        }
    }
    return Some(num);
}

fn part2(input: &str) -> Option<usize> {
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    let mut num = 0usize;
    let passports = input.trim().split("\n\n");
    for passport in passports {
        let data: HashMap<_, _> = passport
            .split_whitespace()
            .map(|item| {
                let parts: Vec<_> = item.split(":").collect();
                (parts[0], parts[1])
            })
            .collect();

        if required.iter().all(|field| data.contains_key(field)) {
            // has required fields
            let byr = data["byr"]
                .parse::<usize>()
                .map_or(false, |num| num >= 1920 && num <= 2002);
            let iyr = data["iyr"]
                .parse::<usize>()
                .map_or(false, |num| num >= 2010 && num <= 2020);
            let eyr = data["eyr"]
                .parse::<usize>()
                .map_or(false, |num| num >= 2020 && num <= 2030);

            let hgt = if let Some(val) = data["hgt"].strip_suffix("cm") {
                val.parse::<usize>()
                    .map_or(false, |num| num >= 150 && num <= 193)
            } else if let Some(val) = data["hgt"].strip_suffix("in") {
                val.parse::<usize>()
                    .map_or(false, |num| num >= 59 && num <= 76)
            } else {
                false
            };

            let hcl = hcl_re.is_match(data["hcl"]);

            let ecl = valid_ecl.contains(&data["ecl"]);

            let pid = pid_re.is_match(data["pid"]);

            if byr && iyr && eyr && hgt && hcl && ecl && pid {
                num += 1;
            }
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
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
        ),
        Some(2)
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(
            r"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
        ),
        Some(0)
    );
    assert_eq!(
        part2(
            r"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"
        ),
        Some(4)
    );
}
