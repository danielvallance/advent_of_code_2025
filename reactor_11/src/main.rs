use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

#[derive(Debug)]
enum MyError {
    InvalidFileFormat,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid file format")
    }
}

impl std::error::Error for MyError {}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_lines("input.txt")?;

    let mut adjacencies = HashMap::new();

    for line in lines {
        let line = line?;

        let mut devices = line
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect::<VecDeque<String>>();

        let start = devices
            .pop_front()
            .ok_or(MyError::InvalidFileFormat)?
            .replace(":", "");

        adjacencies.entry(start).or_insert(vec![]).extend(devices);
    }

    let mut frontier = VecDeque::new();

    frontier.push_back("you");

    let mut answer = 0;

    while let Some(cur) = frontier.pop_front() {
        if cur == "out" {
            answer += 1;
            continue;
        }

        for next in adjacencies.get(cur).ok_or(MyError::InvalidFileFormat)? {
            frontier.push_back(next);
        }
    }

    println!("The answer is {answer}");

    Ok(())
}

fn read_lines<T>(filename: T) -> io::Result<Lines<BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
