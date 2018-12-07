extern crate itertools;

use itertools::Itertools;
use std::collections::HashSet;

static GRID_SIZE: usize = 500;

fn part1(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<Option<usize>>> = vec![vec![None; GRID_SIZE]; GRID_SIZE];
    let points: Vec<(usize, usize)> = input.lines().map(|line| {
        let nums: Vec<&str> = line.split(", ").collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }).collect();

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            grid[i][j] = points
                .iter()
                .enumerate()
                .min_by_key(|&(_, &point)| {
                    let (x, y) = point;
                    (i as i64 - x as i64).abs() + (j as i64 - y as i64).abs()
                })
                .map(|(index, _)| index);
        }
    }

    let mut infinite = HashSet::new();
    for i in 0..GRID_SIZE {
        let points = [(0, i), (i, 0), (GRID_SIZE - 1, i), (i, GRID_SIZE - 1)];
        for (x, y) in points.iter() {
            if let Some(cell) = grid[*x][*y] {
                infinite.insert(cell);
            }
        }
    }

    grid
        .iter()
        .flat_map(|f| f.iter())
        .filter_map(|&x| x)
        .filter(|index| !infinite.contains(index))
        .sorted()
        .into_iter()
        .group_by(|cell: &usize| *cell)
        .into_iter()
        .map(|(_, group)| group.count())
        .max()
}

fn part2(input: &str, threshold: i64) -> Option<usize> {
    let mut grid: Vec<Vec<i64>> = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    let points: Vec<(usize, usize)> = input.lines().map(|line| {
        let nums: Vec<&str> = line.split(", ").collect();
        (nums[0].parse().unwrap(), nums[1].parse().unwrap())
    }).collect();

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {

            grid[i][j] = points
                .iter()
                .map(|&point| {
                    let (x, y) = point;
                    (i as i64 - x as i64).abs() + (j as i64 - y as i64).abs()
                })
                .sum();
        }
    }

    Some(grid.iter().flat_map(|f| f.iter()).filter(|cell| **cell <threshold).count())
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input).unwrap());
    println!("{}", part2(input, 10000).unwrap());
}

#[test]
fn test_part1() {
    let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
    assert_eq!(part1(input), Some(17));
}

#[test]
fn test_part2() {
    let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
    assert_eq!(part2(input, 32), Some(16));
}
