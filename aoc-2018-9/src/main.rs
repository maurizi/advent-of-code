fn part1(players: usize, last_marble: usize) -> usize {
    let mut marbles = Vec::with_capacity(last_marble);
    marbles.push(0usize);
    let mut player_scores = vec![0; players];

    for turn in 1 ..= last_marble {
        let player = (turn - 1) % players;
        if turn % 23 != 0 {
            marbles.rotate_left(1);
            marbles.push(turn);
        } else {
            marbles.rotate_right(7);
            player_scores[player] += turn;
            player_scores[player] += marbles.pop().unwrap();
            marbles.rotate_left(1);
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