mod utils;
use std::collections::{BinaryHeap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, PartialEq, Eq)]
struct GameState {
    hallways: [Option<char>; 11],
    rooms: [Vec<char>; 4],
    cost: u64,
    room_size: usize,
}

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hallways.hash(state);
        self.rooms.hash(state);
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl GameState {
    fn room_to_hallway_index(room_i: usize) -> usize {
        (room_i * 2) + 2
    }
    fn move_cost(ch: &char) -> u64 {
        match ch {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => 0,
        }
    }
    fn room_exit_steps(&self, room: usize) -> usize {
        (self.room_size - self.rooms[room].len()) + 1
    }

    fn get_target(&self, ch: &char) -> usize {
        match ch {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            _ => 0,
        }
    }

    fn can_move(&self, a: usize, b: usize) -> bool {
        let (start, end) = (usize::min(a, b), usize::max(a, b));
        for i in start..=end {
            if i != a && self.hallways[i] != None {
                return false;
            }
        }
        return true;
    }

    fn next_states(&self) -> Vec<GameState> {
        let mut next_states = vec![];

        // Move from Hallways

        let occupied_hallways = self.hallways.iter().enumerate().filter_map(|(i, cell)| {
            if let Some(ch) = cell {
                Some((i, ch))
            } else {
                None
            }
        });

        for (i, ch) in occupied_hallways {
            let target_room = self.get_target(ch);
            let room_hallway_index = GameState::room_to_hallway_index(target_room);
            if !self.can_move(i, room_hallway_index) {
                continue;
            }
            if self.rooms[target_room].len() >= self.room_size {
                continue;
            }
            if self.rooms[target_room].iter().any(|c| c != ch) {
                continue;
            }

            let mut next_room = self.clone();
            next_room.hallways[i] = None;
            next_room.rooms[target_room].push(*ch);
            next_room.cost += ((i as i32 - room_hallway_index as i32).abs() as u64
                + (next_room.room_exit_steps(target_room)) as u64)
                * GameState::move_cost(ch);
            next_states.push(next_room);
        }

        // Move from Rooms
        for (i, room) in self.rooms.iter().enumerate() {
            if room.iter().all(|c| self.get_target(c) == i) {
                continue;
            }
            if room.is_empty() {
                continue;
            }

            let room_hallway_index = GameState::room_to_hallway_index(i);
            for (hallway_i, contents) in self.hallways.iter().enumerate() {
                if [2, 4, 6, 8].contains(&hallway_i) || contents.is_some() {
                    continue;
                }
                if !self.can_move(room_hallway_index, hallway_i) {
                    continue;
                }

                let mut next_state = self.clone();
                let ch = next_state.rooms[i].pop().unwrap();
                next_state.hallways[hallway_i] = Some(ch);
                next_state.cost += ((hallway_i as i32 - room_hallway_index as i32).abs() as u64
                    + self.room_exit_steps(i) as u64)
                    * GameState::move_cost(&ch);
                next_states.push(next_state);
            }
        }

        next_states
    }

    fn is_complete(&self) -> bool {
        self.rooms.iter().enumerate().all(|(i, room)| {
            room.len() == self.room_size && room.iter().all(|ch| self.get_target(ch) == i)
        })
    }
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nHallway: ").unwrap();
        for hallway in self.hallways {
            if let Some(ch) = hallway {
                write!(f, "{}", ch).unwrap();
            } else {
                write!(f, ".").unwrap();
            }
        }
        write!(f, "\nRooms:    ").unwrap();
        for level in 0..self.room_size {
            let i = self.room_size - level - 1;
            for room in &self.rooms {
                if i >= room.len() {
                    write!(f, " .").unwrap();
                } else {
                    write!(f, " {}", room[i]).unwrap();
                }
            }
            write!(f, "\n          ").unwrap();
        }

        write!(f, "\nCost: {}\n", self.cost).unwrap();
        Ok(())
    }
}

fn main() {
    let input = _get_test_input();
    // let input = _get_input();

    let mut p1 = 0;
    let mut p2 = 0;

    // ----------- Parse Input -----------

    let parsed = input
        .trim()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let grid = utils::Grid::new(parsed);

    let initial_state = GameState {
        hallways: [None; 11],
        rooms: [
            [*grid.get(3, 2), *grid.get(3, 3)].to_vec(),
            [*grid.get(5, 2), *grid.get(5, 3)].to_vec(),
            [*grid.get(7, 2), *grid.get(7, 3)].to_vec(),
            [*grid.get(9, 2), *grid.get(9, 3)].to_vec(),
        ],
        cost: 0,
        room_size: 2,
    };
    // let mut initial_state = GameState {
    //     hallways: [None; 11],
    //     rooms: [vec![], vec![], vec![], vec![]],
    //     cost: 0,
    //     room_size: 2,
    // };
    // initial_state.hallways[1] = Some('B');
    // initial_state.hallways[3] = Some('B');
    // initial_state.hallways[7] = Some('C');
    // initial_state.hallways[10] = Some('D');

    // ----------- Solve -----------

    // println!("Initail: {:?}", initial_state);
    // let next_states = initial_state.next_states();
    // for (i, state) in next_states.iter().enumerate() {
    //     println!("Next {}: {:?}", i, state);
    // }
    // println!("Is complete: {}", initial_state.is_complete());

    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    heap.push(initial_state.clone());
    let mut last_seen = 0;
    let mut seen_count = 0;

    seen.insert(initial_state.clone());
    while let Some(state) = heap.pop() {
        if state.is_complete() {
            println!("COMPLETE!: {:?}", state);
            p1 = state.cost;
            break;
        }
        let next_states = state.next_states();
        if state.cost > last_seen + 100 {
            last_seen = state.cost;
            println!(
                "State: {:?}\n Next States: {:?}\nSeen count: {}\nHeap Size: {}",
                state,
                next_states.len(),
                seen_count,
                heap.len()
            );
        }
        for next_state in next_states {
            if seen.contains(&next_state) {
                seen_count += 1;
                continue;
            }
            seen.insert(next_state.clone());
            heap.push(next_state.clone());
        }
    }

    // ----------- Print -----------

    println!("Part 1: {}", p1);
    println!("Part 2: {:?}", 0);
}

fn _get_test_input() -> String {
    return "

#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########

"
    .to_string();
}

fn _get_input() -> String {
    return "

#############
#...........#
###C#B#A#D###
  #B#C#D#A#
  #########

"
    .to_string();
}
