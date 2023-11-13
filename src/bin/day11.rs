use std::collections::HashSet;
use std::time::Instant;

static PART2: bool = true;
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    floors: [u16; 4],
    elevator: i32,
}

impl State {
    const MAX_FLOOR: i32 = 4;

    fn is_valid(&self) -> bool {
        for floor in self.floors.iter() {
            let mut num_generators = 0u32;

            // loop through the bit masks of floor, each even position
            // is a chip, each odd position is a generator, count
            // the  generators
            for i in 0..(if !PART2 { 12 } else { 14 }) {
                if floor & (1 << i) != 0 {
                    // odd numbers are generators
                    if i % 2 == 1 {
                        num_generators += 1;
                    }
                }
            }

            // loop through the bit masks again, now for each chip that is
            // on, check  it's generator which is
            // in the +1 position from corresponding chip position. and if it
            // is off but num_generators is greater than 0, then it's invalid
            // since the chip will be fried
            for i in 0..(if !PART2 { 12 } else { 14 }) {
                // if we're on a chip and its on
                if i % 2 == 0 && floor & (1 << i) != 0 {
                    // if the generator is off and there are generators on the floor
                    // then it must be a generator that is not for it, so it's invalid
                    if floor & (1 << (i + 1)) == 0 && num_generators > 0 {
                        return false;
                    }
                }
            }

            /* old */
            // if !chips.is_empty() && !generators.is_empty() {
            //     for chip in chips.iter() {
            //         if !generators.contains(&chip.replace("M", "G")) {
            //             return false;
            //         }
            //     }
            // }
        }
        true
    }

    fn print(&self) {
        // let mut item_names = vec![
        //     "LM".to_string(),
        //     "LG".to_string(),
        //     "HM".to_string(),
        //     "HG".to_string(),
        // ];
        // the following names: TG TM PG PM ZG ZM RG RM CG CM
        let item_names;
        if !PART2 {
            item_names = vec![
                "TM".to_string(),
                "TG".to_string(),
                "PM".to_string(),
                "PG".to_string(),
                "ZM".to_string(),
                "ZG".to_string(),
                "RM".to_string(),
                "RG".to_string(),
                "CM".to_string(),
                "CG".to_string(),
                "EM".to_string(),
                "EG".to_string(),
            ];
        } else {
            item_names = vec![
                "TM".to_string(),
                "TG".to_string(),
                "PM".to_string(),
                "PG".to_string(),
                "ZM".to_string(),
                "ZG".to_string(),
                "RM".to_string(),
                "RG".to_string(),
                "CM".to_string(),
                "CG".to_string(),
                "EM".to_string(),
                "EG".to_string(),
                "DM".to_string(),
                "DG".to_string(),
            ]
        }

        for (i, floor) in self.floors.iter().rev().enumerate() {
            print!("F{} ", 4 - i);
            if self.elevator + 1 == (4 - i) as i32 {
                print!("E ");
            } else {
                print!(". ");
            }

            for i in (0..(if !PART2 { 12 } else { 14 })).rev() {
                if floor & (1 << i) != 0 {
                    print!("{} ", item_names[i]);
                } else {
                    print!(".  ");
                }
            }
            println!();
        }
        println!();
    }

