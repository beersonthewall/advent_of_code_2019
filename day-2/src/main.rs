use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct Machine {
    pos: usize,
    end: bool,
    tape: Vec<usize>,
}

impl Machine {
    fn new(tape: Vec<usize>) -> Machine {
        Machine {
            pos: 0,
            end: false,
            tape: tape,
        }
    }

    fn next_op(&mut self) {
        let op_code = self.tape[self.pos];
        match op_code {
            1 =>{
                let fst_arg = self.tape[self.pos+1];
                let snd_arg = self.tape[self.pos+2];
                let dest = self.tape[self.pos+3];
                self.tape[dest] = self.tape[fst_arg] + self.tape[snd_arg];
            },
            2 => {
                let fst_arg = self.tape[self.pos+1];
                let snd_arg = self.tape[self.pos+2];
                let dest = self.tape[self.pos+3];
                self.tape[dest] = self.tape[fst_arg] * self.tape[snd_arg];
            },
            99 => self.end = true,
            _ => (),
        }
        self.pos += 4;
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let mut tape: Vec<usize> = contents
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    tape[1] = 12;
    tape[2] = 2;
    let mut machine = Machine::new(tape);
    while !machine.end {
        machine.next_op();
    }
    println!("Machine position zero: {}", machine.tape[0]);
}
