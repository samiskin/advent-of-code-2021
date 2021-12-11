mod utils;
use utils::Grid;
use std::collections::HashSet;

fn main() {
    // let input = _get_test_input();
    let input = _get_input();

    // ----------- Parse Input -----------

    let lines = input
        .trim()
        .split('\n')
        .map(|s| s.trim().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let mut grid = Grid::new(lines);

    // ----------- Solve -----------

    // println!("{}: {:?}", 0, grid);
    let mut p1 = 0;
    let mut p2 = 0;
    for step in 0..1000 {
        let mut coords_to_inc = grid.iter().map(|(pos, _)| pos).collect::<Vec<(usize, usize)>>();
        let mut flashed = HashSet::new();
        while !coords_to_inc.is_empty() {
            let (x, y) = coords_to_inc.remove(0);
            if flashed.contains(&(x, y)) { continue };
            let val = *grid.get(x, y);
            if val == 9 {
                if step <= 100 {
                    p1 += 1;
                }
                flashed.insert((x, y));
                grid.set(x, y, 0);
                for neighbor in grid.neighbors_diag(x, y).iter().filter(|npos| !flashed.contains(&npos)) {
                    coords_to_inc.push(*neighbor);
                }
            } else {
                grid.set(x, y, val + 1);
            }
        }
        if p2 == 0 && flashed.len() == grid.size() {
            p2 = step;
            if step > 100 {
                break;
            }
        }
    }

    // ----------- Print -----------

    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
}

fn _get_test_input() -> String {
    return "

5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526

"
    .to_string();
}

fn _get_input() -> String {
    return "

3113284886
2851876144
2774664484
6715112578
7146272153
6256656367
3148666245
3857446528
7322422833
8152175168

"
    .to_string();
}
