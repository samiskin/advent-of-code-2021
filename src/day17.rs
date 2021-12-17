mod utils;

fn main() {
    let input = _get_test_input();
    // let input = _get_input();

    // ----------- Parse Input -----------

    let parsed: Vec<(i32, i32)> = input
        .trim()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(", ")
        .map(|s| s.split_once("=").unwrap().1.split_once("..").unwrap())
        .map(|(left, right)| (left.parse().unwrap(), right.parse().unwrap()))
        .collect();
    let (x_range, y_range) = (parsed[0], parsed[1]);

    // ----------- Solve -----------

    let in_area = |pos: &(i32, i32)| {
        pos.0 >= x_range.0 && pos.0 <= x_range.1 && pos.1 >= y_range.0 && pos.1 <= y_range.1
    };

    let mut max_y = 0;
    let mut num_shots = 0;

    for x_vel in 0..1000 {
        for y_vel in -1000..1000 {
            let mut probe_pos = (0, 0);
            let mut vel = (x_vel, y_vel);
            let mut max_probe_y = 0;
            while probe_pos.1 >= y_range.0 && probe_pos.0 <= x_range.1 {
                probe_pos.0 += vel.0;
                probe_pos.1 += vel.1;
                vel.0 -= if vel.0 > 0 {
                    1
                } else if vel.0 < 0 {
                    -1
                } else {
                    0
                };
                vel.1 -= 1;
                max_probe_y = i32::max(max_probe_y, probe_pos.1);

                if in_area(&probe_pos) {
                    max_y = i32::max(max_y, max_probe_y);
                    num_shots += 1;
                    break;
                }
            }
        }
    }

    let p1 = max_y;
    let p2 = num_shots;

    // ----------- Print -----------

    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
}

fn _get_test_input() -> String {
    return "

target area: x=20..30, y=-10..-5


"
    .to_string();
}

fn _get_input() -> String {
    return "

target area: x=179..201, y=-109..-63





"
    .to_string();
}
