fn seat_id(input: &str) -> Option<usize> {
    let (mut row_min, mut col_min, mut row_max, mut col_max) = (0, 0, 127, 7);
    for ch in input.chars() {
        match ch {
            'B' | 'F' => {
                let half = (row_max - row_min + 1) / 2;
                if ch == 'F' {
                    row_max -= half;
                } else {
                    row_min += half;
                }
            }
            'R' | 'L' => {
                let half = (col_max - col_min + 1) / 2;
                if ch == 'L' {
                    col_max -= half;
                } else {
                    col_min += half;
                }
            }
            _ => {}
        }
    }
    if row_max != row_min || col_max != col_min {
        None
    } else {
        Some(row_min * 8 + col_min)
    }
}

fn part1(input: &str) -> Option<usize> {
    return input.trim().lines().map(seat_id).flatten().max();
}

fn part2(input: &str) -> Option<usize> {
    let mut ids: Vec<_> = input.trim().lines().map(seat_id).flatten().collect();
    ids.sort();
    for idx in 2..ids.len() {
        if ids[idx - 1] - ids[idx - 2] > 1 {
            return Some(ids[idx - 1] - 1);
        }
    }
    None
}

fn main() {
    let output = part1(include_str!("input"));
    println!("{:?}", output);
    let output = part2(include_str!("input"));
    println!("{:?}", output);
}

#[test]
fn test_part1() {
    assert_eq!(part1("BFFFBBFRRR"), Some(567));
    assert_eq!(part1("FFFBBBFRRR"), Some(119));
    assert_eq!(part1("BBFFBBFRLL"), Some(820));
}
