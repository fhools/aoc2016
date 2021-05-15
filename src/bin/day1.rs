use std::fs;
use std::collections::HashMap;



fn main() {
    const PART : u8 = 2;
    const MAX_DIR : u8 = 4;
    let mut cur_dir = 0;

    let input = fs::read_to_string("day1input.txt").unwrap();  
    //let input = "R8, R4, R4, R8";
    let toks = input.split(",").map(|s| s.trim().to_string()).collect::<Vec<String>>(); 
    println!("toks: {:?}", toks);
    let mut x = 0;
    let mut y = 0;

    let mut visits : HashMap<(i32,i32), u8> = HashMap::new();
    visits.insert((x,y), 1);
    let mut done = false;
    let mut loc : (i32, i32) = (0,0);
    for i in &toks {
        let lr = &i[0..1];
        match lr {
            "L" => {
                cur_dir = (cur_dir - 1) % MAX_DIR;
            },
            "R" => {
                cur_dir = (cur_dir + 1) % MAX_DIR;
            },
            _ => {}
        }
        let steps = i[1..].parse::<i32>().unwrap();
        println!("cur_dir = {} , steps: {}", cur_dir, steps);

        let old_x = x;
        let old_y = y; 
        match cur_dir {
            0 => {
                y -= steps;
                if PART == 2 {
                for ny in y..old_y  {
                    if visits.contains_key(&(x,ny)) {
                        done = true;
                        loc = (x, ny);
                        break;
                    }
                    println!("step: {}, {}", x, ny);
                    visits.insert((x,ny), 1);
                }
            }
            },
            1 => {
                x += steps;
                if PART == 2 { 
                for nx in old_x+1..x+1  {
                    if visits.contains_key(&(nx,y)) {
                        done = true;
                        loc = (nx, y);
                        break;
                    }
                    println!("step: {}, {}", nx, y);
                    visits.insert((nx,y), 1);
                }
                }
            },
            2 => {
                y += steps;
                if PART == 2 {
                for ny in old_y+1..y+1  {
                    if visits.contains_key(&(x,ny)) {
                        done = true;
                        loc = (x, ny);
                        break;
                    }
                    println!("step: {}, {}", x, ny);
                    visits.insert((x,ny), 1);
                }
            }
            },
            3 => {
                x -= steps;
                if PART == 2 {
                for nx in x..old_x  {
                    if visits.contains_key(&(nx,y)) {
                        done = true;
                        loc = (nx, y);
                        break;
                    }
                    println!("step: {}, {}", nx, y);
                    visits.insert((nx,y), 1);
                }
            }
            },
            _ => { panic!("bad dir");}
        }
        if done {
            break;
        }
    }
     if PART == 1 {
    println!("{}, {}", x, y);
    let distance = x.abs() + y.abs();
    println!("distance: {}", distance);
     }
     if PART == 2 {
    println!("{}, {}", loc.0, loc.1);
    let distance = loc.0.abs() + loc.1.abs();
    println!("distance: {}", distance);
     }
}