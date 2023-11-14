#[derive(Debug, Clone, Copy)]
struct Cpu {
    regs: [i32; 4],

    pc: i32,
}

impl Cpu {
    fn do_instr(&mut self, s: &str) {
        println!("executing: {}", s);
        let mut words = s.trim().split(' ');
        let instr = words.next().unwrap();
        if instr == "cpy" {
            let x = words.next().unwrap().to_string();
            let y = (words.next().unwrap().as_bytes()[0] as u32 - 'a' as u32) as usize;
            if let Ok(n) = x.parse::<i32>() {
                self.regs[y] = n;
            } else {
                let x = (x.chars().next().unwrap() as u32 - 'a' as u32) as usize;
                self.regs[y] = self.regs[x];
            }
            self.pc += 1;
        } else if instr == "inc" {
            let reg = words.next().unwrap();
            let x = (reg.chars().next().unwrap() as u32 - 'a' as u32) as usize;
            self.regs[x] += 1;
            self.pc += 1;
        } else if instr == "dec" {
            let reg = words.next().unwrap();
            let x = (reg.as_bytes()[0] as u32 - 'a' as u32) as usize;
            self.regs[x] -= 1;
            self.pc += 1;
        } else if instr == "jnz" {
            let x = words.next().unwrap();
            let xval;
            if let Ok(n) = x.parse::<i32>() {
                xval = n;
            } else {
                xval = self.regs[(x.chars().next().unwrap() as u32 - 'a' as u32) as usize];
            }
            let offset = words.next().unwrap().parse::<i32>().unwrap();
            if xval != 0 {
                println!("jumping from {} to {}", self.pc, self.pc + offset);
                self.pc += offset;
            } else {
                self.pc += 1;
            }
        } else {
            panic!("unknown instruction: {}", instr);
        }
    }
}
static PART2: bool = true;

fn main() {
    let input = include_str!("../../inputs/day12p1.txt").to_string();
    let input = input.trim();
    let mut cpu = Cpu {
        regs: [0; 4],
        pc: 0,
    };
    if PART2 {
        cpu.regs[2] = 1;
    }

    while cpu.pc < input.lines().count() as i32 {
        println!("pc: {}", cpu.pc);
        let line = input.lines().nth(cpu.pc as usize).unwrap();
        cpu.do_instr(line);
    }
    println!("part 1 cpu: {:?}", cpu);
}
