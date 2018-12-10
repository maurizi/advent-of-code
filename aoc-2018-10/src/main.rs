#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate sprs;

use std::str::FromStr;

use regex::Regex;
use sprs::CsMat;
use std::num::ParseIntError;
use std::io;
use std::io::Read;
use std::cmp::Ordering;
use std::io::BufRead;

lazy_static! {
    static ref POINT: Regex = Regex::new(r"^position=< *([-0-9]+), *([-0-9]+)> velocity=< *([- 0-9]+), *([- 0-9]+)>$").unwrap();
}

struct Point {
    position: (isize, isize),
    velocity: (isize, isize),
}

#[derive(Debug, Copy, Clone)]
enum PointError {
    ParseIntError,
    RegexMatchError,
}

impl From<ParseIntError> for PointError {
    fn from(_: ParseIntError) -> Self {
        PointError::ParseIntError
    }
}

impl FromStr for Point {
    type Err = PointError;

    fn from_str(line: &str) -> Result<Self, PointError> {
        let captures = POINT.captures(&line).ok_or(PointError::RegexMatchError)?;
        Ok(Point {
            position: (captures[1].parse()?, captures[2].parse()?),
            velocity: (captures[3].parse()?, captures[4].parse()?),
        })
    }
}

fn parse(input: &str) -> Result<Vec<Point>, PointError> {
    input.lines().map(Point::from_str).collect()
}

fn move_points(points: &mut Vec<Point>) {
    points.into_iter().for_each(|p| {
        p.position.0 += p.velocity.0;
        p.position.1 += p.velocity.1;
    });
}

fn display_to_str(points: &Vec<Point>, bounds: &Bounds) -> String {
    let Bounds(min_x, min_y, max_x, max_y) = bounds.combine(&get_bounds(points));
    let (width, height) = (bounds.width(), bounds.height());
    let mut grid: CsMat<()> = CsMat::zero((height + 1, width + 1));

    let mut output = String::with_capacity(width *  height);
    for point in points {
        grid.insert((point.position.1 - min_y) as usize, (point.position.0 - min_x) as usize, ());
    }
    for y in 0 ..= max_y-min_y {
        for x in 0 ..= max_x-min_x {
            if grid.get(y as usize, x as usize).is_some() {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }
    output
}

struct Bounds(isize, isize, isize, isize);

impl Bounds {
    fn combine(&self, other: &Bounds) -> Bounds {
        Bounds(
            self.0.min(other.0), self.1.min(other.1),
            self.2.max(other.2), self.3.max(other.3)
        )
    }

    fn width(&self) -> usize {
        (self.2 - self.0) as usize
    }

    fn height(&self) -> usize {
        (self.3 - self.1) as usize
    }
}

fn get_bounds(points: &Vec<Point>) -> Bounds {
    let min_x = points.iter().map(|p| p.position.0).min().unwrap_or(0);
    let min_y = points.iter().map(|p| p.position.1).min().unwrap_or(0);
    let max_x = points.iter().map(|p| p.position.0).max().unwrap_or(0);
    let max_y = points.iter().map(|p| p.position.1).max().unwrap_or(0);
    Bounds(min_x, min_y, max_x, max_y)
}

fn part1(input: &str) -> String {
    let mut points = parse(input).unwrap();

    let stdin = io::stdin();
    let mut any_key = stdin.lock().lines();
    for i in 0 .. std::i32::MAX {
        let bounds = get_bounds(&points);
        let area = bounds.width() * bounds.height();

        if area < 10000 {
            println!("{}:\n\n{}", i, display_to_str(&points, &bounds));
            match any_key.next() {
                Some(Result::Ok(key)) => {
                    match key.as_str() {
                        "q" => std::process::exit(0),
                        _ => {},
                    }
                },
                _ => panic!()
            }
        }
        move_points(&mut points);
    };
    unreachable!();
}

fn main() {
    let input = include_str!("input.txt");
    part1(input);
}

#[test]
fn test_part1() {
    let input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
";
    let output = [
"........#.............
................#.....
.........#.#..#.......
......................
#..........#.#.......#
...............#......
....#.................
..#.#....#............
.......#..............
......#...............
...#...#.#...#........
....#..#..#.........#.
.......#..............
...........#..#.......
#...........#.........
...#.......#..........
",
"......................
......................
..........#....#......
........#.....#.......
..#.........#......#..
......................
......#...............
....##.........#......
......#.#.............
.....##.##..#.........
........#.#...........
........#...#.....#...
..#...........#.......
....#.....#.#.........
......................
......................
",
"......................
......................
......................
..............#.......
....#..#...####..#....
......................
........#....#........
......#.#.............
.......#...#..........
.......#..#..#.#......
....#....#.#..........
.....#...#...##.#.....
........#.............
......................
......................
......................
",
"......................
......................
......................
......................
......#...#..###......
......#...#...#.......
......#...#...#.......
......#####...#.......
......#...#...#.......
......#...#...#.......
......#...#...#.......
......#...#..###......
......................
......................
......................
......................
",
"......................
......................
......................
............#.........
........##...#.#......
......#.....#..#......
.....#..##.##.#.......
.......##.#....#......
...........#....#.....
..............#.......
....#......#...#......
.....#.....##.........
...............#......
...............#......
......................
......................
"];

    for (times, output) in output.into_iter().enumerate() {
        let mut points = parse(input);
        let mut points = points.as_mut().unwrap_or_else(|_| panic!(""));
        let mut bounds = get_bounds(&points);
        for _ in 0..times {
            move_points(&mut points);
        }
        let actual = display_to_str(&points, &bounds);
        println!("Actual:\n{}", actual);
        assert_eq!(actual, *output);
    }
}