    // prune next states that have 2 chip and 2 generator. it doesn't matter
    // which chip is moved up, so we can remove those states
    fn optimize_next_states(
        &self,
        states: &Vec<(i32, State, Vec<State>)>,
    ) -> Vec<(i32, State, Vec<State>)> {
        let mut next_states = Vec::new();
        let mut pruned_states = Vec::new();
        let mut count_chipandgen = 0;
        let mut indices_of_chip_and_gen = Vec::new();
        let mut state_index_of_chip_and_gen_up = Vec::new();
        let mut state_index_of_chip_and_gen_down = Vec::new();

        for i in (0..(if !PART2 { 12 } else { 14 })).step_by(2) {
            let floor = self.floors[self.elevator as usize];
            if floor & (0x3 << i) == (0x3 << i) {
                count_chipandgen += 1;
                indices_of_chip_and_gen.push(i);
            }
        }

        // nothing to prune
        if count_chipandgen < 2 {
            return states.clone();
        }

        // println!("current state:");
        // self.print();
        // loop through each state, if the state has any of the of the chip in
        // indices_of_chip_and_gen set on the floor above, remember their
        // state_index in state_index_of_chip_and_gen
        for (state_index, state) in states.iter().enumerate() {
            let cur_floor = self.floors[self.elevator as usize];
            let next_elevator = state.1.elevator;
            let next_floor = state.1.floors[next_elevator as usize];
            if next_elevator > 0 && next_elevator - 1 == self.elevator {
                for chip_index in &indices_of_chip_and_gen {
                    // if cur_floor has chip and next_floor has same chip then
                    // only one of them need to be used so save index for pruning
                    if cur_floor & (1 << chip_index) != 0
                        && next_floor & (3 << chip_index) == (1 << chip_index)
                    {
                        state_index_of_chip_and_gen_up.push(state_index);
                    }
                }
            } else {
                for chip_index in &indices_of_chip_and_gen {
                    // if cur_floor has gen and next_floor has same then then prune
                    if cur_floor & (1 << chip_index) != 0
                        && next_floor & (3 << chip_index) == (1 << chip_index)
                    {
                        state_index_of_chip_and_gen_down.push(state_index);
                    }
                }
            }
        }

        // add the states, but only one of the one in state_index_of_chip_and_gen
        let mut added_one_up = false;
        let mut added_one_down = false;
        for (state_index, state) in states.iter().enumerate() {
            if state_index_of_chip_and_gen_up.contains(&state_index) && !added_one_up {
                added_one_up = true;
                next_states.push(state.clone());
                continue;
            } else if state_index_of_chip_and_gen_up.contains(&state_index) {
                pruned_states.push(state.clone());
                continue;
            } else if state_index_of_chip_and_gen_down.contains(&state_index) && !added_one_down {
                added_one_down = true;
                next_states.push(state.clone());
                continue;
            } else if state_index_of_chip_and_gen_down.contains(&state_index)
                && !state_index_of_chip_and_gen_up.contains(&state_index)
                && added_one_down
            {
                continue;
            }

            next_states.push(state.clone());
        }
        // if next_states.len() < states.len() {
        //     println!("original states:");
        //     for state in states.iter() {
        //         state.1.print();
        //     }
        //     println!("kept states:");
        //     for state in next_states.iter() {
        //         state.1.print();
        //     }
        //     println!("pruned states:");
        //     for state in pruned_states.iter() {
        //         state.1.print();
        //     }
        // }
        next_states
    }

