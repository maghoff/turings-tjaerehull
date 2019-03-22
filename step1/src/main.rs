use std::fs;
use std::error::Error;

use structopt::StructOpt;

#[repr(u8)]
enum OpCode {
    Hlt = 0,
}

struct Machine<'a> {
    memory: &'a [u8],
    pc: usize,
}

#[derive(StructOpt)]
struct Opts {
    /// name of executable in rwa2 format
    input: String,
}

fn main() -> Result<(), Box<Error>>  {
    let args = Opts::from_args();

    let data = fs::read(&args.input)?;
    println!("{:?}", &data);

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn t() {
        assert!(true);
    }
}
