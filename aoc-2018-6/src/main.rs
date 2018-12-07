extern crate itertools;

use itertools::Itertools;

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
    grid
        .into_iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row
                .into_iter()
                .filter_map(|x| x)
                .enumerate()
                .map(|(col_idx, cell): (usize, usize)| {
                    let row_copy = row_idx.clone();
                    (row_copy, col_idx, cell)
                })
        })
        .filter_map(|(row, col, cell)| {
            if col != 0 && col != GRID_SIZE - 1 && row != 0 && row != GRID_SIZE - 1 {
                Some(cell)
            } else {
                None
            }
        })
        .sorted()
        .into_iter()
        .group_by(|cell: &usize| *cell)
        .into_iter()
        .map(|(_, group)| group.count())
        .max()
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_part1() {
    let input = "1, 1
1, 6
8,3
3, 4
5,5
8, 9";
    assert_eq!(part1(input), Some(17));
}
