use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
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

#[derive(PartialEq, Debug)]
enum OP {
    Add,
    Mul,
}

fn str_to_op(s: &str) -> Result<OP, MyError> {
    match s {
        "+" => Ok(OP::Add),
        "*" => Ok(OP::Mul),
        _ => Err(MyError::InvalidFileFormat),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines: Vec<String> = read_lines("input.txt")?.collect::<Result<Vec<String>, _>>()?;

    let operators: Vec<String> = lines
        .pop()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .ok_or(MyError::InvalidFileFormat)?;

    let mut operand_slices: Vec<Vec<&[char]>> = vec![];

    let operand_chars = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start = 0;
    for i in 0..operand_chars[0].len() {
        if operand_chars.iter().all(|v| v[i].is_whitespace()) {
            operand_slices.push(operand_chars.iter().map(|v| &v[start..i]).collect());
            start = i + 1;
        }
    }

    operand_slices.push(operand_chars.iter().map(|v| &v[start..]).collect());

    let mut answer = 0;

    for (idx, operands) in operand_slices.iter().enumerate() {
        let op_str = operators.get(idx).ok_or(MyError::InvalidFileFormat)?;
        let op = str_to_op(op_str)?;

        let mut current_operands: Vec<i64> = vec![];

        for i in (0..operands[0].len()).rev() {
            let current_operand = operands
                .iter()
                .map(|v| v[i])
                .filter(|c| !c.is_whitespace())
                .fold(0, |acc, x| (acc * 10) + x.to_digit(10).unwrap() as i64);
            current_operands.push(current_operand);
        }

        answer += match op {
            OP::Add => current_operands.iter().sum::<i64>(),
            OP::Mul => current_operands.into_iter().product::<i64>(),
        }
    }

    println!("The answer is {answer}");

    Ok(())
}

fn read_lines<T>(filename: T) -> Result<Lines<BufReader<File>>, Box<dyn Error>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn str_to_op_test() {
        assert_eq!(str_to_op("+").unwrap(), OP::Add);
        assert_eq!(str_to_op("*").unwrap(), OP::Mul);

        let invalid_test_data = ["", "++", "**", "*+", "+*", "INVALID"];

        for invalid_op_str in invalid_test_data {
            assert_eq!(
                str_to_op(invalid_op_str).unwrap_err(),
                MyError::InvalidFileFormat
            )
        }
    }
}