    // returns  vector of (steps, state, steps_path)
    fn enumerate_move(&self, step: i32, steps: &Vec<State>) -> Vec<(i32, State, Vec<State>)> {
        let current_floor = self.floors[self.elevator as usize];
        let mut combinations = Vec::new();

        let mut current = 0u16;
        //generate_combinations(current_floor, 0, &mut current, &mut combinations);
        combinations = generate_combinations_v3(current_floor);
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
                if (self.elevator == 0 && *elevator_move == -1)
                    || self.elevator + elevator_move == State::MAX_FLOOR
                    || *combination == 0u16
                    || combination.count_ones() > 2
                {
                    continue;
                }

                // make a new state with the elevator moved
                let mut new_state = self.clone();
                //print!("making moves ");
                for i in 0..(if !PART2 { 12 } else { 14 }) {
                    if combination & (1 << i) != 0 {
                        // if i > 0 {
                        //     print!(", ");
                        // }
                        // print!("{}", 1 << 1);
                        // Remove the item from the current floor of the elevator's state.
                        new_state.floors[self.elevator as usize] &= !(1 << i);
                        // Add the item to the new floor of the elevator's state.
                        new_state.floors[(self.elevator + elevator_move) as usize] |= 1 << i;
                    }
                }

                //println!();

                new_state.elevator += elevator_move;
                assert!(new_state.floors[new_state.elevator as usize].count_ones() != 0);

                if new_state.is_valid() {
                    //println!("added for step {} from:", step + 1);
                    //self.print();
                    //println!("to:");
                    //new_state.print();
                    // add a step to next_steps
                    //let mut next_steps = steps.clone();
                    //next_steps.push(new_state.clone());

                    // add new state to next_states
                    //next_states.push((step + 1, new_state, next_steps));
                    next_states.push((step + 1, new_state, steps.clone()));
                } else {
                    // println!("not valid");
                }
            }
        }
        next_states = self.optimize_next_states(&next_states);
        next_states
    }
    // old
    // returns  vector of (steps, state, steps_path)
    // fn enumerate_move(&self, step: i32, steps: Vec<State>) -> Vec<(i32, State, Vec<State>)> {
    //     let current_floor = self.floors[self.elevator as usize].clone();
    //     let mut combinations = Vec::new();
    //     generate_combinations(&current_floor, 0, &mut Vec::new(), &mut combinations);

    //     //println!("combos:");
    //     //for c in combinations.iter() {
    //     //    println!("{:?}", c);
    //     //}
    //     let mut next_states = Vec::new();
    //     // for each element of combinations, try to move it up and then down
    //     for combination in combinations.iter() {
    //         for elevator_move in [-1i32, 1i32].iter() {
    //             // if its an invalid move, i.e. wrong floor or empty move or
    //             // more than two items in the elevator, skip it
    //             if self.elevator + elevator_move < 0
    //                 || self.elevator + elevator_move >= State::MAX_FLOOR
    //                 || combination.is_empty()
    //                 || combination.len() > 2
    //             {
    //                 continue;
    //             }

    //             // make a new state with the elevator moved
    //             let mut new_state = self.clone();
    //             //print!("making moves ");
    //             for (i, item) in combination.iter().enumerate() {
    //                 assert!(item.len() != 0);
    //                 // if i > 0 {
    //                 //     print!(", ");
    //                 // }
    //                 // print!("{}", item);
    //                 // Remove the item from the current floor of the elevator's state.
    //                 new_state.floors[self.elevator as usize].retain(|x| x != item);
    //                 // Add the item to the new floor of the elevator's state.
    //                 new_state.floors[(self.elevator + elevator_move) as usize].push(item.clone());
    //             }
    //             // println!();

    //             new_state.elevator += elevator_move;
    //             assert!(new_state.floors[new_state.elevator as usize].len() != 0);
    //             if new_state.is_valid() {
    //                 //println!("added for step {} from:", step + 1);
    //                 //self.print();
    //                 //println!("to:");
    //                 //new_state.print();

    //                 // add a step to next_steps
    //                 let mut next_steps = steps.clone();
    //                 next_steps.push(new_state.clone());

    //                 // add new state to next_states
    //                 next_states.push((step + 1, new_state, next_steps));
    //             } else {
    //                 // println!("not valid");
    //             }
    //         }
    //     }
    //     next_states
    // }

    fn is_done(&self) -> bool {
        self.floors[0] == 0 && self.floors[1] == 0 && self.floors[2] == 0
    }
    /* old */
    // fn is_done(&self) -> bool {
    //     self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
    // }
}

fn generate_combinations_v3(set: u16) -> Vec<u16> {
    let mut combinations = Vec::new();

    // Generate all possible combinations (subsets) of set bits
    for subset in 0..=set {
        if set & subset == subset {
            if subset.count_ones() == 1 || subset.count_ones() == 2 {
                combinations.push(subset);
            }
        }
    }

    combinations.sort(); // Sorting before deduplication
    combinations.dedup(); // Remove duplicates

    // println!("current set: {:016b}", set);
    // for c in combinations.iter() {
    //     println!("gen {:16b}", c);
    // }

    combinations
}

fn generate_combinations_v2(set: u16) -> Vec<u16> {
    let mut combinations = Vec::new();
    // for each bit in set that is on, compute all the combinations in that set that
    // goes through all combinations of the bit set on and off, without recursive calls
    let mut subset = 0;

    // Iterate over each bit position
    for i in 0..(if !PART2 { 12 } else { 14 }) {
        let mask = 1 << i;

        // Check if the bit is set in the original set
        if set & mask != 0 {
            // Generate combinations with this bit on and off
            let mut new_combinations = Vec::new();
            for comb in &combinations {
                new_combinations.push(comb | mask);
            }
            combinations.extend(new_combinations);
            subset |= mask;
            combinations.push(subset);
        }
    }

    // Include the case where all bits are off
    let mut combinations: Vec<_> = combinations
        .iter()
        .filter(|&&x| {
            let ones = x.count_ones();
            ones == 1 || ones == 2
        })
        .cloned()
        .collect();
    combinations.sort();
    combinations.dedup();
    // println!("current set: {:16b}", set);
    // for c in combinations.iter() {
    //     println!("gen {:16b}", c);
    // }
    combinations
}

