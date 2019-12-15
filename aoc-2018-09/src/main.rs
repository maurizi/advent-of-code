use std::collections::VecDeque;

fn rotate_left<T>(marbles: &mut VecDeque<T>, amount: usize) {
    (0..amount).for_each(|_| {
        let tmp = marbles.pop_back().unwrap();
        marbles.push_front(tmp);
    } );
}

fn rotate_right<T>(marbles: &mut VecDeque<T>, amount: usize) {
    (0..amount).for_each(|_| {
        let tmp = marbles.pop_front().unwrap();
        marbles.push_back(tmp);
    } );
}

fn part1(players: usize, last_marble: usize) -> usize {
    let mut marbles = VecDeque::with_capacity(last_marble);
    marbles.push_front(0usize);
    let mut player_scores = vec![0; players];

    for turn in 1 ..= last_marble {
        let player = (turn - 1) % players;
        if turn % 23 != 0 {
            rotate_left(&mut marbles,1);
            marbles.push_front(turn);
        } else {
            rotate_right(&mut marbles,7);
            player_scores[player] += turn;
            player_scores[player] += marbles.pop_front().unwrap();
            rotate_left(&mut marbles,1);
        }
    }
    player_scores.into_iter().max().unwrap()
}

fn main() {
    let players =452;
    let last_marble = 70784;
    println!("Top score: {}", part1(players, last_marble));
    println!("Top score: {}", part1(players, last_marble * 100));
}

#[test]
fn test_part1() {
    assert_eq!(part1(9, 25), 32);
    assert_eq!(part1(10, 1618), 8317);
    assert_eq!(part1(13, 7999), 146373);
    assert_eq!(part1(17, 1104), 2764);
    assert_eq!(part1(21, 6111), 54718);
    assert_eq!(part1(30, 5807), 37305);
}