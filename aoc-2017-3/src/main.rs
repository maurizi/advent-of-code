enum Dir {
    Up, Down, Left, Right
}

fn steps_back(step: i32) -> i32 {
    if (step == 0) {
        return 0;
    }

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

fn main() {
    println!("{}", steps_back(312051));
}

#[test]
fn test_steps_back() {
    assert_eq!(steps_back(1), 0);
    assert_eq!(steps_back(12), 3);
    assert_eq!(steps_back(23), 2);
    assert_eq!(steps_back(1024), 31);
}
