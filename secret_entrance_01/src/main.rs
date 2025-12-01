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
        let mut magnitude: i32 = line.collect::<String>().parse()?;

        password += magnitude / 100;
        magnitude %= 100;

        if magnitude == 0 {
            continue;
        }

        match dir {
            'R' => {
                position += magnitude;
                if position >= 100 {
                    password += 1;
                }
            }
            'L' => {
                if position != 0 && position - magnitude <= 0 {
                    password += 1;
                }
                position -= magnitude;
            }
            _ => return Err("Invalid direction".into()),
        }

        position %= 100;

        while position < 0 {
            position += 100;
        }
    }

    println!("The password is {password}");
    Ok(())
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}
