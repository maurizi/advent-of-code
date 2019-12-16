fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|c| c.parse::<usize>().unwrap()).collect()
}

fn part1(mut input: Vec<usize>, noun: usize, verb: usize) -> Vec<usize> {
    input[1] = noun;
    input[2] = verb;
    let mut pos = 0;
    loop {
        match input[pos] {
            opcode @ 1 ..= 2 => {
                let a = input[input[pos+1]];
                let b = input[input[pos+2]];
                let dest_idx = input[pos+3];

                input[dest_idx] = if opcode == 1 {
                    a + b
                } else {
                    a * b
                }
            },
            99 => break,
            _ => panic!("halp!")
        }
        pos += 4;
    }
    input
}

fn part2(input: Vec<usize>) -> Option<(usize, usize)> {
    for i in 0 ..= 99 {
        for j in 0 ..= 99 {
            let res = part1(input.clone(), i, j)[0];
            if res == 19690720 {
                return Some((i, j));
            }
        }
    }
    None
}

fn main() {
    let input = parse(include_str!("input.txt").trim());
    let output = part1(input, 12, 2);
    println!("{:?}", output);
    println!("{}", output[0]);
    let input = parse(include_str!("input.txt").trim());
    println!("{:?}", part2(input).unwrap());
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(vec![1,9,10,3,2,3,11,0,99,30,40,50], 9, 10),
        vec![3500,9,10,70,2,3,11,0,99,30,40,50]
    );
    assert_eq!(part1(vec![ 1,0,0,0,99 ], 0, 0), vec![ 2,0,0,0,99 ]);
    assert_eq!(part1(vec![ 2,3,0,3,99 ], 3, 0), vec![ 2,3,0,6,99 ]);
    assert_eq!(part1(vec![ 2,4,4,5,99,0 ], 4, 4), vec![ 2,4,4,5,99,9801 ]);
    assert_eq!(part1(vec![ 1,1,1,4,99,5,6,0,99 ], 1, 1), vec![ 30,1,1,4,2,5,6,0,99 ]);
}
