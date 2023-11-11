use std::collections::HashMap;

use slotmap::{new_key_type, Key, SlotMap};

new_key_type! {
    pub struct NodeKey;
}

#[derive(Debug, Clone)]
enum Destination {
    Bot(usize),
    Output(usize),
}

#[derive(Debug, Clone)]
enum Instruction {
    Value {
        bot: usize,
        value: usize,
    },
    Give {
        bot: usize,
        low_dest: Destination,
        high_dest: Destination,
    },
}

fn to_instruction(s: &str) -> Instruction {
    let mut words = s.split_whitespace();
    let first = words.next().unwrap();
    if first == "value" {
        let value = words.next().unwrap().parse().unwrap();
        let bot = words.nth(3).unwrap().parse().unwrap();
        Instruction::Value { bot, value }
    } else {
        let bot = words.next().unwrap().parse().unwrap();
        let low_dest = words.nth(3).unwrap();
        let low;
        if low_dest == "bot" {
            low = Destination::Bot(words.nth(0).unwrap().parse().unwrap());
        } else {
            low = Destination::Output(words.nth(0).unwrap().parse().unwrap());
        }
        let high;
        let high_dest = words.nth(3).unwrap();
        if high_dest == "bot" {
            high = Destination::Bot(words.nth(0).unwrap().parse().unwrap());
        } else {
            high = Destination::Output(words.nth(0).unwrap().parse().unwrap());
        }
        Instruction::Give {
            bot,
            low_dest: low,
            high_dest: high,
        }
    }
}

fn get_input() -> (Vec<Instruction>, Vec<Instruction>) {
    /*let input = r"value 5 goes to bot 2
        bot 2 gives low to bot 1 and high to bot 0
        value 3 goes to bot 1
        bot 1 gives low to output 1 and high to bot 0
        bot 0 gives low to output 2 and high to output 0
        value 2 goes to bot 2";
    */
    let input = include_str!("../../inputs/day10.txt").to_string();
    let instructions = input.lines().map(to_instruction).collect::<Vec<_>>();
    let values = instructions
        .iter()
        .filter(|i| match i {
            Instruction::Value { .. } => true,
            _ => false,
        })
        .cloned()
        .collect();

    let gives = instructions
        .iter()
        .filter(|i| match i {
            Instruction::Give { .. } => true,
            _ => false,
        })
        .cloned()
        .collect();

    (values, gives)
}

// create the Node struct
#[derive(Debug, Clone)]
struct Node {
    node_id: usize,
    node_type: String,
    low_dest: NodeKey,
    high_dest: NodeKey,
    values: Vec<usize>,
}

fn push_value(
    node: NodeKey,
    value: Option<usize>,
    sm: &mut SlotMap<NodeKey, Node>,
    found: &mut bool,
) {
    let mut node = sm.get_mut(node).unwrap();
    let node_id = node.node_id;
    let low_dest = node.low_dest.clone();
    let high_dest = node.high_dest.clone();
    if let Some(value) = value {
        node.values.push(value);
    }
    println!("pushing node: {} ", node_id,);
    let mut values = node.values.clone();
    values.sort();

    if values.len() == 2 {
        if values.contains(&61) && values.contains(&17) {
            println!("part 1: node {}", node_id);
        }
        node.values.clear();
        push_value(low_dest, Some(values[0]), sm, found);
        push_value(high_dest, Some(values[1]), sm, found);
    }
}
fn dfs(node: NodeKey, sm: &mut SlotMap<NodeKey, Node>, visited: &mut Vec<NodeKey>) {
    if visited.contains(&node) {
        return;
    }
    visited.push(node);
    let node = sm.get_mut(node).unwrap().clone();
    println!("visiting node: {:?}", node);
    if node.values.contains(&5) && node.values.contains(&2) {
        println!("part 1: {}", node.node_id);
    }

    if node.node_type == "bot" {
        dfs(node.low_dest, sm, visited);
        dfs(node.high_dest, sm, visited);
    }
}

