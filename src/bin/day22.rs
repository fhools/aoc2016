use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    // probably get rid of this x,y fields
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    nodes: Vec<Node>,
    empty: usize,
    goal: usize,
    width: usize,
    height: usize,
    cost_from_start: usize,
}

impl State {
    fn gen_next_state(&self) -> Vec<State> {
        // find all the way the empty node can move around
        let mut next_states = Vec::new();
        let mut empty = self.empty;

        for i in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let x = empty % self.width;
            let y = empty / self.width;
            let new_x = x as i32 + i.0;
            let new_y = y as i32 + i.1;
            if new_x < 0 || new_y < 0 || new_x >= self.width as i32 || new_y >= self.height as i32 {
                continue;
            }
            let candidate_pos = (new_x as usize) + (new_y as usize) * self.width;
            if self.nodes[candidate_pos as usize].used > self.nodes[empty].size {
                continue;
            }

            let mut new_goal = self.goal;
            let mut new_nodes = self.nodes.clone();
            new_nodes[empty].used += new_nodes[candidate_pos as usize].used;
            new_nodes[candidate_pos as usize].used = 0;
            if candidate_pos == self.goal {
                new_goal = empty;
            }
            next_states.push(State {
                nodes: new_nodes,
                empty: candidate_pos as usize,
                goal: new_goal,
                width: self.width,
                height: self.height,
                cost_from_start: self.cost_from_start + 1,
            });
        }
        next_states
    }

    fn print(&self) {
        println!("board:");
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = x + y * self.width;
                let is_full =
                    ((self.nodes[pos].used as f32) / (self.nodes[pos].size as f32)) >= 0.8;
                let is_large = self.nodes[pos].size > (self.nodes[0].size * 3);
                if pos == self.empty {
                    print!("_ ");
                } else if pos == self.goal {
                    print!("G ");
                } else if is_full && !is_large {
                    print!(". ");
                } else if is_large {
                    print!("# ");
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }

    fn distance_to_goal(&self) -> usize {
        let goal_x = self.goal % self.width;
        let goal_y = self.goal / self.width;
        let empty_x = self.empty % self.width;
        let empty_y = self.empty / self.width;
        let dx = (goal_x as i32 - empty_x as i32).abs() as usize;
        let dy = (goal_y as i32 - empty_y as i32).abs() as usize;
        goal_x + goal_y + empty_x + empty_y + dx + dy
    }

    fn solve(&self) -> usize {
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();
        let mut path = Vec::new();
        queue.push(self.clone());
        // we're trying A* search, but two thing are wrong
        // 1. the next move popped off the queue is not a valid next move
        // 2. the distance_to_goal heuristic function is not optimal
        // I think this problem may not be solvable with A* search
        while let Some(pos) = queue.pop() {
            println!("visiting board:");
            pos.print();
            path.push(pos.clone());

            if pos.goal == 0 {
                return path.len();
            }
            if visited.contains(&(pos.empty, pos.goal)) {
                continue;
            }
            visited.insert((pos.empty, pos.goal));
            for next_state in pos.gen_next_state() {
                queue.push(next_state);
            }
        }
        unreachable!("could not find solution");
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        let other_cost = other.cost_from_start + 3 * other.distance_to_goal();
        let self_cost = self.cost_from_start + 3 * self.distance_to_goal();
        other_cost.cmp(&self_cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        let other_cost = other.cost_from_start + 3 * other.distance_to_goal();
        let self_cost = self.cost_from_start + 3 * self.distance_to_goal();
        Some(other_cost.cmp(&self_cost))
    }
}

fn parse(s: &str) -> Node {
    let s = s.split_whitespace().collect::<Vec<_>>();
    let x_pos = s[0].find('x').unwrap();
    let pos = &s[0][x_pos..].split('-').collect::<Vec<_>>();
    let x = pos[0][1..].parse::<usize>().unwrap();
    let y = pos[1][1..].parse::<usize>().unwrap();
    let size = s[1][..s[1].len() - 1].parse::<usize>().unwrap();
    let used = s[2][..s[2].len() - 1].parse::<usize>().unwrap();
    let avail = s[3][..s[3].len() - 1].parse::<usize>().unwrap();
    Node {
        x: x,
        y: y,
        size: size,
        used: used,
        avail: avail,
    }
}

fn part1(nodes: &Vec<Node>) -> usize {
    let mut viable = 0;
    for a in nodes {
        for b in nodes {
            if a.used == 0 {
                continue;
            }
            if a == b {
                continue;
            }
            if a.used <= b.avail {
                viable += 1;
            }
        }
    }
    viable
}
fn main() {
    let s = include_str!("../../inputs/day22.txt");
    let mut lines = s.lines().collect::<Vec<_>>();
    lines.remove(0);
    lines.remove(0);
    //     let s = r"/dev/grid/node-x0-y0   10T    8T     2T   80%
    // /dev/grid/node-x0-y1   11T    6T     5T   54%
    // /dev/grid/node-x0-y2   32T   28T     4T   87%
    // /dev/grid/node-x1-y0    9T    7T     2T   77%
    // /dev/grid/node-x1-y1    8T    0T     8T    0%
    // /dev/grid/node-x1-y2   11T    7T     4T   63%
    // /dev/grid/node-x2-y0   10T    6T     4T   60%
    // /dev/grid/node-x2-y1    9T    8T     1T   88%
    // /dev/grid/node-x2-y2    9T    6T     3T   66%
    // ";
    //let mut lines = s.lines().collect::<Vec<_>>();
    //lines.remove(0);

    let mut nodes = lines.iter().map(|l| parse(l)).collect::<Vec<_>>();
    // for n in &nodes {
    //     println!("{:?}", n);
    // }
    // println!();
    let width = 30;
    //let height = 3;
    nodes.sort_by(|a, b| {
        let a_pos = a.x + a.y * width;
        let b_pos = b.x + b.y * width;
        a_pos.cmp(&b_pos)
    });

    let mut empty_pos = 0;
    for n in &mut nodes {
        if (n.used as f32 / n.size as f32) < 0.2 {
            empty_pos = n.x + n.y * width;
        }
    }
    println!("empty pos: {}", empty_pos);

    println!("part1: {}", part1(&nodes));
    println!("num nodes: {}", nodes.len());

    let state = State {
        nodes: nodes,
        empty: empty_pos,
        goal: 29,
        width: 30,
        height: 35,
        cost_from_start: 0,
    };
    state.print();
    // let next_states = state.gen_next_state();
    // for s in next_states {
    //     s.print();
    // }
    // note code solution: just print out the board and count it.
    // note: when i am in top right corner i am 45 steps.
    // when i am in top left corner i am 184 steps. then
    // moving G one more time is 185.
    //println!("part2: {}", state.solve());
}
