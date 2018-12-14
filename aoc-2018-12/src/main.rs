#[macro_use]
extern crate nom;

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Plants {
    pots: VecDeque<bool>,
    rules: HashMap<[bool; 5], bool>
}

impl Plants {
    fn from_input(input: &str) -> Result<Plants, nom::Err<&str>> {
        let (_, (pots, rules)) = parse_plants(input)?;
        Ok(Plants {
            pots: VecDeque::from(pots),
            rules: rules.into_iter().collect()
        })
    }

    fn age(&mut self) {
        for _ in 0..2 {
            self.pots.push_front(false);
        }
        for _ in 0..2 {
            self.pots.push_back(false);
        }

        for i in 0..self.pots.len() {
            let window = [
                self.pots.get(i - 2).map(|b| *b).unwrap_or(false),
                self.pots.get(i - 1).map(|b| *b).unwrap_or(false),
                self.pots.get(i).map(|b| *b).unwrap_or(false),
                self.pots.get(i + 1).map(|b| *b).unwrap_or(false),
                self.pots.get(i + 2).map(|b| *b).unwrap_or(false),
            ];
            let rule_matches = self.rules.get(&window).map(|b| *b).unwrap_or(false);
            self.pots[i] = rule_matches;
        }

        if self.pots[0] && self.pots[1] {
            self.pots.pop_front();
            self.pots.pop_front();
        } else if self.pots[0] {
            self.pots.pop_front();
        }
        let (second_last, last) = (self.pots[self.pots.len() - 2], self.pots[self.pots.len() - 1]);
        if second_last && last {
            self.pots.pop_back();
            self.pots.pop_back();
        } else if last {
            self.pots.pop_back();
        }
    }
}

named!(initial_list<&str, Vec<bool>>,
    many1!(map!(take!(1), |ch| ch == "#"))
);

named!(rules_list<&str, Vec<([bool; 5], bool)>>,
    many1!(
        do_parse!(
            input_and_output: separated_pair!(
                count_fixed!(bool, map!(take!(1), |ch| ch == "#"), 5),
                tag!(" => "),
                map!(take!(1), |ch| ch == "#")
            ) >>
            tag!("\n") >>

            (input_and_output)
        )
    )
);

named!(parse_plants<&str, (Vec<bool>, Vec<([bool; 5], bool)>)>,
    do_parse!(
        tag!("initial state: ") >>
        pots: initial_list >>
        tag!("\n\n") >>
        rules: rules_list >>

        ((pots, rules))
    )
);

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_parse() {
    let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";
    let res = Plants::from_input(input);
    println!("{:?}", res);
    assert_eq!(res.is_ok(), true);
    let plants = res.unwrap();
    assert_eq!(plants.pots.len(), 25);
    assert_eq!(plants.rules.len(), 14);
}

#[test]
fn test_generation() {
    let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";
    let mut plants = Plants::from_input(input).unwrap();

    let expected_output: Vec<_> = "...#..#.#..##......###...###...........".chars().map(|c| c == '3').collect();

    plants.age();
    assert_eq!(plants.pots, expected_output);
}
