#![feature(pattern)]
use std::collections::HashSet;
use std::str::pattern::Pattern;

#[derive(Debug, Clone)]
struct Cpu {
    regs: [i32; 4],

    pc: i32,

    toggles: HashSet<i32>,
}

impl Cpu {
    fn jnz<'a, P: Pattern<'a>>(&mut self, words: &mut std::str::Split<'a, P>) {
        let x = words.next().unwrap();
        let xval;
        if let Ok(n) = x.parse::<i32>() {
            xval = n;
        } else {
            xval = self.regs[(x.chars().next().unwrap() as u32 - 'a' as u32) as usize];
        }
        let jmp_to = words.next().unwrap();
        let offset;
        if let Ok(n) = jmp_to.parse::<i32>() {
            offset = n;
        } else {
            let reg = (jmp_to.chars().next().unwrap() as u32 - 'a' as u32) as usize;
            offset = self.regs[reg];
        }
        if xval != 0 {
            //println!("jumping from {} to {}", self.pc, self.pc + offset);
            self.pc += offset;
        } else {
            self.pc += 1;
        }
    }

    fn cpy<'a, P: Pattern<'a>>(&mut self, words: &mut std::str::Split<'a, P>, toggled: bool) {
        let x = words.next().unwrap().to_string();
        //let y = (words.next().unwrap().as_bytes()[0] as u32 - 'a' as u32) as usize;
        let ych = words.next().unwrap().as_bytes()[0] as u32;
        let xch = x.chars().next().unwrap() as u32;
        if let Ok(n) = x.parse::<i32>() {
            if toggled && !(ych >= 'a' as u32 && ych <= 'd' as u32) {
                println!("skipping cpy {} {}", x, ych);
            } else {
                let y = (ych - 'a' as u32) as usize;
                self.regs[y] = n;
            }
        } else {
            if toggled && !(ych >= 'a' as u32 && ych <= 'd' as u32) {
                println!("skipping cpy {} {}", x, ych as u8 as char);
            } else {
                let x = (xch - 'a' as u32) as usize;
                let y = (ych - 'a' as u32) as usize;
                self.regs[y] = self.regs[x];
            }
        }
        self.pc += 1;
    }

    fn inc<'a, P: Pattern<'a>>(&mut self, words: &mut std::str::Split<'a, P>) {
        let reg = words.next().unwrap();
        let x = (reg.chars().next().unwrap() as u32 - 'a' as u32) as usize;
        self.regs[x] += 1;
        self.pc += 1;
    }

    fn dec<'a, P: Pattern<'a>>(&mut self, words: &mut std::str::Split<'a, P>) {
        let reg = words.next().unwrap();
        let x = (reg.as_bytes()[0] as u32 - 'a' as u32) as usize;
        self.regs[x] -= 1;
        self.pc += 1;
    }

    fn tgl<'a, P: Pattern<'a>>(&mut self, words: &mut std::str::Split<'a, P>) {
        let x = words.next().unwrap().to_string();
        let tgl_offset;
        if let Ok(n) = x.parse::<i32>() {
            tgl_offset = n;
        } else {
            let x = (x.chars().next().unwrap() as u32 - 'a' as u32) as usize;
            tgl_offset = self.regs[x];
        }
        let tgl_addr = self.pc + tgl_offset;
        if self.toggles.contains(&tgl_addr) {
            panic!("toggled instruction twice: {}", tgl_addr);
            self.toggles.remove(&tgl_addr);
        } else {
            self.toggles.insert(tgl_addr);
        }
        self.pc += 1;
    }

    fn do_instr(&mut self, s: &str, addr: i32) {
        //println!("executing: {}", s);
        let mut words = s.trim().split(' ');
        let instr = words.next().unwrap();
        if instr == "tgl" {
            if self.toggles.contains(&addr) {
                println!("tgl became inc");
                self.inc(&mut words);
            } else {
                self.tgl(&mut words);
            }
        } else if instr == "cpy" {
            if self.toggles.contains(&addr) {
                println!("cpy became jnz");
                self.jnz(&mut words);
            } else {
                self.cpy(&mut words, false);
            }
        } else if instr == "inc" {
            if self.toggles.contains(&addr) {
                println!("inc became dec");
                self.dec(&mut words);
            } else {
                self.inc(&mut words);
            }
        } else if instr == "dec" {
            if self.toggles.contains(&addr) {
                println!("dec became inc");
                self.inc(&mut words);
            } else {
                self.dec(&mut words);
            }
        } else if instr == "jnz" {
            if self.toggles.contains(&addr) {
                println!("jnz became cpy");
                self.cpy(&mut words, true);
            } else {
                self.jnz(&mut words);
            }
        } else {
            panic!("unknown instruction: {}", instr);
        }
    }
}
static PART2: bool = true;

fn main() {
    let input = include_str!("../../inputs/day23.txt").to_string();
    let input = input.trim();
    let mut cpu = Cpu {
        regs: [0; 4],
        pc: 0,
        toggles: HashSet::new(),
    };

    cpu.regs[0] = 7;

    if PART2 {
        cpu.regs[0] = 12;
    }

    while cpu.pc < input.lines().count() as i32 && cpu.pc >= 0 {
        //println!("pc: {}", cpu.pc);
        let line = input.lines().nth(cpu.pc as usize).unwrap();
        //println!("cpu: {:?}", cpu);
        cpu.do_instr(line, cpu.pc);
        //println!("cpu after: {:?}", cpu);
    }
    println!("part 1 cpu: {:?}", cpu);
}
