use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

#[derive(Debug, PartialEq)]
enum MyError {
    InvalidFileFormat,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid file format")
    }
}

impl Error for MyError {}

#[derive(Debug, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn new(x: u64, y: u64) -> Self {
        Point { x, y }
    }

    fn area(&self, other: &Point) -> u64 {
        let x_len = self.x.abs_diff(other.x) + 1;
        let y_len = self.y.abs_diff(other.y) + 1;
        x_len * y_len
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_lines("input.txt")?;

    let points = lines
        .map(|line| {
            let mut coords = line?
                .split(',')
                .map(|coord| Ok(coord.parse::<u64>()?))
                .collect::<Result<Vec<u64>, Box<dyn Error>>>()?
                .into_iter();

            Ok(Point::new(
                coords.next().ok_or(MyError::InvalidFileFormat)?,
                coords.next().ok_or(MyError::InvalidFileFormat)?,
            ))
        })
        .collect::<Result<Vec<Point>, Box<dyn Error>>>()?;

    let answer = points
        .iter()
        .flat_map(|point_a| points.iter().map(|point_b| point_a.area(point_b)))
        .max()
        .unwrap_or(0);

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
