static PART2: bool = true;
fn scan_blocks() {
    let s = include_str!("../../inputs/day20.txt");
    let mut lines = s.lines().collect::<Vec<_>>();

    // blocks contains list of blocked range tuples (u32, u32)
    let mut blocked = lines
        .iter()
        .map(|l| {
            let l = l.split('-').collect::<Vec<_>>();
            let l = (l[0].parse::<u32>().unwrap(), l[1].parse::<u32>().unwrap());
            assert!(l.0 <= l.1, "invalid range: {:?}", l);
            l
        })
        .collect::<Vec<_>>();

    println!("lines: {}", lines.len());
    // sort the starts of the block in preparation for merging
    blocked.sort_by(|a, b| a.0.cmp(&b.0));

    for (i, m) in (&blocked).iter().enumerate() {
        println!("blocked {} : {:?}", i, m);
    }
    println!();
    // merge the entries
    // contains the merged entries. first entry is the first blocked range
    let mut merged = vec![blocked[0]];

    // index of current block we are merging with merge
    let mut i = 1;

    // for each blocked ip in block
    while i < blocked.len() {
        // if the current block starts before the last block in merged, then we need to merge
        // with the current block, and last merged block end is updated to current block end
        if blocked[i].0 <= merged.last().unwrap().1 {
            if blocked[i].1 >= merged.last().unwrap().1 {
                println!(
                    "merging: {:?} with {:?}",
                    merged.last().unwrap(),
                    blocked[i]
                );
                merged.last_mut().unwrap().1 = blocked[i].1;
            }
            i += 1;
            continue;
        }

        // if the current block starts right before last merged block ends, we can update the
        // last merged block end to current block end
        if blocked[i].0 == merged.last().unwrap().1 + 1 {
            println!(
                "updating last merged block with {:?} was {:?} ",
                blocked[i],
                merged.last().unwrap()
            );
            merged.last_mut().unwrap().1 = blocked[i].1;
            i += 1;
            continue;
        }

        // if the current block stars after the last block in merged, then we can just add it
        if blocked[i].0 > merged.last().unwrap().1 + 1 {
            println!(
                "adding new last merge block: {:?}, previous: {}",
                blocked[i],
                merged.last().unwrap().1
            );
            merged.push(blocked[i]);
            i += 1;
            continue;
        }

        unreachable!(
            "did not handle a case: current block: {:?}, last merged block: {:?}",
            blocked[i],
            merged.last().unwrap()
        );
    }

    for (i, m) in (&merged).iter().enumerate() {
        println!("merge {} : {:?}", i, m);
    }

    // find the first non-blocked ip.
    let mut allowed_count = 0;

    for (i, m) in (&merged).windows(2).enumerate() {
        if i == 0 {
            if m[0].0 > 0 {
                println!("first non-blocked ip: {}", 0);
                break;
            }
        }

        if m[1].1 > m[0].0 + 1 {
            println!("m[0]: {:?}, m[1]: {:?}", m[0], m[1]);

            if !PART2 {
                println!("first non-blocked ip: {}", m[0].1 + 1);
                break;
            } else {
                allowed_count += m[1].0 - m[0].1 - 1;
            }
        }
    }
    if PART2 {
        println!("allowed count: {}", allowed_count);
    }
}
fn main() {
    scan_blocks();
}
