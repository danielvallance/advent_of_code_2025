use std::{
    collections::VecDeque,
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

fn largest_joltage(bank: &str) -> Result<u64, Box<dyn Error>> {
    let size = 12;

    let mut batteries: VecDeque<u64> = VecDeque::new();

    for num in bank
        .chars()
        .map(|c| c.to_digit(10).ok_or("Non-digit character encountered"))
        .rev()
    {
        let num = num? as u64;
        if batteries.len() < size {
            batteries.push_front(num);
        } else if num >= batteries[0] {
            let mut temp = VecDeque::new();
            temp.push_back(num);

            let mut prev = num;

            while let Some(battery) = batteries.pop_front() {
                if prev >= battery {
                    temp.push_back(battery);
                    prev = battery;
                } else {
                    batteries.push_front(battery);
                    break;
                }
            }

            temp.pop_back();

            while let Some(battery) = temp.pop_back() {
                batteries.push_front(battery);
            }
        }
    }

    let mut sum = 0;

    for battery in batteries {
        sum *= 10;
        sum += battery;
    }

    Ok(sum)
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
            ("182837571142", 182837571142),
            ("987654321111111", 987654321111),
            ("811111111111119", 811111111119),
            ("234234234234278", 434234234278),
            ("818181911112111", 888911112111),
        ];

        for (str, joltage) in data {
            assert_eq!(largest_joltage(str).unwrap(), joltage);
        }
    }
}
