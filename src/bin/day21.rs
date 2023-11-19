fn decode(s: &str, instr: &Vec<String>, reversed: bool) -> String {
    // convet to Vec<char> to make it easier to work with.
    let mut decoded = s
        .as_bytes()
        .to_vec()
        .into_iter()
        .map(|c| c as char)
        .collect::<Vec<char>>();
    for w in instr {
        let words = w.split(' ').collect::<Vec<_>>();

        if w.contains("swap position") {
            let pos_a = words[2].parse::<usize>().unwrap();
            let pos_b = words[5].parse::<usize>().unwrap();
            if !reversed {
                decoded.swap(pos_a, pos_b);
            } else {
                decoded.swap(pos_b, pos_a);
            }
        } else if w.contains("swap letter") {
            let letter_a = words[2].chars().next().unwrap();
            let letter_b = words[5].chars().next().unwrap();

            let positions_a: Vec<usize> = decoded
                .iter()
                .enumerate()
                .filter(|&(i, &c)| c == letter_a)
                .map(|(i, _)| i)
                .collect();
            let positions_b: Vec<usize> = decoded
                .iter()
                .enumerate()
                .filter(|&(i, &c)| c == letter_b)
                .map(|(i, _)| i)
                .collect();
            if positions_a.len() > 1 || positions_b.len() > 1 {
                panic!(
                    "invalid swap letter instruction: {} {} {}",
                    letter_a,
                    positions_a.len(),
                    letter_b
                );
            }
            if positions_a.len() != 1 || positions_b.len() != 1 {
                panic!(
                    "invalid swap letter instruction: {} {} {} {}",
                    letter_a,
                    positions_a.len(),
                    letter_b,
                    positions_b.len()
                );
            }
            positions_a
                .iter()
                .zip(positions_b.iter())
                .for_each(|(a, b)| {
                    if !reversed {
                        decoded.swap(*a, *b);
                    } else {
                        decoded.swap(*b, *a);
                    }
                });
        } else if w.contains("rotate left") {
            let rotl_steps = words[2].parse::<usize>().unwrap();
            if !reversed {
                decoded.rotate_left(rotl_steps);
            } else {
                decoded.rotate_right(rotl_steps);
            }
        } else if w.contains("rotate right") {
            let rotr_steps = words[2].parse::<usize>().unwrap();
            if !reversed {
                decoded.rotate_right(rotr_steps);
            } else {
                decoded.rotate_left(rotr_steps);
            }
        } else if w.contains("rotate based") {
            let rot_based_letter = words[6].chars().next().unwrap();
            let pos = decoded.iter().position(|&c| c == rot_based_letter).unwrap();
            let len = decoded.len();
            if !reversed {
                if pos >= 4 {
                    decoded.rotate_right((pos + 2) % len);
                } else {
                    decoded.rotate_right((pos + 1) % len);
                }
            } else {
                // pos is position of letter after scrambling:
                // if it less than 4, then it was rotated right index + 1 step
                // if pos was 0  1 2 3 then it would rotate to
                // 1 , 3, 5, 7 and we need to rotate left by 1, 2, 3, 4
                //
                // if it was pos 4 5 6 7 then it would rotate by
                // 4+2 = 6, rotate by 6  for 4 ends up at 2
                // 5+ 2 = 7, rotate by 7 for 5 ends up at 4
                // 6 + 2 = 8, rotate by 0 for 6 ends up 6
                // 7 + 2 = 9, rotate by 1 for 7 ends up 0

                // all these results means that the 'rotate by'
                // is reversible and don't overlap with each other
                if [1, 3, 5, 7].contains(&pos) {
                    decoded.rotate_left(((pos / 2) + 1) % len);
                } else if pos == 0 {
                    decoded.rotate_left(1);
                } else if pos == 2 {
                    decoded.rotate_left(6);
                } else if pos == 4 {
                    decoded.rotate_left(7);
                } else if pos == 6 {
                    decoded.rotate_left(0);
                } else {
                    panic!("invalid pos: {}", pos);
                }
            }
        } else if w.contains("reverse positions") {
            let rev_start = words[2].parse::<usize>().unwrap();
            let rev_end = words[4].parse::<usize>().unwrap();
            decoded[rev_start..=rev_end].reverse();
        } else if w.contains("move position") {
            let move_from = words[2].parse::<usize>().unwrap();
            let move_to = words[5].parse::<usize>().unwrap();
            if !reversed {
                let c = decoded.remove(move_from);
                decoded.insert(move_to, c);
            } else {
                let c = decoded.remove(move_to);
                decoded.insert(move_from, c);
            }
        } else {
            panic!("unknown instruction: {}", w);
        }
    }
    decoded.iter().collect::<String>()
}
fn main() {
    let word = "abcdefgh";
    let scrambled = "fbgdceah";

    // example input
    // let instr = vec![
    //     "swap position 4 with position 0".to_string(),
    //     "swap letter d with letter b".to_string(),
    //     "reverse positions 0 through 4".to_string(),
    //     "rotate left 1 step".to_string(),
    //     "move position 1 to position 4".to_string(),
    //     "move position 3 to position 0".to_string(),
    //     "rotate based on position of letter b".to_string(),
    //     "rotate based on position of letter d".to_string(),
    // ];

    let instr = include_str!("../../inputs/day21.txt")
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    // part 1
    println!("scramble {} to {}", word, decode(word, &instr, false));

    // part 2
    println!(
        "unscrambled {} to {}",
        scrambled,
        decode(
            scrambled,
            &instr.iter().rev().cloned().collect::<Vec<String>>(),
            true
        )
    );
}
