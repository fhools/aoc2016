use std::collections::HashSet;

use md5::digest::generic_array::sequence::Lengthen;
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    floors: Vec<Vec<String>>,
    elevator: i32,
}

impl State {
    const MAX_FLOOR: i32 = 4;
    fn new() -> State {
        State {
            floors: vec![vec![], vec![], vec![], vec![]],
            elevator: 0,
        }
    }
    fn add(&mut self, floor: usize, item: String) {
        self.floors[floor].push(item);
    }

    fn is_valid(&self) -> bool {
        for floor in self.floors.iter() {
            let mut generators = Vec::new();
            let mut chips = Vec::new();
            for item in floor.iter() {
                if item.contains("G") {
                    generators.push(item.clone());
                } else if item.contains("M") {
                    chips.push(item.clone());
                }
            }
            if !chips.is_empty() && !generators.is_empty() {
                for chip in chips.iter() {
                    if !generators.contains(&chip.replace("M", "G")) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn print(&self) {
        for (i, floor) in self.floors.iter().rev().enumerate() {
            print!("F{} ", 4 - i);
            if self.elevator + 1 == (4 - i) as i32 {
                print!("E ");
            } else {
                print!(". ");
            }

            for i in 0..9 {
                if i == 0 && floor.contains(&"TG".to_string()) {
                    print!("TG ");
                } else if i == 1 && floor.contains(&"TM".to_string()) {
                    print!("TM ");
                } else if i == 2 && floor.contains(&"ZG".to_string()) {
                    print!("ZG ");
                } else if i == 3 && floor.contains(&"RG".to_string()) {
                    print!("RG ");
                } else if i == 4 && floor.contains(&"RM".to_string()) {
                    print!("RM ");
                } else if i == 5 && floor.contains(&"CG".to_string()) {
                    print!("CG ");
                } else if i == 6 && floor.contains(&"CM".to_string()) {
                    print!("CM ");
                } else if i == 7 && floor.contains(&"PM".to_string()) {
                    print!("PM ");
                } else if i == 8 && floor.contains(&"ZM".to_string()) {
                    print!("ZM ");
                } else {
                    print!(".  ");
                }
            }
            // for i in 0..4 {
            //     if i == 0 && floor.contains(&"LG".to_string()) {
            //         print!("LG ");
            //     } else if i == 1 && floor.contains(&"LM".to_string()) {
            //         print!("LM ");
            //     } else if i == 2 && floor.contains(&"HG".to_string()) {
            //         print!("HG ");
            //     } else if i == 3 && floor.contains(&"HM".to_string()) {
            //         print!("HM ");
            //     } else {
            //         print!(".  ");
            //     }
            // }
            println!();
        }
        println!();
    }

    // returns  vector of (steps, state, steps_path)
    fn enumerate_move(&self, step: i32, steps: Vec<State>) -> Vec<(i32, State, Vec<State>)> {
        let current_floor = self.floors[self.elevator as usize].clone();
        let mut combinations = Vec::new();
        generate_combinations(&current_floor, 0, &mut Vec::new(), &mut combinations);

        //println!("combos:");
        //for c in combinations.iter() {
        //    println!("{:?}", c);
        //}
        let mut next_states = Vec::new();
        // for each element of combinations, try to move it up and then down
        for combination in combinations.iter() {
            for elevator_move in [-1i32, 1i32].iter() {
                // if its an invalid move, i.e. wrong floor or empty move or
                // more than two items in the elevator, skip it
                if self.elevator + elevator_move < 0
                    || self.elevator + elevator_move >= State::MAX_FLOOR
                    || combination.is_empty()
                    || combination.len() > 2
                {
                    continue;
                }

                // make a new state with the elevator moved
                let mut new_state = self.clone();
                //print!("making moves ");
                for (i, item) in combination.iter().enumerate() {
                    assert!(item.len() != 0);
                    // if i > 0 {
                    //     print!(", ");
                    // }
                    // print!("{}", item);
                    // Remove the item from the current floor of the elevator's state.
                    new_state.floors[self.elevator as usize].retain(|x| x != item);
                    // Add the item to the new floor of the elevator's state.
                    new_state.floors[(self.elevator + elevator_move) as usize].push(item.clone());
                }
                // println!();

                new_state.elevator += elevator_move;
                assert!(new_state.floors[new_state.elevator as usize].len() != 0);
                if new_state.is_valid() {
                    //println!("added for step {} from:", step + 1);
                    //self.print();
                    //println!("to:");
                    //new_state.print();

                    // add a step to next_steps
                    let mut next_steps = steps.clone();
                    next_steps.push(new_state.clone());

                    // add new state to next_states
                    next_states.push((step + 1, new_state, next_steps));
                } else {
                    // println!("not valid");
                }
            }
        }
        next_states
    }

    fn is_done(&self) -> bool {
        self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
    }
}

fn generate_combinations(
    set: &[String],
    index: usize,
    current: &mut Vec<String>,
    combinations: &mut Vec<Vec<String>>,
) {
    if index == set.len() {
        if current.len() == 0 || current.len() > 2 {
            return;
        }
        combinations.push(current.clone());
        return;
    }

    // Include the current element
    current.push(set[index].clone());
    generate_combinations(set, index + 1, current, combinations);

    // Exclude the current element
    current.pop();
    generate_combinations(set, index + 1, current, combinations);
}

/*
fn find_finish(state: &State, num_moves: i32, steps: &mut Vec<i32>) {
    if state.is_done() {
        steps.push(num_moves);
    }
    let next_states = state.enumerate_move();
    for next_state in next_states.iter() {
        find_finish(next_state, num_moves + 1, steps);
    }
}
*/

fn main() {
    // let mut state_try = State {
    //     /*F4 .
    //     F3 . LG
    //     F2 E HG HM
    //     F1 . LM  */
    //     floors: vec![
    //         vec!["LM".to_string()],
    //         vec!["HG".to_string(), "HM".to_string()],
    //         vec!["LG".to_string()],
    //         vec![],
    //     ],
    //     elevator: 1,
    // };
    // state_try.print();
    // for next_state in state_try.enumerate_move(0, vec![state_try.clone()]).iter() {
    //     next_state.1.print();
    // }

    // println!("next");
    // let mut state_try = State {
    //     /*F4 .
    //     F3 E LG HG HM
    //     F2 .
    //     F1 . LM
    //     */
    //     floors: vec![
    //         vec!["LM".to_string()],
    //         vec![],
    //         vec!["LG".to_string(), "HG".to_string(), "HM".to_string()],
    //         vec![],
    //     ],
    //     elevator: 2,
    // };
    // state_try.print();
    // for next_state in state_try.enumerate_move(0, vec![state_try.clone()]).iter() {
    //     next_state.1.print();
    // }
    // println!("next 2");
    // let mut state_try = State {
    //     /*F4 .
    //     F3   LG HG
    //     F2 E HM
    //     F1 . LM
    //     */
    //     floors: vec![
    //         vec!["LM".to_string()],
    //         vec!["HM".to_string()],
    //         vec!["LG".to_string(), "HG".to_string()],
    //         vec![],
    //     ],
    //     elevator: 1,
    // };

    // println!("current:");
    // state_try.print();
    // for next_state in state_try.enumerate_move(0, vec![state_try.clone()]).iter() {
    //     next_state.1.print();
    // }

    // std::process::exit(0);

    // let mut state = State {
    //     floors: vec![
    //         vec!["HM".to_string(), "LM".to_string()],
    //         vec!["HG".to_string()],
    //         vec!["LG".to_string()],
    //         vec![],
    //     ],
    //     elevator: 0,
    // };

    let mut state = State {
        floors: vec![
            vec![
                "TM".to_string(),
                "TG".to_string(),
                "PG".to_string(),
                "ZG".to_string(),
                "RG".to_string(),
                "RM".to_string(),
                "CG".to_string(),
                "CM".to_string(),
            ],
            vec!["PM".to_string(), "ZM".to_string()],
            vec![],
            vec![],
        ],
        elevator: 0,
    };

    let mut move_set: HashSet<State> = HashSet::new();
    let mut finish_step_counts = Vec::new();
    state.print();
    let mut next_moves = state.enumerate_move(0, vec![state.clone()]);
    loop {
        if next_moves.is_empty() {
            break;
        }
        let next_move = next_moves.remove(0);
        //next_move.1.print();
        if next_move.1.is_done() {
            println!("done in {} steps", next_move.0);
            finish_step_counts.push((next_move.0, next_move.2.clone()));
            continue;
        }
        let current_step = next_move.0;
        let next_next_moves = next_move.1.enumerate_move(current_step, next_move.2);
        for next_next_move in next_next_moves.iter() {
            let mut has_same_floor = false;
            for old_move in move_set.iter() {
                let mut same_floors = true;
                for i in 0..4 {
                    let mut old_floor = (*old_move).floors[i].clone();
                    let mut new_floor = (*next_next_move).1.floors[i].clone();
                    old_floor.sort();
                    new_floor.sort();
                    if old_floor != new_floor {
                        same_floors = false;
                        break;
                    }
                }
                if same_floors && (*old_move).elevator == (*next_next_move).1.elevator {
                    //println!("same ====== ");
                    //(*old_move).print();
                    //(*next_next_move).1.print();
                    has_same_floor = true;
                } else {
                    //println!("not same ======");
                    //(*old_move).print();
                    //(*next_next_move).1.print();
                }
            }
            if move_set.len() == 0 || !has_same_floor {
                println!("pushing move {}", move_set.len());
                next_next_move.1.print();
                next_moves.push(next_next_move.clone());
                move_set.insert(next_next_move.1.clone());
            }
        }
    }
    if let Some(min) = finish_step_counts.iter().min_by(|x, y| x.0.cmp(&y.0)) {
        println!("min steps: {}", min.0);
        println!("solution: ");
        for step in min.1.iter() {
            step.print();
        }
    }
}
