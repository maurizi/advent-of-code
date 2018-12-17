use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Write;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Track {
    Vertical, Horizontal, ForwardCurve, BackwardCurve, Intersection
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Dir {
    Up, Down, Left, Right
}

impl Dir {
    fn left_from(self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Right => Dir::Up,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
        }
    }

    fn right_from(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Left => Dir::Up,
            Dir::Down => Dir::Left,
            Dir::Right => Dir::Down,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Turn {
    Straight, Left, Right
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Cart {
    direction: Dir,
    next_turn: Turn,
}

impl Cart {
    fn new(direction: Dir) -> Cart {
        Cart { direction, next_turn: Turn::Left }
    }
}

#[derive(Eq, PartialEq, Clone)]
enum Item {
    Track(Track),
    TrackWithCrash(Track),
    TrackWithCart(Track, Cart),
}

impl Item {
    fn remove_cart(self) -> (Track, Option<Cart>) {
        match self {
            Item::Track(track) | Item::TrackWithCrash(track) => (track, None),
            Item::TrackWithCart(track, cart) => {
                (track, Some(cart))
            }
        }
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_char(match self {
            Item::Track(Track::ForwardCurve) => '/',
            Item::Track(Track::BackwardCurve) => '\\',
            Item::Track(Track::Vertical) => '|',
            Item::Track(Track::Horizontal) => '-',
            Item::Track(Track::Intersection) => '+',
            Item::TrackWithCart(_, Cart{ direction: Dir::Down, .. }) => 'v',
            Item::TrackWithCart(_, Cart{ direction: Dir::Up, .. }) => '^',
            Item::TrackWithCart(_, Cart{ direction: Dir::Right, .. }) => '>',
            Item::TrackWithCart(_, Cart{ direction: Dir::Left, .. }) => '<',
            Item::TrackWithCrash( .. ) => 'X',
        })
    }
}

#[derive(Eq, PartialEq)]
struct Tracks {
    grid: Vec<Vec<Option<Item>>>,
    carts: Vec<(usize, usize)>

}

impl Debug for Tracks {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in self.grid.iter() {
            for cell in row.iter() {
                if let Some(item) = cell {
                    item.fmt(f)?;
                } else {
                    f.write_char(' ')?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}


fn parse(input: &str) -> Tracks {
    let mut carts = vec![];
    let grid = input.lines().enumerate().map(|(y, line)| {
        line.bytes().enumerate().map(|(x, b)| {
            let (item, cart) = match b as char {
                '/' => (Some(Item::Track(Track::ForwardCurve)), false),
                '\\' => (Some(Item::Track(Track::BackwardCurve)), false),
                '|' => (Some(Item::Track(Track::Vertical)), false),
                '-' => (Some(Item::Track(Track::Horizontal)), false),
                '+' => (Some(Item::Track(Track::Intersection)), false),
                'v' => (Some(Item::TrackWithCart(Track::Vertical, Cart::new(Dir::Down))), true),
                '^' => (Some(Item::TrackWithCart(Track::Vertical, Cart::new(Dir::Up))), true),
                '>' => (Some(Item::TrackWithCart(Track::Horizontal, Cart::new(Dir::Right))), true),
                '<' => (Some(Item::TrackWithCart(Track::Horizontal, Cart::new(Dir::Left))), true),
                _ => (None, false)
            };
            if cart {
                carts.push((x, y));
            }
            item
        }).collect()
    }).collect();
    Tracks { grid, carts }
}

fn tick(tracks: &mut Tracks) -> Vec<(usize, usize)> {
    let mut new_carts = Vec::with_capacity(tracks.carts.len());
    let mut crashes = vec![];

    tracks.carts.sort();
    for (x, y) in tracks.carts.iter().cloned() {
        // If we already crashed into the car, don't move it
        if crashes.contains(&(x, y)) {
            continue;
        }

        let (cur_track, cart) = tracks.grid[y][x].take().unwrap().remove_cart();
        tracks.grid[y][x] = Some(Item::Track(cur_track));

        let mut cart = cart.unwrap();
        let (new_x, new_y) = match cart.direction {
            Dir::Up => (x, y - 1),
            Dir::Down => (x, y + 1),
            Dir::Left => (x - 1, y),
            Dir::Right => (x + 1, y),
        };

        let next_item = tracks.grid[new_y][new_x].clone().unwrap();

        match next_item {
            Item::TrackWithCart(next_track, ..) | Item::TrackWithCrash(next_track) => {
                tracks.grid[new_y][new_x] = Some(Item::TrackWithCrash(next_track));
                crashes.push((new_x, new_y));
            },
            Item::Track(next_track) => {
                cart.direction = match (next_track, cart.direction) {
                    (Track::Vertical, _) | (Track::Horizontal, _) => cart.direction,
                    (Track::ForwardCurve, Dir::Up) => Dir::Right,
                    (Track::ForwardCurve, Dir::Down) => Dir::Left,
                    (Track::ForwardCurve, Dir::Left) => Dir::Down,
                    (Track::ForwardCurve, Dir::Right) => Dir::Up,
                    (Track::BackwardCurve, Dir::Up) => Dir::Left,
                    (Track::BackwardCurve, Dir::Down) => Dir::Right,
                    (Track::BackwardCurve, Dir::Left) => Dir::Up,
                    (Track::BackwardCurve, Dir::Right) => Dir::Down,
                    (Track::Intersection, _) => {
                        let (next_turn, dir) = match cart.next_turn {
                            Turn::Left => (Turn::Straight, cart.direction.left_from()),
                            Turn::Straight => (Turn::Right, cart.direction),
                            Turn::Right => (Turn::Left, cart.direction.right_from()),
                        };
                        cart.next_turn = next_turn;
                        dir
                    }
                };
                tracks.grid[new_y][new_x] = Some(Item::TrackWithCart(next_track, cart));
                new_carts.push((new_x, new_y));
            },
        }
    }
    tracks.carts = new_carts.into_iter().filter(|coords| !crashes.contains(coords)).collect();
    crashes
}

fn part1(tracks: &mut Tracks) -> Vec<(usize, usize)> {
    loop {
        let coordinates = tick(tracks);
        if coordinates.len() > 0 {
            return coordinates;
        }
    }
}

fn part2(tracks: &mut Tracks) -> (usize, usize) {
    while tracks.carts.len() > 1 {
        let crashes = part1(tracks);
        for (x, y) in crashes {
            let (track, _) = tracks.grid[y][x].take().unwrap().remove_cart();
            tracks.grid[y][x] = Some(Item::Track(track));
        }
    }
    tracks.carts[0]
}

fn main() {
    let mut tracks = parse(include_str!("input.txt"));
    println!("part 1 {:?}", part1(&mut tracks));
    let mut tracks = parse(include_str!("input.txt"));
    println!("part 2 {:?}", part2(&mut tracks));
}

#[test]
fn test_tick() {
    let input = r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/"#;

    let output_3 = r#"/---\
|   v  /----\
| /-+--+-\  |
| | |  | |  |
\-+-/  \-+->/
  \------/   "#;

    let expected = parse(output_3);
    let mut tracks = parse(input);
    for _ in 0..3 {
        tick(&mut tracks);
    }

    println!("actual:\n{:?}", tracks);
    println!("expected:\n{:?}", expected);
    assert_eq!(format!("{:?}", tracks), format!("{:?}", expected));
}

#[test]
fn test_part1() {
    let input = r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/"#;

    assert_eq!(part1(&mut parse(input)), vec![(7, 3)])
}

#[test]
fn test_part2() {
    let input = r#"/>-<\
|   |
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;

    assert_eq!(part2(&mut parse(input)), (6, 4))
}
