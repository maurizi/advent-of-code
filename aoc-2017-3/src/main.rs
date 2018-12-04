extern crate itertools;

use itertools::Itertools;

enum Dir {
    Up, Down, Left, Right
}

fn steps_back(step: i32) -> i32 {
    let mut up = 0i32;
    let mut right = 0i32;
    let mut size = 1;
    let mut cur_step = 1;
    let mut dir = Dir::Right;
    let mut stride = 1;

    while cur_step < step {
        match &dir {
            Dir::Right => right += 1,
            Dir::Left => right -= 1,
            Dir::Up => up += 1,
            Dir::Down => up -= 1,
        }
        if stride % size == 0 {
            dir = match &dir {
                Dir::Right => Dir::Up,
                Dir::Left => Dir::Down,
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
            };
            if stride > size {
                stride = 0;
                size += 1;
            }
        }
        cur_step += 1;
        stride += 1;
    }

    up.abs() + right.abs()
}

fn sum_larger_than(limit: i32) -> i32 {
    let mut grid = vec![vec![0; 1000]; 1000];
    let mut x = 500;
    let mut y = 500;
    let mut size = 1;
    let mut dir = Dir::Right;
    let mut stride = 1;

    grid[x][y] = 1;

    while grid[x][y] <= limit {
        match &dir {
            Dir::Right => x += 1,
            Dir::Left => x -= 1,
            Dir::Up => y += 1,
            Dir::Down => y -= 1,
        }
        let dirs: [i32; 3] = [-1, 0, 1];
        let pairs = dirs.iter().cartesian_product(dirs.iter());
        let sum: i32 = pairs.map(|(i, j)| {
            let _x = ((x as i64) + (*i as i64)) as usize;
            let _y = ((y as i64) + (*j as i64)) as usize;
            grid[_x][_y]
        }).sum();
        grid[x][y] = sum;

        if stride % size == 0 {
            dir = match &dir {
                Dir::Right => Dir::Up,
                Dir::Left => Dir::Down,
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
            };
            if stride > size {
                stride = 0;
                size += 1;
            }
        }
        stride += 1;
    }

    grid[x][y]
}

fn main() {
    println!("{}", steps_back(312051));
    println!("{}", sum_larger_than(312051));
}

#[test]
fn test_steps_back() {
    assert_eq!(steps_back(1), 0);
    assert_eq!(steps_back(12), 3);
    assert_eq!(steps_back(23), 2);
    assert_eq!(steps_back(1024), 31);
}

#[test]
fn test_sum_adjacent() {
    assert_eq!(sum_larger_than(1), 2);
    assert_eq!(sum_larger_than(2), 4);
    assert_eq!(sum_larger_than(3), 4);
    assert_eq!(sum_larger_than(4), 5);
    assert_eq!(sum_larger_than(5), 10);
    assert_eq!(sum_larger_than(10), 11);
}