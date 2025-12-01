use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut position = 50;
    let mut password = 0;
    let mut lines = read_lines("input.txt")?;
    while let Some(Ok(line)) = lines.next() {
        let mut line = line.chars();

        let dir = line.next().ok_or("Could not parse line")?;
        let magnitude: i32 = line.collect::<String>().parse()?;

        match dir {
            'R' => position += magnitude,
            'L' => position -= magnitude,
            _ => return Err("Invalid direction".into()),
        }

        position %= 100;

        while position < 0 {
            position += 100;
        }

        if position == 0 {
            password += 1;
        }
    }

    println!("The password is {password}");
    Ok(())
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}
