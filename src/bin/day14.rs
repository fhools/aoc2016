use std::collections::HashMap;

use md5;
static PART2: bool = true;

fn compute_hash(s: &str, hashes: &mut HashMap<String, String>) -> String {
    if !PART2 {
        format!("{:x}", md5::compute(s))
    } else {
        if let Some(hash) = hashes.get(s) {
            return hash.to_string();
        }
        let mut hash = md5::compute(s);
        for _ in 0..2016 {
            hash = md5::compute(format!("{:x}", hash));
        }
        hashes.insert(s.to_string(), format!("{:x}", hash));
        format!("{:x}", hash)
    }
}

fn contains_triple_or_fives(
    seed: &str,
    index: isize,
    ch: Option<char>,
    hash: &mut HashMap<String, String>,
) -> Option<char> {
    // if ch is None then we are looking for a triple

    let triplets = vec![
        "000", "111", "222", "333", "444", "555", "666", "777", "888", "999", "aaa", "bbb", "ccc",
        "ddd", "eee", "fff",
    ];
    let fives = vec![
        "00000", "11111", "22222", "33333", "44444", "55555", "66666", "77777", "88888", "99999",
        "aaaaa", "bbbbb", "ccccc", "ddddd", "eeeee", "fffff",
    ];
    // if we're looking for fivers
    if let Some(ch) = ch {
        let ord = usize::from_str_radix(&ch.to_string(), 16).unwrap();
        let needle = fives[ord];
        let s = format!("{}{}", seed, index);
        let hash = format!("{}", compute_hash(&s, hash));
        if hash.contains(&needle) {
            return Some(ch);
        }
    } else {
        let s = format!("{}{}", seed, index);
        let hash = format!("{}", compute_hash(&s, hash));
        let mut triplet_indices = Vec::new();
        // otherwise we are looking for triples
        for i in 0..16 {
            let needle = triplets[i];
            if let Some(index) = hash.find(needle) {
                triplet_indices.push(index);
            }
        }
        if triplet_indices.len() > 0 {
            triplet_indices.sort();
            return Some(hash.chars().nth(triplet_indices[0]).unwrap());
        }
    }
    None
}

fn main() {
    let mut index = 0;
    let mut keys = Vec::new();
    let mut triple_index = 0;
    let mut hashes = HashMap::new();
    //let seed = "abc";
    let seed = "ngcjuoqr";
    loop {
        let s = format!("{}{}", seed, index);
        let hash = format!("{}", compute_hash(&s, &mut hashes));
        if let Some(ch) = contains_triple_or_fives(seed, index, None, &mut hashes) {
            triple_index = index;
            let mut fiver = None;
            for i in 1..1001 {
                if let Some(ch) =
                    contains_triple_or_fives(seed, triple_index + i, Some(ch), &mut hashes)
                {
                    fiver = Some(ch);
                    break;
                }
            }
            if let Some(_ch) = fiver {
                println!(
                    "found key #{}: {} at index {}",
                    keys.len() + 1,
                    hash,
                    triple_index
                );
                keys.push(triple_index);
                if keys.len() == 64 {
                    break;
                }
                index = triple_index + 1;
            } else {
                index += 1;
            }
        } else {
            index += 1;
        }
    }
}
