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

#[derive(Debug, PartialEq, Copy, Clone)]
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

fn is_valid_rectangle(point_a: &Point, point_b: &Point, edges: &[(Point, Point)]) -> bool {
    let (low_x_rect, high_x_rect) = if point_a.x < point_b.x {
        (point_a.x, point_b.x)
    } else {
        (point_b.x, point_a.x)
    };
    let (low_y_rect, high_y_rect) = if point_a.y < point_b.y {
        (point_a.y, point_b.y)
    } else {
        (point_b.y, point_a.y)
    };

    for (e1, e2) in edges {
        let x1_edge = e1.x;
        let x2_edge = e2.x;
        let y1_edge = e1.y;
        let y2_edge = e2.y;

        if x1_edge == x2_edge {
            let (low_y_edge, high_y_edge) = if y1_edge < y2_edge {
                (y1_edge, y2_edge)
            } else {
                (y2_edge, y1_edge)
            };

            if low_x_rect < x1_edge
                && x1_edge < high_x_rect
                && low_y_edge.max(low_y_rect) < high_y_edge.min(high_y_rect)
            {
                return false;
            }
        } else {
            let (low_x_edge, high_x_edge) = if x1_edge < x2_edge {
                (x1_edge, x2_edge)
            } else {
                (x2_edge, x1_edge)
            };

            if low_y_rect < y1_edge
                && y1_edge < high_y_rect
                && low_x_edge.max(low_x_rect) < high_x_edge.min(high_x_rect)
            {
                return false;
            }
        }
    }

    let mid_x_rect = low_x_rect + (high_x_rect - low_x_rect) / 2;
    let mid_y_rect = low_y_rect + (high_y_rect - low_y_rect) / 2;

    let mut crossings = 0;
    for (e1, e2) in edges {
        if e1.x == e2.x
            && e1.y.min(e2.y) < mid_y_rect
            && mid_y_rect < e1.y.max(e2.y)
            && e1.x > mid_x_rect
        {
            crossings += 1;
        }
    }

    crossings % 2 == 1
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

    let edges = points
        .iter()
        .copied()
        .zip(points.iter().copied().cycle().skip(1))
        .collect::<Vec<(Point, Point)>>();

    let answer = points
        .iter()
        .flat_map(|point_a| {
            points
                .iter()
                .filter(|&point_b| is_valid_rectangle(point_a, point_b, &edges))
                .map(|point_b| point_a.area(point_b))
        })
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
