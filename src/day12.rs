mod utils;
use std::collections::HashSet;
use utils::Graph;

fn main() {
    let input = _get_test_input();
    // let input = _get_input();

    // ----------- Parse Input -----------

    let lines = input
        .trim()
        .split('\n')
        .map(|s| s.trim().split('-').collect())
        .collect::<Vec<Vec<&str>>>();

    // ----------- Solve -----------

    let mut graph = Graph::new();
    for line in lines {
        let a = line[0];
        let b = line[1];
        graph.add_edge(a, b);
    }

    println!("{:?}", graph);

    fn explore<'a>(
        is_p2: bool,
        g: &Graph<&str>,
        start: &'a str,
        mut seen: HashSet<&'a str>,
        extra_small: &str,
        mut path: Vec<&'a str>,
    ) -> u32 {
        path.push(start);
        if start == "end" {
            if !extra_small.is_empty() && !seen.contains(extra_small) {
                return 0;
            }
            return 1;
        }
        let is_small = start.chars().all(|c| c.is_lowercase());
        if is_small {
            seen.insert(start);
        }

        return g
            .neighbors(&start)
            .iter()
            .filter(|n| !seen.contains(**n))
            .fold(0, |mut a, n| {
                if is_p2 && is_small && extra_small == "" && start != "start" {
                    let mut seen_without = seen.to_owned();
                    seen_without.remove(start);
                    a += explore(is_p2, g, n, seen_without, start, path.to_owned());
                }
                a + explore(is_p2, g, n, seen.to_owned(), extra_small, path.to_owned())
            });
    }

    let p1 = explore(false, &graph, "start", HashSet::new(), "", vec![]);
    let p2 = explore(true, &graph, "start", HashSet::new(), "", vec![]);

    // ----------- Print -----------

    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
}

fn _get_test_input() -> String {
    return "

start-A
start-b
A-c
A-b
b-d
A-end
b-end

"
    .to_string();
}

fn _get_input() -> String {
    return "

cz-end
cz-WR
TD-end
TD-cz
start-UM
end-pz
kb-UM
mj-UM
cz-kb
WR-start
WR-pz
kb-WR
TD-kb
mj-kb
TD-pz
UM-pz
kb-start
pz-mj
WX-cz
sp-WR
mj-WR

"
    .to_string();
}
