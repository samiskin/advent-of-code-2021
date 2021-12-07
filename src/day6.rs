fn main() {
    // let input = _get_test_input();
    let input = _get_input();

    // ----------- Parse Input -----------

    let lines = input
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // ----------- Solve -----------

    fn solve_fishes(init: &Vec<i32>, days: usize) -> usize {
        let mut new_fishes_per_day = vec![0; days + 1];
        for starting_fish in init {
            for birth in ((*starting_fish as usize)..(new_fishes_per_day.len() - 1)).step_by(7) {
                new_fishes_per_day[birth + 1] += 1;
            }
        }

        for day in 0..new_fishes_per_day.len() {
            for birth in ((day + 9)..new_fishes_per_day.len()).step_by(7) {
                new_fishes_per_day[birth] += new_fishes_per_day[day];
            }
        }
        return new_fishes_per_day.iter().fold(0, |a, f| a + f) + init.len();
    }


    // ----------- Print -----------

    println!("Part 1: {:?}", solve_fishes(&lines, 80));
    println!("Part 2: {:?}", solve_fishes(&lines, 256));
}

fn _get_test_input() -> String {
    return "
3,4,3,1,2
"
    .to_string();
}

fn _get_input() -> String {
    return "
1,3,4,1,5,2,1,1,1,1,5,1,5,1,1,1,1,3,1,1,1,1,1,1,1,2,1,5,1,1,1,1,1,4,4,1,1,4,1,1,2,3,1,5,1,4,1,2,4,1,1,1,1,1,1,1,1,2,5,3,3,5,1,1,1,1,4,1,1,3,1,1,1,2,3,4,1,1,5,1,1,1,1,1,2,1,3,1,3,1,2,5,1,1,1,1,5,1,5,5,1,1,1,1,3,4,4,4,1,5,1,1,4,4,1,1,1,1,3,1,1,1,1,1,1,3,2,1,4,1,1,4,1,5,5,1,2,2,1,5,4,2,1,1,5,1,5,1,3,1,1,1,1,1,4,1,2,1,1,5,1,1,4,1,4,5,3,5,5,1,2,1,1,1,1,1,3,5,1,2,1,2,1,3,1,1,1,1,1,4,5,4,1,3,3,1,1,1,1,1,1,1,1,1,5,1,1,1,5,1,1,4,1,5,2,4,1,1,1,2,1,1,4,4,1,2,1,1,1,1,5,3,1,1,1,1,4,1,4,1,1,1,1,1,1,3,1,1,2,1,1,1,1,1,2,1,1,1,1,1,1,1,2,1,1,1,1,1,1,4,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,2,5,1,2,1,1,1,1,1,1,1,1,1
"
    .to_string();
}
