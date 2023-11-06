use std::collections::HashMap;

fn get_input() -> String {
    include_str!("../../inputs/day6p1.txt").into()
}

fn compute_freq_columns(part1: bool) -> String {
    let mut data: Vec<HashMap<char, (char,usize)>> = Vec::new();
    for l in get_input().lines() {
        for (i, c) in l.chars().enumerate() {
            if data.len() <= i + 1 {
                data.push(HashMap::new());
            } 
            data[i].entry(c)
                .and_modify(|e| *e = (e.0, e.1 + 1))
                .or_insert((c, 0));
        }
    }
    let mut s: String = String::new();
    for (i, v) in data.into_iter().enumerate() {
        let mut values : Vec<(char, usize)> = v.values().cloned().collect();
        values.sort_by(|a ,b| {
                       if !part1 {
                           a.1.cmp(&b.1)
                       } else {
                           b.1.cmp(&a.1)
                       }
        });
        if !values.is_empty() {
            println!("column {} c:{} count:{}", 
                     i, values[0].0, values[0].1);
            s.push_str(&values[0].0.to_string());
        }
    }
    s
}

fn main() {
    let part1 = compute_freq_columns(true);
    let part2 = compute_freq_columns(false);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
