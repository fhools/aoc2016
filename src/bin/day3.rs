pub fn input_p1() -> String {
    let input = include_str!("../../inputs/day3p1.txt");
    input.to_string()
}

pub fn to_numbers(s: &str) -> Vec<Vec<i32>> {
    s.lines().map(|l| {
        l.split_ascii_whitespace()
         .filter_map(|s| s.parse().ok())
         .collect()
    }).collect()
}

pub fn is_triangle(lens: &Vec<i32>) -> Option<()> {
    if (lens[0] < (lens[1] + lens[2])) && 
       (lens[1] < (lens[0] + lens[2])) && 
       (lens[2] < (lens[0] + lens[1])) {
                Some(()) 
            } else {
                None
            }
}

pub fn transpose_input(v: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for i in 0..3 {
        let mut j = 0;
        while j < v.len() {
           let triangle = vec![v[j][i], v[j+1][i], v[j+2][i]];
           result.push(triangle);
           j += 3;
        }
    }
    result
}

pub fn part1() {
    let input = input_p1();
    let lengths = to_numbers(&input);
    println!("{} entries", lengths.len());
    let possible: usize = lengths.iter()
        .filter_map(is_triangle)
        .collect::<Vec<()>>()
        .len();
    println!("possible triangles: {}", possible)
}

pub fn part2() {
    let input = input_p1();
    let lengths = to_numbers(&input);
    let lengths = transpose_input(&lengths);
    let possible: usize = lengths.iter()
        .filter_map(is_triangle)
        .collect::<Vec<()>>()
        .len();
    println!("possible triangles: {}", possible)
}

fn main() {
    // part1();
    part2();
}

mod tests {
    use super::*;

    #[test]
    pub fn test_day3() {
        let s = input_p1();
        assert!(s.len() > 0);
    }

    #[test]
    pub fn test_conversio() {
        let input = input_p1();
        let lengths = to_numbers(&input);
        for l in lengths {
            println!("length: {:?}", l);
        }
    }
}
