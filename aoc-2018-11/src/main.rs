#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct Cell(usize, usize);

impl Cell {
    fn power_level(&self, grid_serial: isize) -> isize {
        let (x, y) = (self.0 as isize, self.1 as isize);
        let rack_id = x + 10;
        let mut power_level: isize = rack_id * y;
        power_level += grid_serial;
        power_level *= rack_id;
        let digits: Vec<u32> = power_level.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect();
        power_level = if digits.len() > 2 {
            digits[digits.len() - 3] as isize
        } else {
            0
        };
        power_level - 5
    }
}

fn get_power_levels(grid_serial: isize) -> [[isize; 300]; 300] {
    let mut grid = [[0; 300]; 300];
    for x in 1..=300 {
        for y in 1..=300 {
            grid[x-1][y-1] = Cell(x, y).power_level(grid_serial);
        }
    }
    grid
}

fn square_power_level(cell: Cell, power_levels: &[[isize; 300]; 300], size: usize) -> isize {
    let mut sum = 0;
    for x in cell.0 .. cell.0 + size {
        for y in cell.1 .. cell.1 + size {
            sum += power_levels[x-1][y-1];
        }
    }
    sum
}

fn part1(grid_serial: isize) -> Cell {
    let power_levels = get_power_levels(grid_serial);
    let mut max_cell = Cell(0, 0);
    let mut max = std::isize::MIN;
    for x in 1..=297 {
        for y in 1..=297 {
            let cell = Cell(x, y);
            let power_level = square_power_level(cell, &power_levels, 3);
            if power_level > max {
                max_cell = cell;
                max = power_level;
            }
        }
    }
    max_cell
}

fn part2(grid_serial: isize) -> (Cell, usize) {
    let power_levels = get_power_levels(grid_serial);
    let mut max_cell = Cell(0, 0);
    let mut max_size = 0;
    let mut max = std::isize::MIN;
    for size in 1..=300 {
        for x in 1..=300-size {
            for y in 1..=300-size {
                let cell = Cell(x, y);
                let power_level = square_power_level(cell, &power_levels, size);
                if power_level > max {
                    max_cell = cell;
                    max_size = size;
                    max = power_level;
                }
            }
        }
    }
    (max_cell, max_size)
}

fn main() {
    println!("{:?}", part1(5093));
    println!("{:?}", part2(5093));
}

#[test]
fn test_power_level() {
    assert_eq!(Cell(3, 5).power_level(8), 4);
    assert_eq!(Cell(122, 79).power_level(57), -5);
    assert_eq!(Cell(217, 196).power_level(39), 0);
    assert_eq!(Cell(101, 153).power_level(71), 4);
}

#[test]
fn test_part1() {
    assert_eq!(part1(18), Cell(33, 45));
    assert_eq!(part1(42), Cell(21, 61));
}

#[test]
fn test_part2() {
    assert_eq!(part2(18), (Cell(90, 269), 16));
    assert_eq!(part2(42), (Cell(232, 251), 12));
}
