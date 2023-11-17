static PART2: bool = true;
struct Disc {
    num_id: i32,
    positions: i32,
    start: i32,
}

fn slot_at(t: i32, slot: &Disc) -> i32 {
    // println!(
    //     "checking disc {} at time {}. it is at slot {}",
    //     slot.num_id + 1,
    //     t,
    //     (slot.start + t) % slot.positions
    // );
    (slot.start + t) % slot.positions
}

fn main() {
    let mut slots = vec![
        Disc {
            num_id: 0,
            positions: 13,
            start: 1,
        },
        Disc {
            num_id: 1,
            positions: 19,
            start: 10,
        },
        Disc {
            num_id: 2,
            positions: 3,
            start: 2,
        },
        Disc {
            num_id: 3,
            positions: 7,
            start: 1,
        },
        Disc {
            num_id: 4,
            positions: 5,
            start: 3,
        },
        Disc {
            num_id: 5,
            positions: 17,
            start: 5,
        },
    ];

    if PART2 {
        slots.push(Disc {
            num_id: 6,
            positions: 11,
            start: 0,
        });
    }

    for t in 1.. {
        let mut all_matched = true;
        let first_pos = slot_at(t, &slots[0]);
        //println!("disk 1 is at pos {} at t {}", first_pos, t);
        for (i, s) in slots.iter().skip(1).enumerate() {
            if slot_at(t + s.num_id, s) != first_pos {
                all_matched = false;
                break;
            }
        }
        if all_matched {
            println!("slots lines up at t: {}", t - 1);
            break;
        }
    }
}
