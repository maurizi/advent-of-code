struct Claim {
    id: u32,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn from_input(line: &str) -> Option<Claim> {
        let id_sep = line.find(" @ ")?;
        let left_sep = line.find(",")?;
        let top_sep = line.find(": ")?;
        let width_sep = line.find("x")?;

        Some(Claim {
            id: line[1..id_sep].parse().ok()?,
            left: line[id_sep + 3..left_sep].parse().ok()?,
            top: line[left_sep + 1..top_sep].parse().ok()?,
            width: line[top_sep + 2..width_sep].parse().ok()?,
            height: line[width_sep + 1..].parse().ok()?,
        })
    }
}

fn parse_claims(input: &str) -> Vec<Claim> {
    input.lines().map(Claim::from_input).flatten().collect()
}

fn get_overlaps(claims: &Vec<Claim>) -> Vec<Vec<Vec<u32>>> {
    let mut squares = vec![vec![vec![]; 1000]; 1000];

    for claim in claims.iter() {
        for i in claim.left..claim.left + claim.width {
            for j in claim.top..claim.top + claim.height {
                squares[i][j].push(claim.id);
            }
        }
    }
    squares
}

fn part1(input: &str) -> Option<usize> {
    let claims = parse_claims(input);
    let squares = get_overlaps(&claims);

    Some(squares.iter().flatten().filter(|list| list.len() > 1).count())
}

fn part2(input: &str) -> Option<Claim> {
    let claims = parse_claims(input);
    let squares = get_overlaps(&claims);

    'outer: for claim in claims {
        for i in claim.left..claim.left + claim.width {
            for j in claim.top..claim.top + claim.height {
                if squares[i][j].len() != 1 {
                    continue 'outer;
                }
            }
        }
        // If we got here every square was 1 for this claim
        return Some(claim);
    }

    None
}

fn main() {
    let input = include_str!("input.txt").trim();
    println!("{:?}", part1(input));
    println!("{:?}", part2(input).unwrap().id);
}

#[test]
fn test_part1() {
    let input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
";
    assert_eq!(part1(input).unwrap(), 4);
}

#[test]
fn test_part2() {
    let input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
";
    assert_eq!(part2(input).unwrap().id, 3);
}
