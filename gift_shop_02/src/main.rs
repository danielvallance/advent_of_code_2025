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

            for repeating_len in 1..digits {
                if digits % repeating_len != 0 {
                    continue;
                }

                let base = 10u64.pow(repeating_len);
                let mantissa = candidate % base;
                let mut dividend = candidate;

                while dividend > 0 && dividend % base == mantissa {
                    dividend /= base;
                }

                if dividend == 0 {
                    sum += candidate;
                    break;
                }
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
