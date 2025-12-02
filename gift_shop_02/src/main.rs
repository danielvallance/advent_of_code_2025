use std::{error::Error, fs::read_to_string, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    for (lower, upper) in get_ranges("input.txt")? {
        for candidate in lower..=upper {
            let mut digits = 0;
            let mut dividend = candidate;

            while dividend > 0 {
                dividend /= 10;
                digits += 1;
            }

            if digits % 2 == 1 {
                continue;
            }

            let pivot = 10u64.pow(digits / 2);

            if candidate % pivot == candidate / pivot {
                sum += candidate;
            }
        }
    }

    println!("Answer is {sum}");

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
