mod utils;
use std::collections::HashMap;

fn main() {
    let input = _get_input();

    // ----------- Parse Input -----------
    let parsed = input.trim().split('\n').collect::<Vec<&str>>();
    let mut groups: Vec<(u64, i64, u64)> = vec![];
    for input_i in (0..parsed.len()).step_by(18) {
        let values: Vec<i64> = [4, 5, 15].iter().map(|i| parsed[*i + input_i][6..].parse::<i64>().unwrap()).collect();
        groups.push((values[0] as u64, values[1], values[2] as u64));
    }

    // ----------- Solve -----------

    let mut cache: HashMap<(usize, u64, u64), u64> = HashMap::new();
    fn compute_group(group: &(u64, i64, u64), w: u64, prev_z: u64) -> u64 {
        let (a, b, c) = group;

        let mut z = prev_z / a;
        if w as i64 != b + (prev_z % 26) as i64 {
            z = z * 26;
            z += w + c;
        }

        z
    }
    fn compute_best(find_max: bool, groups: &Vec<(u64, i64, u64)>, cache: &mut HashMap<(usize, u64, u64), u64>, group: usize, prev_z: u64) -> Option<u64> {
        let (best, worst) = if find_max { (9, 1) } else { (1, 9) };
        for w in utils::iter_nums(best, worst) {
            if let Some(next_z) = cache.get(&(group, w, prev_z)) {
                if *next_z == 0 {
                    return Some(*next_z);
                } else {
                    return None;
                }
            }

            let z = compute_group(&groups[group], w, prev_z);
            cache.insert((group, w, prev_z), z);
            if group == groups.len() - 1 && z == 0 {
                return Some(w);
            } else if group < groups.len() - 1 {
                if let Some(best_rest) = compute_best(find_max, groups, cache, group + 1, z) {
                    return Some(utils::prepend_digit(w, best_rest));
                }
            }
        }
        return None;
    }

    // ----------- Print -----------

    println!(
        "Part 1: {}",
        compute_best(true, &groups, &mut cache, 0, 0).unwrap()
    );
    println!(
        "Part 2: {}",
        compute_best(false, &groups, &mut cache, 0, 0).unwrap()
    );
}

fn _get_test_input() -> String {
    return "



inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y


"
    .to_string();
}

fn _get_input() -> String {
    return "



inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 11
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -1
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x 0
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -5
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w    
mul x 0
add x z
mod x 26
div z 26
add x -16
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y  
inp w
mul x 0
add x z
mod x 26
div z 26
add x -7
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 11
mul y x
add z y        
inp w
mul x 0
add x z
mod x 26
div z 26
add x -11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y









"
    .to_string();
}
