use std::fs;
use std::error::Error;
use std::io::Write;

use structopt::StructOpt;

enum Instruction {
    Hlt,
    OutByte(u32),
}

struct MachineState<'a> {
    memory: &'a [u8],
    pc: usize,
    running: bool,
}

impl<'a> MachineState<'a> {
    fn new(memory: &[u8]) -> MachineState {
        MachineState {
            memory,
            pc: 0,
            running: false,
        }
    }

    fn decode_pointer(&mut self) -> u32 {
        let mut result = 0;
        for i in 0..4 {
            result += (self.memory[self.pc] as u32) << (i * 8);
            self.pc += 1;
        }
        result
    }

    fn read_instruction(&mut self) -> Instruction {
        let opcode = self.memory[self.pc];
        self.pc += 1;
        match opcode {
            0 => Instruction::Hlt,
            1 => Instruction::OutByte(self.decode_pointer()),
            x => panic!("Invalid opcode {:?}", x),
        }
    }

    fn step(&mut self) {
        match self.read_instruction() {
            Instruction::Hlt => self.running = false,
            Instruction::OutByte(addr) => {
                std::io::stdout().lock().write(&self.memory[addr as usize..addr as usize + 1]).unwrap();
            }
        }
    }

    fn run(&mut self) {
        self.running = true;
        while self.running {
            self.step();
        }
    }
}

#[derive(StructOpt)]
struct Opts {
    /// name of executable in rwa2 format
    input: String,
}

fn main() -> Result<(), Box<Error>>  {
    let args = Opts::from_args();

    let data = fs::read(&args.input)?;
    let mut machine = MachineState::new(&data);

    machine.run();

    Ok(())
}
