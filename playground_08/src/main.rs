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
impl std::error::Error for MyError {}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl Point {
    fn new(x: u32, y: u32, z: u32) -> Point {
        Point { x, y, z }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_lines("input.txt")?;

    let points = lines
        .into_iter()
        .map(|line| {
            let mut coords = line?
                .split(',')
                .map(|coord_str| Ok(coord_str.parse::<u32>()?))
                .collect::<Result<Vec<u32>, Box<dyn Error>>>()?
                .into_iter();
            Ok(Point::new(
                coords.next().ok_or(MyError::InvalidFileFormat)?,
                coords.next().ok_or(MyError::InvalidFileFormat)?,
                coords.next().ok_or(MyError::InvalidFileFormat)?,
            ))
        })
        .collect::<Result<Vec<Point>, Box<dyn Error>>>()?;

    println!("{points:?}");

    Ok(())
}

fn read_lines<T>(filename: T) -> io::Result<Lines<BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
