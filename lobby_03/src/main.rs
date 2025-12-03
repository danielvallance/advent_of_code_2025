use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = read_lines("input.txt")?;

    let mut sum = 0;
    while let Some(Ok(line)) = lines.next() {
        sum += largest_joltage(&line)?;
    }

    println!("The answer is {sum}");

    Ok(())
}

fn largest_joltage(bank: &str) -> Result<u32, Box<dyn Error>> {
    let mut largest = None;
    let mut second_largest = None;

    for num in bank
        .chars()
        .map(|c| c.to_digit(10).ok_or("Non-digit character encountered"))
        .rev()
    {
        let num = num?;
        match largest {
            None => {
                largest = Some(num);
            }
            Some(largest_num) => match second_largest {
                None => {
                    second_largest = largest;
                    largest = Some(num);
                }
                Some(second_largest_num) => {
                    if num >= largest_num {
                        if largest_num >= second_largest_num {
                            second_largest = Some(largest_num);
                        }
                        largest = Some(num)
                    }
                }
            },
        }
    }

    match (largest, second_largest) {
        (Some(largest), Some(second_largest)) => Ok(largest * 10 + second_largest),
        _ => Err("Invalid input".into()),
    }
}

fn read_lines<T>(filepath: T) -> Result<io::Lines<io::BufReader<File>>, Box<dyn Error>>
where
    T: AsRef<Path>,
{
    let file = File::open(filepath)?;

    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_joltage() {
        let data = [
            ("182837571142", 88),
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
        ];

        for (str, joltage) in data {
            assert_eq!(largest_joltage(str).unwrap(), joltage);
        }
    }
}
