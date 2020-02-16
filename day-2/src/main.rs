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

    fn run(&mut self, fst: usize, snd: usize) -> usize {
        self.mem[1] = fst;
        self.mem[2] = snd;

        loop {
            self.next_op();
            if self.end {
                break;
            }
        }
        self.mem[0] 
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

    let mut machine = Machine::new(mem.to_vec());
    machine.run(12, 2);
    println!("Machine position zero: {}", machine.mem[0]);

    let answer = 19690720;

    for i in 0..101 {
        for j in 0..101 {
            let mut mchn = Machine::new(mem.to_vec());
            mchn.run(i, j);
            if mchn.mem[0] == answer {
                println!("100 * {} + {} = {}", i, j, 100 * i + j);
                println!("Output: {}", mchn.mem[0]);
                break;
            }
        }
    }
}
