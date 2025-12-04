use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = read_lines("input.txt")?;
    while let Some(Ok(line)) = lines.next() {
        println!("{line}");
    }

    Ok(())
}

fn read_lines<T>(filepath: T) -> Result<io::Lines<BufReader<File>>, Box<dyn Error>>
where
    T: AsRef<Path>,
{
    let file = File::open(filepath)?;

    Ok(io::BufReader::new(file).lines())
}
