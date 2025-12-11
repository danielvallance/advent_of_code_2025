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
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn new(x: u64, y: u64, z: u64) -> Point {
        Point { x, y, z }
    }

    fn distance_squared(&self, other: &Point) -> u64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        dx * dx + dy * dy + dz * dz
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_lines("input.txt")?;

    let points = lines
        .into_iter()
        .map(|line| {
            let mut coords = line?
                .split(',')
                .map(|coord_str| Ok(coord_str.parse::<u64>()?))
                .collect::<Result<Vec<u64>, Box<dyn Error>>>()?
                .into_iter();
            Ok(Point::new(
                coords.next().ok_or(MyError::InvalidFileFormat)?,
                coords.next().ok_or(MyError::InvalidFileFormat)?,
                coords.next().ok_or(MyError::InvalidFileFormat)?,
            ))
        })
        .collect::<Result<Vec<Point>, Box<dyn Error>>>()?;

    let mut distance_matrix = vec![vec![0; points.len()]; points.len()];

    for i in 0..points.len() {
        for j in 0..points.len() {
            distance_matrix[i][j] = if i < j {
                points[i].distance_squared(&points[j])
            } else if j < i {
                distance_matrix[j][i]
            } else {
                0
            }
        }
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
