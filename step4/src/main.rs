use std::fs;
use std::error::Error;
use std::io::{Read, Write};

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
    InByte(u32),
}

struct MachineState<'a, I, O> where
    I: Read,
    O: Write,
{
    memory: &'a mut [u8],
    pc: usize,
    running: bool,
    i: I,
    o: O,
}

impl<'a, I: Read, O: Write> MachineState<'a, I, O> {
    fn new(memory: &mut [u8], i: I, o: O) -> MachineState<I, O> {
        MachineState {
            memory,
            pc: 0,
            running: false,
            i,
            o,
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
            4 => Instruction::InByte(self.decode_pointer()),
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
                self.o.write(&self.memory[addr as usize..addr as usize + 1]).unwrap();
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
            Instruction::InByte(dstptr) => {
                self.i.read_exact(&mut self.memory[dstptr as usize..dstptr as usize + 1]).unwrap();
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
    let i = std::io::stdin();
    let o = std::io::stdout();
    let mut machine = MachineState::new(&mut data, i.lock(), o.lock());

    machine.run();

    Ok(())
}
