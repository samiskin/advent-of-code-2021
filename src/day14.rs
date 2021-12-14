mod utils;
use std::collections::HashMap;

fn main() {
    let input = _get_test_input();
    // let input = _get_input();

    // ----------- Parse Input -----------

    let sections = input.trim().split("\n\n").collect::<Vec<&str>>();
    let template = sections[0].trim();
    let rules = sections[1]
        .trim()
        .split('\n')
        .map(|l| l.split(" -> ").collect::<Vec<&str>>())
        .map(|v| {
            (
                v[0].chars().collect::<Vec<char>>(),
                v[1].chars().next().unwrap(),
            )
        })
        .collect::<Vec<(Vec<char>, char)>>();

    // ----------- Solve -----------

    let mut rule_map: HashMap<(char, char), char> = HashMap::new();
    for rule in rules.iter() {
        rule_map.insert((rule.0[0], rule.0[1]), rule.1);
    }

    let simulate_n = |n: usize| -> u64 {
        let mut rule_counts: HashMap<(char, char), u64> = HashMap::new();
        for pair in template.chars().zip(template.chars().skip(1)) {
            *rule_counts.entry(pair).or_insert(0) += 1;
        }

        let mut char_counts: HashMap<char, u64> = HashMap::new();
        for c in template.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        for _ in 0..n {
            let mut next_rule_counts: HashMap<(char, char), u64> = HashMap::new();
            for (pair, count) in rule_counts.iter() {
                let center = rule_map.get(pair).unwrap();
                *next_rule_counts.entry((pair.0, *center)).or_insert(0) += count;
                *next_rule_counts.entry((*center, pair.1)).or_insert(0) += count;
                *char_counts.entry(*center).or_insert(0) += count;
            }
            rule_counts = next_rule_counts;
        }

        let most_freq = char_counts.values().max().unwrap();
        let min_freq = char_counts.values().min().unwrap();

        return most_freq - min_freq;
    };

    // ----------- Print -----------

    println!("Part 1: {:?}", simulate_n(10));
    println!("Part 2: {:?}", simulate_n(40));
}

fn _get_test_input() -> String {
    return "

NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C

"
    .to_string();
}

fn _get_input() -> String {
    return "

SHHBNFBCKNHCNOSHHVFF

CK -> N
VP -> B
CF -> S
FO -> V
VC -> S
BV -> V
NP -> P
SN -> C
KN -> V
NF -> P
SB -> C
PC -> B
OB -> V
NS -> O
FH -> S
NK -> S
HO -> V
NV -> O
FV -> O
FB -> S
PS -> S
FN -> K
HS -> O
CB -> K
HV -> P
NH -> C
BO -> B
FF -> N
PO -> F
BB -> N
PN -> C
BP -> C
HN -> K
CO -> P
BF -> H
BC -> S
CV -> B
VV -> F
FS -> B
BN -> P
VK -> S
PV -> V
PP -> B
PH -> N
SS -> O
SK -> S
NC -> P
ON -> F
NB -> N
CC -> N
SF -> H
PF -> H
OV -> O
KH -> C
CP -> V
PK -> O
KC -> K
KK -> C
KF -> B
HP -> C
FK -> H
BH -> K
VN -> H
OO -> S
SC -> K
SP -> B
KO -> V
KV -> F
HK -> N
FP -> N
NN -> B
VS -> O
HC -> K
BK -> N
KS -> K
VB -> O
OH -> F
KB -> F
KP -> H
HB -> N
NO -> N
OF -> O
BS -> H
VO -> H
SH -> O
SV -> K
HF -> C
CS -> F
FC -> N
VH -> H
OP -> K
OK -> H
PB -> K
HH -> S
OC -> V
VF -> B
CH -> K
CN -> C
SO -> P
OS -> O

"
    .to_string();
}
