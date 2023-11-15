use core::panic;
use std::collections::{HashMap, HashSet, VecDeque};

static FAV_NUM: i32 = 1358;
fn is_pos_wall(x: i32, y: i32, fav_num: i32) -> bool {
    let n = x * x + 3 * x + 2 * x * y + y + y * y + fav_num;
    let n = n.count_ones();
    n % 2 == 1
}

#[allow(dead_code)]
fn print_grid(fav_num: i32) {
    for y in 0..10 {
        for x in 0..10 {
            if is_pos_wall(x, y, fav_num) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
// instead of creating the graph data structure ahead of time
// we just generate it's neighbors on the fly as we do
// a BFS shortest path search. since this is
// unweighted graph this works fine.
fn path_to(start: (i32, i32), dest: (i32, i32), fav_num: i32) -> Vec<(i32, i32)> {
    let mut visited = HashSet::new();
    let mut pred = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((1, 1));
    let mut distances: HashMap<(i32, i32), i32> = HashMap::new();
    assert!(
        !is_pos_wall(dest.0, dest.1, fav_num),
        "destination is a wall!"
    );
    distances.entry(start).or_insert(0);
    while q.len() != 0 {
        let pos = q.pop_front().unwrap();
        //println!("pos: {:?}", pos);
        if pos == dest {
            break;
        }
        visited.insert(pos);
        for w_offset in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let w = (pos.0 + w_offset.0, pos.1 + w_offset.1);
            if w.0 < 0 || w.1 < 0 {
                continue;
            }
            if !is_pos_wall(w.0, w.1, fav_num) && !visited.contains(&w) {
                q.push_back(w);
                pred.entry(w).or_insert(pos);

                // read then write, trying to nest .get() call with .entry() call
                // results in a borrow checker error.
                let w_dist = distances.get(&pos).unwrap() + 1;
                distances.entry(w).or_insert(w_dist);
            }
        }
    }

    let mut path = Vec::new();
    let mut pos = dest;
    let mut steps = 0;
    while pos != start {
        //println!("dest pos: {:?}", pos);
        path.push(pos);
        if pred.contains_key(&pos) {
            pos = pred[&pos];
        } else {
            panic!("no path to start found");
        }
        steps += 1;
        if steps > 100 {
            panic!("too many steps");
        }
    }

    // we get lucky that the ending position is more than 50 steps away.
    // otherwise we would loop through a large grid and count the number
    // of positions within 50. but this is not necessary for our input.
    let num_with_50_steps = distances
        .iter()
        .filter(|(_, &dist)| dist <= 50)
        .collect::<Vec<_>>()
        .len();
    println!("num within 50 steps: {}", num_with_50_steps);
    path.push(start);
    path.reverse();
    path
}
fn main() {
    let path = path_to((1, 1), (31, 39), FAV_NUM);
    println!("steps to dest: {:?}", path.len() - 1);
}
