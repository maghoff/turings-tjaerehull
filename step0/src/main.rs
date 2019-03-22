use std::fs;
use std::error::Error;

use structopt::StructOpt;

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

    fn run(&mut self) {
        self.running = true;
        while self.running {
            self.step();
        }
    }

    fn step(&mut self) {
        let instruction = self.memory[self.pc];
        self.pc += 1;
        match instruction {
            0 => self.running = false,
            x => panic!("Invalid opcode {:?}", x),
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
