mod utils;
use std::fmt;

#[derive(Debug, Clone)]
enum PairVal {
    Number(usize),
    Pair(Vec<Pair>),
}

#[derive(Debug, Clone)]
struct Pair {
    val: PairVal,
    depth: usize,
}

impl Pair {
    fn add(&self, rhs: &Pair) -> Pair {
        let mut left = self.clone();
        left.inc_depth();
        let mut right = rhs.clone();
        right.inc_depth();

        let mut merged = Pair {
            val: PairVal::Pair(vec![left, right]),
            depth: 0,
        };

        loop {
            if !merged.explode().is_some() && !merged.split().is_some() {
                break;
            }
        }

        return merged;
    }

    fn inc_depth(&mut self) {
        self.depth += 1;
        match &mut self.val {
            PairVal::Pair(pair) => {
                pair[0].inc_depth();
                pair[1].inc_depth();
            }
            _ => {}
        };
    }

    fn add_from_left(&mut self, amt: usize) {
        match &mut self.val {
            PairVal::Number(n) => {
                *n += amt;
            }
            PairVal::Pair(pair) => {
                pair[0].add_from_left(amt);
            }
        }
    }

    fn add_from_right(&mut self, amt: usize) {
        match &mut self.val {
            PairVal::Number(n) => {
                *n += amt;
            }
            PairVal::Pair(pair) => {
                pair[1].add_from_right(amt);
            }
        }
    }

    fn explode(&mut self) -> Option<(usize, usize)> {
        if self.depth >= 4 {
            match &self.val {
                PairVal::Number(_) => None,
                PairVal::Pair(pair) => {
                    let result = match &pair[0].val {
                        PairVal::Number(left) => match &pair[1].val {
                            PairVal::Number(right) => Some((*left, *right)),
                            _ => None,
                        },
                        _ => None,
                    };
                    self.val = PairVal::Number(0);
                    result
                }
            }
        } else {
            match &mut self.val {
                PairVal::Number(_) => None,
                PairVal::Pair(pair) => match pair[0].explode() {
                    Some((carry_l, carry_r)) => {
                        pair[1].add_from_left(carry_r);
                        Some((carry_l, 0))
                    }
                    _ => match pair[1].explode() {
                        Some((carry_l, carry_r)) => {
                            pair[0].add_from_right(carry_l);
                            Some((0, carry_r))
                        }
                        _ => None,
                    },
                },
            }
        }
    }

    fn split(&mut self) -> Option<()> {
        match &mut self.val {
            PairVal::Number(num) => {
                if *num >= 10 {
                    let avg = (*num as f64) / 2.0;
                    self.val = PairVal::Pair(vec![
                        Pair {
                            val: PairVal::Number(avg.floor() as usize),
                            depth: self.depth + 1,
                        },
                        Pair {
                            val: PairVal::Number(avg.ceil() as usize),
                            depth: self.depth + 1,
                        },
                    ]);
                    Some(())
                } else {
                    None
                }
            }
            PairVal::Pair(pair) => {
                if pair[0].split().is_some() || pair[1].split().is_some() {
                    Some(())
                } else {
                    None
                }
            }
        }
    }

    fn magnitude(&self) -> usize {
        match &self.val {
            PairVal::Number(_) => panic!("Cannot get magnitude for number"),
            PairVal::Pair(pair) => {
                let left_mag = match pair[0].val {
                    PairVal::Number(n) => n,
                    PairVal::Pair(_) => pair[0].magnitude(),
                };
                let right_mag = match pair[1].val {
                    PairVal::Number(n) => n,
                    PairVal::Pair(_) => pair[1].magnitude(),
                };

                3 * left_mag + 2 * right_mag
            }
        }
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.val {
            PairVal::Number(n) => {
                write!(f, "{}", n).unwrap();
            }
            PairVal::Pair(pair) => {
                write!(f, "[{},{}]", pair[0], pair[1]).unwrap();
            }
        };
        Ok(())
    }
}

