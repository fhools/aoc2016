/* read characters from a string, if
encounter a '(CxD)' then read C characters
after that and append it to the output
D times, otherwise just append the character
to the output. NOTE: all of this was written by copilot!
all I did was describe the problem above. it must know
all about advent of code or something!
the code is pretty good too.
*/
fn decompress(s: &str) -> String {
    let mut output = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '(' {
            let mut marker = String::new();
            while let Some(c) = chars.next() {
                if c == ')' {
                    break;
                }
                marker.push(c);
            }
            let mut marker = marker.split('x');
            let chars_to_repeat: usize = marker.next().unwrap().parse().unwrap();
            let times_to_repeat: usize = marker.next().unwrap().parse().unwrap();
            let mut repeated = String::new();
            for _ in 0..chars_to_repeat {
                repeated.push(chars.next().unwrap());
            }
            for _ in 0..times_to_repeat {
                output.push_str(&repeated);
            }
        } else {
            output.push(c);
        }
    }
    output
}

// the following doesn't work uses too much memory!
fn nested_decompres(s: &str) {
    let mut s = s.to_string();
    while s.contains('(') {
        s = decompress(&s);
    }
    println!("part 2 decompressed length: {}", s.len());
}

// the following is a recursive solution that works by
// decompressing substrings, each nested call will
// work on a string that is shorter.
fn compute_just_length_nested(s: &str) -> usize {
    let mut length = 0;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '(' {
            let mut marker = String::new();
            while let Some(c) = chars.next() {
                if c == ')' {
                    break;
                }
                marker.push(c);
            }
            let mut marker = marker.split('x');
            let chars_to_repeat: usize = marker.next().unwrap().parse().unwrap();
            let times_to_repeat: usize = marker.next().unwrap().parse().unwrap();
            let mut repeated = String::new();
            for _ in 0..chars_to_repeat {
                repeated.push(chars.next().unwrap());
            }
            length += times_to_repeat * compute_just_length_nested(&repeated);
        } else {
            length += 1;
        }
    }
    println!("part 2 decompressed length: {}", length);
    length
}

fn main() {
    let input = include_str!("../../inputs/day9.txt");
    let input = input.trim();
    let decompressed = decompress(input);
    println!("decompressed length: {}", decompressed.len());
    //nested_decompres(input);
    println!("decompressed length: {}", compute_just_length_nested(input));
}
