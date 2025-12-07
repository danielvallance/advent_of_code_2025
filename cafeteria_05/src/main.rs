use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_lines("input.txt")?;

    for line in lines {
        let line = line?;
        println!("{line}");
    }
    Ok(())
}

fn read_lines<T>(filename: T) -> Result<Lines<BufReader<File>>, Box<dyn Error>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(BufReader::new(file).lines())
}
