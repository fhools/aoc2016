use std::collections::HashMap;

pub fn input_p1() -> String {
    include_str!("../../inputs/day4p1.txt").to_string()
}

pub fn is_real(input: &str) -> Option<i32> {
    let input_no_checksum = &input[0..input.find(|c: char | c.is_ascii_digit()).unwrap()];
    let given_checksum = input[input.find("[").unwrap()+1 .. input.find("]").unwrap()].to_string();
    let sector: i32 = input[input.find(|c: char| c.is_ascii_digit()).unwrap() .. input.find("[").unwrap()]
        .parse::<i32>().unwrap();
    let mut map: HashMap<String, (String, usize)> = HashMap::new(); 
    for c in input_no_checksum.chars().filter(|c| *c != '-') {
        map.entry(c.to_string())
            .and_modify(|e| *e = (e.0.clone(), e.1 + 1))
            .or_insert((c.to_string(), 0));
    }
    //println!("{}", input_no_checksum);
    let mut values : Vec<(String, usize)> =  map.values().cloned().collect();
    values.sort_by(|a, b| 
                   if a.1 == b.1 {
                      a.0.cmp(&b.0)
                   } else {
                       b.1.cmp(&a.1)
                   });
    let k = values.into_iter().take(5).map(|a| a.0).collect::<String>();
    if k == given_checksum {
        Some(sector)
    } else {
        None
    }
}

pub fn decrypt_string(input: &str) -> (String, i32) {
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    let sector: i32 = input[input.find(|c: char| c.is_ascii_digit()).unwrap() .. input.find("[").unwrap()]
        .parse::<i32>().unwrap();
    let input = &input[0..input.find(|c: char | c.is_ascii_digit()).unwrap()];
    let decrypted =  input.chars().map(|c| {
        if c == '-' {
            " ".to_string()
        } else {
            let pos = alpha.find(c).unwrap(); 
            let decrypt_pos = (pos + (sector as usize % 26)) % 26;
            let decrypt_ch = alpha.chars().nth(decrypt_pos).unwrap();
            decrypt_ch.to_string()
        }
    }).collect();
    (decrypted, sector)
}
pub fn part1() {
    // part 1
    let input = input_p1();
    let sum_of_real_sectors: i32 = input.lines()
        .filter_map(is_real)
        .sum();
    println!("part1: {}", sum_of_real_sectors);


}

pub fn part2() {
    let input = input_p1();
    input.lines()
        .filter(|s| is_real(s).is_some())
        .map(decrypt_string)
        .filter(|s| s.0.contains("north"))
        .for_each(|s| println!("found 'north': {:?}", s));
}

fn main() {
    part1();
    part2();
}