fn main() {
    let (values, gives) = get_input();
    let mut node_id_to_key = HashMap::new();
    let mut sm: SlotMap<NodeKey, Node> = SlotMap::with_key();
    let mut startnode = NodeKey::null();
    let mut visited: Vec<NodeKey> = Vec::new();

    // iterate through the values instructions and
    // create the nodes in the slotmap graph
    for v in values.iter() {
        match v {
            Instruction::Value { bot, value } => {
                let nodekey = node_id_to_key
                    .entry(*bot)
                    .or_insert_with(|| {
                        // Insert a new Node if it doesn't exist
                        sm.insert(Node {
                            node_id: *bot,
                            node_type: "bot".to_string(),
                            low_dest: NodeKey::null(),
                            high_dest: NodeKey::null(),
                            values: Vec::new(),
                        })
                    })
                    .clone();

                if let Some(node) = sm.get_mut(nodekey) {
                    node.values.push(*value);
                    println!("node: {}, values: {:?}", node.node_id, node.values);
                    if node.values.len() == 2 {
                        startnode = nodekey;
                        println!("startnode: {:?}", node);
                    }
                }
                node_id_to_key.insert(*bot, nodekey);
            }
            _ => panic!("not a value instruction"),
        }
    }

    // iterate through the gives instructions
    // and create a node for each bot and link
    // to the low and high destinations
    for g in gives.iter() {
        match g {
            Instruction::Give {
                bot,
                low_dest,
                high_dest,
            } => {
                let givekey = node_id_to_key
                    .entry(*bot)
                    .or_insert_with(|| {
                        // Insert a new Node if it doesn't exist
                        sm.insert(Node {
                            node_id: *bot,
                            node_type: "bot".to_string(),
                            low_dest: NodeKey::null(),
                            high_dest: NodeKey::null(),
                            values: Vec::new(),
                        })
                    })
                    .clone();
                let lowkey;
                match low_dest {
                    Destination::Bot(low_dest_bot) => {
                        lowkey = node_id_to_key
                            .entry(*low_dest_bot)
                            .or_insert_with(|| {
                                // Insert a new Node if it doesn't exist
                                sm.insert(Node {
                                    node_id: *low_dest_bot,
                                    node_type: "bot".to_string(),
                                    low_dest: NodeKey::null(),
                                    high_dest: NodeKey::null(),
                                    values: Vec::new(),
                                })
                            })
                            .clone();
                    }
                    Destination::Output(low_dest_output) => {
                        lowkey = node_id_to_key
                            .entry(*low_dest_output + 1000)
                            .or_insert_with(|| {
                                // Insert a new Node if it doesn't exist
                                sm.insert(Node {
                                    node_id: *low_dest_output + 1000,
                                    node_type: "output".to_string(),
                                    low_dest: NodeKey::null(),
                                    high_dest: NodeKey::null(),
                                    values: Vec::new(),
                                })
                            })
                            .clone();
                    }
                };
                let highkey;
                match high_dest {
                    Destination::Bot(high_dest_bot) => {
                        highkey = node_id_to_key
                            .entry(*high_dest_bot)
                            .or_insert_with(|| {
                                // Insert a new Node if it doesn't exist
                                sm.insert(Node {
                                    node_id: *high_dest_bot,
                                    node_type: "bot".to_string(),
                                    low_dest: NodeKey::null(),
                                    high_dest: NodeKey::null(),
                                    values: Vec::new(),
                                })
                            })
                            .clone();
                    }
                    Destination::Output(high_dest_output) => {
                        highkey = node_id_to_key
                            .entry(*high_dest_output + 1000)
                            .or_insert_with(|| {
                                // Insert a new Node if it doesn't exist
                                sm.insert(Node {
                                    node_id: *high_dest_output + 1000,
                                    node_type: "output".to_string(),
                                    low_dest: NodeKey::null(),
                                    high_dest: NodeKey::null(),
                                    values: Vec::new(),
                                })
                            })
                            .clone();
                    }
                };
                let givebot = sm.get_mut(givekey).unwrap();
                givebot.low_dest = lowkey;
                givebot.high_dest = highkey;
            }
            _ => panic!("not a give instruction"),
        }
    }

    loop {
        // check to see if any bot nodes still have 2 values
        let mut found = false;
        for (_k, v) in node_id_to_key.iter() {
            push_value(*v, None, &mut sm, &mut found);
        }

        let mut found = false;
        for (_k, v) in node_id_to_key.iter() {
            if let Some(node) = sm.get_mut(*v) {
                if node.values.len() == 2 && node.node_type == "bot" {
                    println!("node: {:?} stil has values {:?}", node.node_id, node.values);
                    found = true;
                }
            }
        }
        if !found {
            break;
        }

        visited.clear();
        dfs(startnode, &mut sm, &mut visited);
    }

    let mut product = 1;
    for (_k, v) in node_id_to_key.iter() {
        if let Some(node) = sm.get(*v) {
            if node.node_type == "output" && node.node_id >= 1000 && node.node_id < 1003 {
                println!("node: {:?} values {:?}", node.node_id, node.values);
                product *= node.values[0];
                if node.values.len() > 1 {
                    product *= node.values[1];
                }
            }
        }
    }
    println!("part 2: {}", product);
}
