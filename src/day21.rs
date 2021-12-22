mod utils;

fn main() {
    // let input = _get_test_input();
    let input = _get_input();

    // ----------- Parse Input -----------

    let starts = input
        .trim()
        .split("\n")
        .map(|s| s.split_once(": ").unwrap().1)
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // ----------- Solve -----------

    let add = |n: usize, i: usize| (n + i - 1) % 10 + 1;
    let sub = |n: usize, i: usize| (n + (10 - i) - 1) % 10 + 1;

    let mut positions = starts.clone();
    let mut scores = starts.iter().map(|_| 0).collect::<Vec<usize>>();
    let mut player = 0;
    let mut dice_val = 1;
    let mut num_rolls = 0;
    loop {
        for _ in 0..3 {
            num_rolls += 1;
            positions[player] = add(positions[player], dice_val);
            dice_val = (dice_val % 100) + 1;
        }
        scores[player] += positions[player];
        if scores[player] >= 1000 {
            break;
        }

        player = (player + 1) % positions.len();
    }

    let p1 = scores.iter().min().unwrap() * num_rolls;

    let mut universes = [[[[0 as u64; 11]; 31]; 13]; 2]; // universes[player][moves][points][position]

    universes[0][0][0][starts[0]] = 1;
    universes[1][0][0][starts[1]] = 1;

    for player in 0..2 {
        for moves in 1..=12 {
            for points in 0..31 {
                for position in 1..=10 {
                    let mut num_possibilities = 0;
                    if points < position {
                        continue;
                    }
                    for dice_1 in 1..=3 {
                        for dice_2 in 1..=3 {
                            for dice_3 in 1..=3 {
                                num_possibilities += universes[player][moves - 1]
                                    [points - position][sub(position, dice_1 + dice_2 + dice_3)];
                            }
                        }
                    }
                    universes[player][moves][points][position] = num_possibilities;
                }
            }
        }
    }

    let mut wins_after_points = [[0; 22]; 2];
    for points_to_win in 1..=21 {
        let mut num_possibilities = (0, 0);
        for moves in 1..=12 {
            let mut wins_at_move = (0, 0);
            let mut loses_at_move = (0, 0);

            for points in 0..31 {
                for position in 1..=10 {
                    if points >= points_to_win && points < points_to_win + position {
                        wins_at_move.0 += universes[0][moves][points][position];
                        wins_at_move.1 += universes[1][moves][points][position];
                    }
                    if points < points_to_win {
                        loses_at_move.0 += universes[0][moves][points][position];
                        loses_at_move.1 += universes[1][moves - 1][points][position];
                    }
                }
            }
            num_possibilities.0 += wins_at_move.0 * loses_at_move.1;
            num_possibilities.1 += wins_at_move.1 * loses_at_move.0;
        }

        wins_after_points[0][points_to_win] = num_possibilities.0;
        wins_after_points[1][points_to_win] = num_possibilities.1;
    }

    let p2 = wins_after_points.iter().map(|w| w[21]).max().unwrap();

    // ----------- Print -----------

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn _get_test_input() -> String {
    return "

Player 1 starting position: 4
Player 2 starting position: 8



"
    .to_string();
}

fn _get_input() -> String {
    return "


Player 1 starting position: 8
Player 2 starting position: 4




"
    .to_string();
}
