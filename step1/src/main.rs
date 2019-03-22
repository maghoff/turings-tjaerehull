use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<Error>>  {
    println!("Hello, world!");

    let data = fs::read("input")?;
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