fn main() {
    // let input = _get_test_input();
    let input = _get_input();

    // ----------- Parse Input -----------

    fn parse(s: &str, i: &mut usize, depth: usize) -> Pair {
        if s.chars().nth(*i).unwrap() != '[' {
            return Pair {
                val: PairVal::Number(
                    s.chars()
                        .skip(*i)
                        .take_while(|c| c.is_digit(10))
                        .map(|c| {
                            *i += 1;
                            c
                        })
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap(),
                ),
                depth,
            };
        }

        *i += 1;
        let left = parse(s, i, depth + 1);
        *i += 1;
        let right = parse(s, i, depth + 1);
        *i += 1;

        return Pair {
            val: PairVal::Pair(vec![left, right]),
            depth,
        };
    }

    let parsed: Vec<Pair> = input
        .trim()
        .split('\n')
        .map(|s| s.trim())
        .map(|s| {
            let mut i = 0;
            parse(s, &mut i, 0)
        })
        .collect();

    // ----------- Solve -----------
    let p1 = parsed
        .clone()
        .into_iter()
        .reduce(|acc, pair| acc.add(&pair))
        .unwrap()
        .magnitude();

    let p2 = parsed
        .iter()
        .flat_map(|left| parsed.iter().map(|right| (left, right)).collect::<Vec<(&Pair, &Pair)>>())
        .map(|(left, right)| left.add(right).magnitude())
        .max()
        .unwrap();

    // ----------- Print -----------

    println!("Part 1: {}", p1);
    println!("Part 2: {:?}", p2);
}

fn _get_test_input() -> String {
    return "


[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]



"
    .to_string();
}

