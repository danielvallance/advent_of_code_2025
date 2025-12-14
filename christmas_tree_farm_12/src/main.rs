use std::{
    collections::VecDeque,
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

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    quantities: [usize; 6],
}

fn parse_present(lines: &mut Vec<String>) -> usize {
    lines
        .drain(0..5)
        .skip(1)
        .take(4)
        .filter(|s| !s.is_empty())
        .map(|row| row.chars().filter(|&c| c == '#').count())
        .sum::<usize>()
}

fn parse_grid(s: &str) -> Result<Grid, Box<dyn Error>> {
    let mut portions = s.split(':').collect::<VecDeque<&str>>();

    let dims = portions
        .pop_front()
        .ok_or(MyError::InvalidFileFormat)?
        .split('x')
        .filter(|dim| !dim.is_empty())
        .map(|num| Ok(num.parse::<usize>()?))
        .collect::<Result<Vec<usize>, Box<dyn Error>>>()?;

    let quantities = portions
        .pop_front()
        .ok_or(MyError::InvalidFileFormat)?
        .split_whitespace()
        .map(|num| Ok(num.parse::<usize>()?))
        .collect::<Result<Vec<usize>, Box<dyn Error>>>()?;

    Ok(Grid {
        rows: dims.first().copied().ok_or(MyError::InvalidFileFormat)?,
        cols: dims.get(1).copied().ok_or(MyError::InvalidFileFormat)?,
        quantities: quantities
            .try_into()
            .map_err(|_| MyError::InvalidFileFormat)?,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = read_lines("input.txt")?
        .map(|line| Ok(line?))
        .collect::<Result<Vec<String>, Box<dyn Error>>>()?;

    let present_sizes = (0..=5)
        .map(|_| parse_present(&mut lines))
        .collect::<Vec<usize>>();

    let grids = lines
        .iter()
        .map(|line| parse_grid(line))
        .collect::<Result<Vec<Grid>, Box<dyn Error>>>()?;

    let mut answer = 0;

    for grid in grids {
        let max_units = grid.rows * grid.cols;

        let required_units = grid
            .quantities
            .iter()
            .zip(present_sizes.iter())
            .map(|(qty, size)| qty * size)
            .sum::<usize>();

        if max_units < required_units {
            continue;
        }

        let lower_bound = (grid.rows / 3) * (grid.cols / 3);

        if lower_bound >= grid.quantities.iter().sum::<usize>() {
            answer += 1;
            continue;
        }

        panic!("Cannot determine answer, quitting.");
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
