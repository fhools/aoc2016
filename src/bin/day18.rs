fn gen_row(s: &str) -> String {
    let ss: String = "."
        .to_string()
        .chars()
        .chain(s.chars())
        .chain(".".to_string().chars())
        .collect();

    let mut next = String::new();
    for w in ss.as_bytes().windows(3) {
        // traps ^^. , .^^, ^.., ..^  (6, 3, 4, 1)
        // this means that not trap is ..., ^^^, .^. , ^.^, (0, 7, 2, 5)
        let prefix = std::str::from_utf8(&*w).unwrap();
        if ["^^.", ".^^", "^..", "..^"].contains(&prefix) {
            next.push('^');
        } else {
            next.push('.');
        }
    }
    next
}
static PART2: bool = true;
fn main() {
    //let mut r = "..^^.".to_string();
    let mut r = "^.^^^..^^...^.^..^^^^^.....^...^^^..^^^^.^^.^^^^^^^^.^^.^^^^...^^...^^^^.^.^..^^..^..^.^^.^.^......."
    .to_string();
    let mut rows = Vec::new();
    for _ in 0..(if !PART2 { 40 } else { 400000 }) {
        //println!("{}", r);
        rows.push(r.clone());
        r = gen_row(&r);
    }
    let safe_count = rows
        .iter()
        .flat_map(|s| s.chars())
        .filter(|&c| c == '.')
        .count();

    println!("safe spots: {}", safe_count);
}
