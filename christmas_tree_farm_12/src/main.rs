use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
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

fn read_lines<T>(filename: T) -> io::Result<Lines<BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
