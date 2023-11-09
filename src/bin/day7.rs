#![feature(array_windows)]

fn is_abba_p1(s:&str) -> bool {
    for w in s.as_bytes().array_windows::<4>() {
        if w[0] != w[1] && 
            w[0] == w[3] &&
            w[1] == w[2] {
                return true;
            }
    }
    return false;
}

fn is_aba_with_bab(s:&str, t: &str) -> bool {
    for w in s.as_bytes().array_windows::<3>() {
        if w[0] != w[1] && 
            w[0] == w[2] {
                for x in t.as_bytes().array_windows::<3>() {
                    if x[0] != x[1] && 
                        x[0] == x[2] && 
                            w[0] == x[1] && 
                            w[1] == x[0] {
                        return true;
                    }
                }
            }
    }
    return false;
}

fn process_input_p1(s: &str, is_abba: fn(s:&str) -> bool) -> bool {
    let mut i = 0;
    let mut begin = 0;
    let mut inbracket = false;
    let mut found_abba = false;
    while i < s.len() {
        if !inbracket {
            if s.chars().nth(i).unwrap() == '[' {
               inbracket = true;
               if i >= begin && is_abba(&s[begin..i]) {
                       found_abba = true;
               }
               i += 1;
               begin = i;
            }  else {
                i+= 1;
            }
        } else if s.chars().nth(i).unwrap() == ']' {
            inbracket = false;
            if i >= begin && is_abba(&s[begin..i]) {
                return  false;
            }
            i += 1;
            begin = i;
        } else {
            i += 1;
        }
    }
    if i > begin &&  is_abba(&s[begin..i]) {
        found_abba = true;
    }
    found_abba
}

fn process_input_p2(s: &str) -> bool {
    let mut i = 0;
    let mut begin = 0;
    let mut inbracket = false;
    let mut outside_strs = Vec::new();
    let mut inside_strs = Vec::new();
    while i < s.len() {
        if !inbracket {
            if s.chars().nth(i).unwrap() == '[' {
               inbracket = true;
               if i >= begin {
                   outside_strs.push(&s[begin..i]);
               }
               i += 1;
               begin = i;
            }  else {
                i+= 1;
            }
        } else if s.chars().nth(i).unwrap() == ']' {
            inbracket = false;
            if i >= begin {
                inside_strs.push(&s[begin..i]);
            }
            i += 1;
            begin = i;
        } else {
            i += 1;
        }
    }

    if i > begin {
        outside_strs.push(&s[begin..i]); 
    }
    for outside_str in &outside_strs {
        for inside_str in &inside_strs {
            if is_aba_with_bab(outside_str, inside_str) {
                return true;
            }
        }
    }
    return false;

}

fn part1() {
    let input = include_str!("../../inputs/day7p1.txt");
    let num_tls = input.lines().filter(|s| process_input_p1(s, is_abba_p1)).collect::<Vec<_>>().len();
    println!("part 1 supports tls: {}", num_tls);
}

fn part2() {
    let input = include_str!("../../inputs/day7p1.txt");
    let num_tls = input.lines().filter(|s| process_input_p2(s)).collect::<Vec<_>>().len();
    println!("part 2 supports tls: {}", num_tls);
}
fn main() {
    part1();
    part2();
}
