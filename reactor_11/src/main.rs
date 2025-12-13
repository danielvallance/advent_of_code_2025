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

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct PathTrace<'a> {
    name: &'a str,
    dac: bool,
    fft: bool,
}

impl<'a> PathTrace<'a> {
    fn new(name: &'a str, dac: bool, fft: bool) -> Self {
        Self { name, dac, fft }
    }
}

fn dfs<'a>(
    adjacencies: &'a HashMap<String, Vec<String>>,
    start: &PathTrace<'a>,
    visited: &mut HashMap<PathTrace<'a>, u64>,
) -> Result<u64, MyError> {
    if let Some(&ret) = visited.get(start) {
        return Ok(ret);
    }

    let PathTrace {
        name,
        mut dac,
        mut fft,
    } = *start;

    if name == "out" {
        if dac && fft {
            return Ok(1);
        } else {
            return Ok(0);
        }
    } else if name == "dac" {
        dac = true;
    } else if name == "fft" {
        fft = true;
    }

    let mut answer = 0;

    for next in adjacencies
        .get(start.name)
        .ok_or(MyError::InvalidFileFormat)?
    {
        let next_path_trace = PathTrace::new(next, dac, fft);
        let next_ret = dfs(adjacencies, &next_path_trace, visited)?;
        visited.insert(next_path_trace, next_ret);
        answer += next_ret;
    }

    Ok(answer)
}

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

    let mut visited = HashMap::new();

    let answer = dfs(
        &adjacencies,
        &PathTrace::new("svr", false, false),
        &mut visited,
    )?;

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