fn generate_combinations(set: u16, index: usize, current: &mut u16, combinations: &mut Vec<u16>) {
    if index == (if !PART2 { 12 } else { 14 }) {
        if current.count_ones() == 0 || current.count_ones() > 2 {
            return;
        }
        combinations.push(current.clone());
        return;
    }

    // Include the current element
    *current |= set & (1 << index);
    generate_combinations(set, index + 1, current, combinations);

    // Exclude the current element
    *current &= !(set & (1 << index));
    generate_combinations(set, index + 1, current, combinations);
}

/* old */
// fn generate_combinations(
//     set: &[String],
//     index: usize,
//     current: &mut Vec<String>,
//     combinations: &mut Vec<Vec<String>>,
// ) {
//     if index == set.len() {
//         if current.len() == 0 || current.len() > 2 {
//             return;
//         }
//         combinations.push(current.clone());
//         return;
//     }

//     // Include the current element
//     current.push(set[index].clone());
//     generate_combinations(set, index + 1, current, combinations);

//     // Exclude the current element
//     current.pop();
//     generate_combinations(set, index + 1, current, combinations);
//}

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
    // let mut state = State {
    //     floors: [0x05, 0x08, 0x02, 0x00],
    //     elevator: 0,
    // };

    // let mut state = State {
    //     floors: vec![
    //         vec![
    //             "TM".to_string(),
    //             "TG".to_string(),
    //             "PG".to_string(),
    //             "ZG".to_string(),
    //             "RG".to_string(),
    //             "RM".to_string(),
    //             "CG".to_string(),
    //             "CM".to_string(),
    //         ],
    //         vec!["PM".to_string(), "ZM".to_string()],
    //         vec![],
    //         vec![],
    //     ],
    //     elevator: 0,
    // };

    let state;

    if !PART2 {
        // state = State {
        //     floors: [0x05, 0x8, 0x2, 0x0],
        //     elevator: 0,
        // };
        // TG TM PG PM ZG ZM RG RM CG CM
        // 0b1101011111
        // 0b0010100000
        state = State {
            floors: [0b10_1011_1111, 0b01_0100_0000, 0x0, 0x0],
            elevator: 0,
        };
        // state = State {
        //     floors: [0b1010_1111_1111, 0b0101_0000_0000, 0x0, 0x0],
        //     elevator: 0,
        // }
    } else {
        // TG TM PG PM ZG ZM RG RM CG CM EG EM DG DM
        // 0b11101011111111
        // 0b00010100000000
        state = State {
            floors: [0b11111110101111, 0b00000001010000, 0x0, 0x0],
            elevator: 0,
        };
    }

    let mut move_set: HashSet<State> = HashSet::new();
    let mut finish_step_counts = Vec::new();
    state.print();
    //let mut next_moves = state.enumerate_move(0, &vec![state.clone()]);
    let states = Vec::new();
    let mut next_moves = state.enumerate_move(0, &states);
    let mut num_moves = 0usize;
    // declare a time variable to hold the current time
    let mut time = Instant::now();
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
        let next_next_moves = next_move.1.enumerate_move(current_step, &next_move.2);
        for next_next_move in next_next_moves.iter() {
            if !move_set.contains(&next_next_move.1) {
                next_moves.push(next_next_move.clone());
                move_set.insert(next_next_move.1.clone());
            }
        }
        num_moves += 1;
        // get current time now and compare with last and update time
        let now = Instant::now();
        let elapsed = now.duration_since(time);
        if elapsed.as_secs() >= 10 {
            println!("total moves: {}", num_moves);
            println!("moves seen: {}", move_set.len());
            println!("moves in queue: {}", next_moves.len());
            time = now;
        }
    }
    if let Some(min) = finish_step_counts.iter().min_by(|x, y| x.0.cmp(&y.0)) {
        println!("min steps: {}", min.0);
        //println!("solution: ");
        //for step in min.1.iter() {
        //    step.print();
        //}
    } else {
        println!("no solution found");
    }
}
