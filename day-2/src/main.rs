use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Machine {
    ptr: usize,
    end: bool,
    mem: Vec<usize>,
}

impl Machine {
    fn new(mem: Vec<usize>) -> Machine {
        Machine {
            ptr: 0,
            end: false,
            mem: mem,
        }
    }

    fn next_op(&mut self) {
        let op_code = self.mem[self.ptr];
        match op_code {
            1 =>{
                let fst_arg = self.mem[self.ptr+1];
                let snd_arg = self.mem[self.ptr+2];
                let dest = self.mem[self.ptr+3];
                self.mem[dest] = self.mem[fst_arg] + self.mem[snd_arg];
            },
            2 => {
                let fst_arg = self.mem[self.ptr+1];
                let snd_arg = self.mem[self.ptr+2];
                let dest = self.mem[self.ptr+3];
                self.mem[dest] = self.mem[fst_arg] * self.mem[snd_arg];
            },
            99 => self.end = true,
            _ => (),
        }
        self.ptr += 4;
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let mem: Vec<usize> = contents
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    let mut memory = mem.to_vec();
    memory[1] = 12;
    memory[2] = 2;
    let mut machine = Machine::new(memory);
    while !machine.end {
        machine.next_op();
    }
    println!("Machine position zero: {}", machine.mem[0]);

    let answer = 19690720;

    for i in 0..101 {
        for j in 0..101 {
            let mut memory = mem.to_vec();
            memory[1] = i;
            memory[2] = j;
            let mut mchn = Machine::new(memory);
            while !mchn.end {
                mchn.next_op();
            }
            if mchn.mem[0] == answer {
                println!("100 * {} + {} = {}", i, j, 100 * i + j);
                println!("Output: {}", mchn.mem[0]);
                break;
            }
        }
    }

    
}
