use std::{error::Error, fs::read_to_string, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    for (lower, upper) in get_ranges("input.txt")? {
        println!("Lower={lower} Upper={upper}");
    }

    Ok(())
}

fn get_ranges<P>(filename: P) -> Result<Vec<(u64, u64)>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let input = read_to_string(filename)?;

    input
        .split(',')
        .map(|range| {
            let (lower, upper) = range
                .split_once('-')
                .ok_or("Could not split range on '-'")?;

            Ok((lower.trim().parse::<u64>()?, upper.trim().parse::<u64>()?))
        })
        .collect()
}
