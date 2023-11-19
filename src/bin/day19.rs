fn elephant(n: usize) {
    let mut v = Vec::new();
    for i in 1..=n {
        v.push(i);
    }

    let mut i = 0;
    let mut count = n;
    'done: loop {
        if count == 1 {
            break 'done;
        }
        if v[i] > 0 {
            let mut j = (i + 1) % n;
            while v[j] == 0 {
                j = (j + 1) % n;
            }
            v[i] += v[j];
            v[j] = 0;
            count -= 1;
        }
        i = (i + 1) % n;
    }
    let index = v.iter().position(|&x| x > 0).unwrap();
    println!("elf {}", index + 1);
}

fn elephant2(n: usize) -> usize {
    let mut v = Vec::new();
    for i in 1..=n {
        v.push(i);
    }

    let mut i = 0;
    let mut count = n;
    let mut winner = 0;
    loop {
        if count == 2 {
            println!("last {} steals from {}", v[(i) % count], v[(i + 1) % count]);
            winner = v[(i) % count];
            break;
        }

        let across_offset = count / 2;

        let j = (i + across_offset) % count;
        let thief = v[i];
        let victim = v[j];
        v.remove(j);
        if count < 4 {
            println!("{} steals from {}", thief, victim);
        }
        count -= 1;
        // only move forward if we stole from the elf after us
        // since removing elf prior to us will shift the array
        if i <= j {
            i = (i + 1) % count;
        } else {
            // if we didn't increment we still need to make sure it's within bounds
            i = i % count;
        }
        //println!("new i: {} j: {} count: {}", i, j, count);
    }
    winner
}

static PART2: bool = true;
fn main() {
    let count;
    if !PART2 {
        count = 3014387;
        elephant(count);
    } else {
        count = 3014387;
        //count = 5;
        let winner = elephant2(count);
        println!("winner: {}", winner);
    }
}
