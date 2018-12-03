struct Claim {
    id: u32,
    left: usize,
    top: usize,
    width: usize,
    height: usize
}

impl Claim {
    fn from_input(line: &str) -> Option<Claim> {
        let id_sep = line.find(" @ ")?;
        let left_sep = line.find(",")?;
        let top_sep = line.find(": ")?;
        let width_sep = line.find("x")?;

        Some(Claim {
            id: line[1..id_sep].parse().ok()?,
            left: line[id_sep+3..left_sep].parse().ok()?,
            top: line[left_sep+1..top_sep].parse().ok()?,
            width: line[top_sep+2..width_sep].parse().ok()?,
            height: line[width_sep+1..].parse().ok()?
        })
    }
}

fn part1(input: &str) -> Option<usize> {
    let mut squares = vec![vec![0; 1000]; 1000];

    for line in input.lines() {
        let claim = Claim::from_input(line)?;
        for i in claim.left .. claim.left + claim.width {
            for j in claim.top .. claim.top + claim.height {
                squares[i][j] += 1;
            }
        }
    }

    Some(squares.iter().flatten().filter(|count| **count > 1).count())
}

fn part2(input: &str) -> Option<Claim> {
    let mut squares = vec![vec![0; 1000]; 1000];
    let claims: Vec<Claim> = input.lines().map(Claim::from_input).flatten().collect();

    for claim in claims.iter() {
        for i in claim.left .. claim.left + claim.width {
            for j in claim.top .. claim.top + claim.height {
                squares[i][j] += 1;
            }
        }
    }

    'outer: for claim in claims {
        for i in claim.left .. claim.left + claim.width {
            for j in claim.top .. claim.top + claim.height {
                if squares[i][j] != 1 {
                    continue 'outer;
                }
            }
        }
        // If we got here every square was 1
        return Some(claim)
    }

    None
}

fn main() {
    let input = include_str!("input.txt").trim();
    println!("{:?}", part1(input));
    println!("{:?}", part2(input).unwrap().id);
}