fn _get_input() -> String {
    return "

[[[[8,1],8],[[8,1],0]],[[8,[2,4]],[0,8]]]
[[[[2,2],[7,4]],[[9,1],6]],8]
[[[3,6],[[8,7],[2,9]]],[8,[[2,3],9]]]
[[[[4,5],[1,4]],1],[[0,8],[2,[6,8]]]]
[[7,[[4,6],[0,0]]],[[4,3],5]]
[[[[8,4],4],[[4,4],[1,0]]],[[5,[5,5]],[[5,2],1]]]
[[[0,[5,8]],[1,7]],[[[5,0],[1,3]],7]]
[4,[[[6,2],[7,8]],[0,[4,4]]]]
[[[3,[5,3]],8],[[[6,8],4],[8,9]]]
[[[6,0],[9,[8,1]]],[[[9,7],3],0]]
[[9,[[9,3],[0,8]]],[[[5,3],0],[[5,6],2]]]
[[3,[[7,7],3]],[[7,[5,2]],[[6,9],0]]]
[1,[4,[3,8]]]
[[[[0,2],9],[[0,7],8]],[[5,4],[2,8]]]
[[[[1,8],9],[1,7]],[[4,[8,5]],[[6,3],[1,0]]]]
[[9,[[4,3],[3,3]]],[[[4,9],[0,9]],6]]
[7,[[8,0],[5,6]]]
[[[[3,2],1],[[4,9],6]],[[9,[1,1]],[[8,7],1]]]
[[[[5,1],[3,3]],0],[1,[[3,2],2]]]
[[7,9],[[9,9],[5,[9,5]]]]
[[[[4,3],[1,7]],[4,[9,2]]],[[6,[1,7]],[[8,0],3]]]
[[[5,[2,8]],[[1,2],7]],[[3,[0,5]],[[3,5],8]]]
[[[[2,2],6],[[2,1],7]],[[[4,6],8],7]]
[[2,[[3,0],[0,5]]],[[[3,4],[0,1]],0]]
[[[[9,9],5],[[9,9],6]],[[[4,1],2],0]]
[4,[[2,9],[6,2]]]
[[[8,6],6],7]
[[7,[8,2]],[[[5,5],6],[9,0]]]
[5,[[2,5],[[4,9],[8,6]]]]
[[4,[7,[9,6]]],7]
[[[9,[3,3]],[[3,1],[8,7]]],[[6,[3,5]],[4,1]]]
[[8,6],[8,[[0,2],[8,1]]]]
[6,[8,[[7,7],0]]]
[3,4]
[[9,[8,0]],[[[7,8],3],1]]
[5,[[3,[8,7]],[[5,0],[9,7]]]]
[[[[4,2],9],[6,[0,2]]],6]
[[4,[3,[4,9]]],[[4,[1,6]],1]]
[[[6,3],[8,8]],[5,[[9,3],[6,3]]]]
[[[9,9],[[7,1],6]],[[[1,0],[7,4]],[3,[2,0]]]]
[[[[2,5],9],[3,[6,2]]],[4,7]]
[[1,[7,8]],[[[0,1],8],[[1,1],9]]]
[[[9,[6,4]],[[9,8],[0,2]]],[[[8,9],[2,3]],[3,[8,0]]]]
[[[[6,8],2],3],[[2,2],5]]
[[[4,[8,5]],[[4,3],1]],[[[2,4],[4,4]],[[4,1],[1,7]]]]
[[[[2,6],6],[[9,2],4]],[[[9,9],[9,5]],5]]
[[[[7,5],[4,9]],4],[[[0,7],[3,6]],[[6,5],[3,0]]]]
[[[4,4],[[5,7],[8,5]]],[0,8]]
[[3,[[1,3],[7,5]]],[6,[[8,1],0]]]
[[[9,9],[5,[9,6]]],[[[4,0],[5,4]],6]]
[0,[[[9,2],4],3]]
[[[1,[8,5]],[0,[6,0]]],[[[6,5],[3,1]],[[6,2],[1,5]]]]
[[[4,0],[4,7]],6]
[1,[[[5,2],9],[[3,9],4]]]
[[[[9,6],[4,1]],4],[2,[[0,2],6]]]
[9,[[[1,5],[3,1]],1]]
[5,0]
[9,[[[7,5],[2,1]],[[2,3],[5,3]]]]
[[5,[[0,5],[9,5]]],[[[2,7],3],[[2,9],[3,5]]]]
[[[1,9],2],[[7,[1,7]],[8,[9,8]]]]
[[8,9],[[5,[9,0]],[[6,8],[5,2]]]]
[6,[[[1,3],[0,8]],4]]
[[[[9,8],[0,9]],[[8,4],[3,5]]],[[[5,0],8],[[6,8],1]]]
[[6,[[1,4],[7,0]]],[[3,4],[[2,1],[2,7]]]]
[[[5,0],[3,[4,7]]],[[9,3],[[9,4],[9,6]]]]
[[[[8,3],[8,0]],5],[[[5,5],[0,2]],[[0,1],9]]]
[[[[6,4],[1,8]],[3,[0,2]]],[8,[[8,8],5]]]
[2,[[2,1],[1,4]]]
[8,[0,[3,5]]]
[[[[0,2],3],[[4,9],[1,2]]],[[8,2],[6,[7,1]]]]
[[[0,0],9],1]
[8,[[4,1],[[1,3],9]]]
[[[8,[5,9]],9],[[[5,7],[9,0]],3]]
[[5,[2,9]],7]
[5,6]
[[[[7,5],[8,3]],[[4,3],8]],[[2,2],[[7,2],[4,2]]]]
[[[9,5],[3,[1,5]]],6]
[[[[7,4],[7,9]],[[3,1],[3,1]]],[[[6,4],[0,1]],1]]
[[3,[[7,4],9]],[[[5,8],[2,7]],[[0,4],[3,6]]]]
[[[3,[2,3]],[[6,0],[7,7]]],1]
[[2,[[8,8],[2,3]]],[5,2]]
[[[0,[5,5]],[8,1]],5]
[[3,9],[6,[[0,5],[1,7]]]]
[[[[3,0],9],[8,2]],[[[2,2],8],0]]
[[[9,6],[[5,1],[4,9]]],[[[1,1],[0,3]],[[4,9],[7,5]]]]
[[[2,[6,1]],[[5,7],[9,2]]],[[[4,2],8],9]]
[[9,[7,1]],[[4,5],[9,1]]]
[[9,[5,0]],[[1,7],[[9,6],[4,5]]]]
[[[[1,1],[8,7]],4],[[0,4],[[1,7],[3,5]]]]
[[5,[1,[8,4]]],[[[9,4],0],[1,[5,5]]]]
[[[5,[1,6]],[6,0]],[[0,[9,7]],1]]
[2,[9,[[0,3],[2,3]]]]
[3,[4,[[0,9],8]]]
[[5,6],[[[9,9],[4,0]],[7,[2,0]]]]
[[[[5,1],6],[[1,0],[7,1]]],[[6,[1,0]],[[4,2],[0,0]]]]
[[[4,[0,2]],6],[[[4,3],[8,0]],[[9,6],[1,5]]]]
[[[[5,3],[2,2]],[8,[8,3]]],[[9,1],2]]
[[3,4],[[[4,7],[2,3]],[9,[9,0]]]]
[[[5,[6,2]],[[1,5],[9,2]]],[[[7,9],3],[[6,7],[6,2]]]]
[[[5,3],9],[[2,[4,3]],[[5,3],1]]]




"
    .to_string();
}
