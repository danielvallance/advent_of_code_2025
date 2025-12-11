use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

#[derive(Debug, PartialEq)]
enum MyError {
    InvalidFileFormat,
    ElementNotFound,
    IncompleteGrid,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MyError::ElementNotFound => {
                write!(f, "Element not found")
            }
            MyError::InvalidFileFormat => {
                write!(f, "Invalid file format")
            }
            MyError::IncompleteGrid => {
                write!(f, "Grid not complete")
            }
        }
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
    fn new(x: u64, y: u64, z: u64) -> Self {
        Point { x, y, z }
    }

    fn distance_squared(&self, other: &Point) -> u64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        dx * dx + dy * dy + dz * dz
    }
}

struct DisjointSet {
    parent: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        DisjointSet {
            parent: (0..size).collect(),
        }
    }

    fn find(&mut self, element: usize) -> Option<usize> {
        let cur = *self.parent.get(element)?;

        if *self.parent.get(cur)? != cur {
            let ans = self.find(cur)?;
            *self.parent.get_mut(element)? = ans;
            return Some(ans);
        }

        Some(cur)
    }

    fn union(&mut self, a: usize, b: usize) -> Option<usize> {
        let a_rep = self.find(a)?;
        let b_rep = self.find(b)?;
        *self.parent.get_mut(b_rep)? = a_rep;
        Some(a_rep)
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

    let mut pq = BinaryHeap::new();

    for i in 0..points.len() {
        for j in 0..points.len() {
            distance_matrix[i][j] = if i < j {
                let distance = points[i].distance_squared(&points[j]);
                pq.push(Reverse((distance, i, j)));
                distance
            } else if j < i {
                distance_matrix[j][i]
            } else {
                0
            }
        }
    }

    let mut disjoint_set = DisjointSet::new(points.len());

    while let Some(Reverse((_, i, j))) = pq.pop() {
        if disjoint_set.find(i).ok_or(MyError::ElementNotFound)?
            != disjoint_set.find(j).ok_or(MyError::ElementNotFound)?
        {
            disjoint_set.union(i, j).ok_or(MyError::ElementNotFound)?;

            let rep = disjoint_set.find(0).ok_or(MyError::ElementNotFound)?;

            if (1..points.len()).all(|idx| disjoint_set.find(idx) == Some(rep)) {
                println!(
                    "The answer is {}",
                    points.get(i).ok_or(MyError::ElementNotFound)?.x
                        * points.get(j).ok_or(MyError::ElementNotFound)?.x
                );
                return Ok(());
            }
        }
    }

    Err(MyError::IncompleteGrid.into())
}

fn read_lines<T>(filename: T) -> io::Result<Lines<BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
