fn checksum_one(s: &str, _len: usize) -> String {
    let cksum: Vec<String> = s
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|p| {
            if p[0] == p[1] {
                "1".to_string()
            } else {
                "0".to_string()
            }
        })
        .collect::<Vec<_>>();
    cksum.join("").to_string()
}

fn checksum(s: &str) -> String {
    let mut cksum = checksum_one(s, s.len());
    while (cksum.len() % 2) == 0 {
        cksum = checksum_one(&cksum, cksum.len());
    }
    cksum
}

fn dragon_one(s: &str) -> String {
    let mut a = s.to_string();
    a.chars()
        .chain(std::iter::once('0'))
        .chain(a.chars().rev().map(|c| if c == '0' { '1' } else { '0' }))
        .collect()
}

fn dragon(s: &str, len: usize) -> String {
    let mut curve = dragon_one(s);
    while curve.len() < len {
        curve = dragon_one(&curve);
    }
    curve[0..len].to_string()
}
fn main() {
    let s = "10001110011110000";
    //let s = "10000";
    let PART2 = true;
    let d = dragon(s, if !PART2 { 272 } else { 35651584 });
    let cksum = checksum(&d);
    println!("checksum is {} which is len {}", cksum, cksum.len());
}
