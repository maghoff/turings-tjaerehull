use std::fs;
use std::error::Error;
use std::io::Write;

use byteorder::{ByteOrder, LittleEndian};
use structopt::StructOpt;

enum Instruction {
    Hlt,
    OutByte(u32),
    BranchIfPlus {
        jmpptr: u32,
        srcptr: u32,
    },
    Sub {
        dstptr: u32,
        srcptr: u32,
    },
}

struct MachineState<'a> {
    memory: &'a mut [u8],
    pc: usize,
    running: bool,
}

impl<'a> MachineState<'a> {
    fn new(memory: &mut [u8]) -> MachineState {
        MachineState {
            memory,
            pc: 0,
            running: false,
        }
    }

    fn decode_pointer(&mut self) -> u32 {
        let result = LittleEndian::read_u32(&self.memory[self.pc..self.pc + 4]);
        self.pc += 4;
        result
    }

    fn read_instruction(&mut self) -> Instruction {
        let opcode = self.memory[self.pc];
        self.pc += 1;
        match opcode {
            0 => Instruction::Hlt,
            1 => Instruction::OutByte(self.decode_pointer()),
            2 => Instruction::BranchIfPlus {
                jmpptr: self.decode_pointer(),
                srcptr: self.decode_pointer(),
            },
            3 => Instruction::Sub {
                dstptr: self.decode_pointer(),
                srcptr: self.decode_pointer(),
            },
            x => panic!("Invalid opcode {:?}", x),
        }
    }

    fn read(&self, ptr: u32) -> u8 {
        self.memory[ptr as usize]
    }

    fn step(&mut self) {
        match self.read_instruction() {
            Instruction::Hlt => self.running = false,
            Instruction::OutByte(addr) => {
                std::io::stdout().lock().write(&self.memory[addr as usize..addr as usize + 1]).unwrap();
            }
            Instruction::BranchIfPlus { jmpptr, srcptr } => {
                if self.memory[srcptr as usize] < 128 {
                    self.pc = jmpptr as usize;
                }
            }
            Instruction::Sub { dstptr, srcptr } => {
                let src = self.read(srcptr);
                let dst = self.read(dstptr);
                self.memory[dstptr as usize] = dst.wrapping_sub(src);
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

    let mut data = fs::read(&args.input)?;
    let mut machine = MachineState::new(&mut data);

    machine.run();

    Ok(())
}
