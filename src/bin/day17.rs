use std::collections::{HashMap, VecDeque};

static PART2: bool = true;
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Node {
    pos: (i32, i32),
    path: String,
}

fn run_maze(seed: &str) {
    let open_chars = ['b', 'c', 'd', 'e', 'f'];
    let mut q: VecDeque<Node> = VecDeque::new();
    let mut pred = HashMap::new();
    q.push_back(Node {
        pos: (0, 0),
        path: String::new(),
    });
    let mut found = Node {
        pos: (0, 0),
        path: String::new(),
    };

    let mut paths = Vec::new();

    while q.len() > 0 {
        let v = q.pop_front().unwrap();

        if v.pos == (3, 3) {
            found = v.clone();
            if !PART2 {
                break;
            }
            paths.push(v.path.clone());
            let max_so_far = paths.iter().max_by_key(|p| p.len()).unwrap();
            println!("max path so far: {}", max_so_far);
            println!("length of max path so far: {}", max_so_far.len());
            continue;
        }
        //println!("at {:?}", v);
        // normally BFS would keep track of visited
        // but since the nodes will have hashes it
        // keeps changing so odds of node having same hash
        // is impossiblly low.
        for dir in &['U', 'D', 'L', 'R'] {
            if v.pos.0 == 0 && *dir == 'L'
                || v.pos.0 == 3 && *dir == 'R'
                || v.pos.1 == 0 && *dir == 'U'
                || v.pos.1 == 3 && *dir == 'D'
            {
                continue;
            }
            let tohash = seed.to_string() + &v.path;
            let hash = md5::compute(&tohash);
            let hash_str = format!("{:x}", hash);
            //println!("hash of {:?} is {}", tohash, hash_str);
            let ch;
            let next_pos;
            let next_path = v.path.clone() + dir.to_string().as_ref();
            match *dir {
                'U' => {
                    ch = hash_str.chars().nth(0).unwrap();
                    next_pos = (v.pos.0, v.pos.1 - 1);
                }
                'D' => {
                    ch = hash_str.chars().nth(1).unwrap();
                    next_pos = (v.pos.0, v.pos.1 + 1);
                }
                'L' => {
                    ch = hash_str.chars().nth(2).unwrap();
                    next_pos = (v.pos.0 - 1, v.pos.1);
                }
                'R' => {
                    ch = hash_str.chars().nth(3).unwrap();
                    next_pos = (v.pos.0 + 1, v.pos.1);
                }
                _ => unreachable!(),
            }
            if open_chars.contains(&ch) {
                //println!("v of {:?} has open door  {}", v, *dir);
                let w = Node {
                    pos: next_pos,
                    path: next_path,
                };
                q.push_back(w.clone());
                assert!(!pred.contains_key(&w));
                pred.entry(w).or_insert(v.clone());
            }
        }
    }
    if found.path.len() == 0 {
        println!("no path found");
    } else {
        println!("found path: {}", found.path);
    }
}

fn main() {
    run_maze("pvhmgsws");
}
