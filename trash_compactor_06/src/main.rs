use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

const INVALID_FILE_FORMAT: &str = "Invalid file format";

enum OP {
    ADD,
    MUL,
}

fn str_to_op(s: &str) -> Result<OP, Box<dyn Error>> {
    match s {
        "+" => Ok(OP::ADD),
        "*" => Ok(OP::MUL),
        _ => Err(INVALID_FILE_FORMAT.into()),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut elements = read_lines("input.txt")?
        .map(|line| {
            let line = line?;
            Ok(line.split_whitespace().map(|s| s.to_owned()).collect())
        })
        .collect::<Result<Vec<Vec<String>>, Box<dyn Error>>>()?;

    let operators = elements.pop().ok_or(INVALID_FILE_FORMAT)?;

    let operands = elements;

    let mut answer = 0;

    for idx in 0..operands[0].len() {
        let op_str = operators.get(idx).ok_or(INVALID_FILE_FORMAT)?;
        let op = str_to_op(op_str)?;

        let current_operands = operands
            .iter()
            .map(|operands| Ok(operands[idx].parse::<i64>()?))
            .collect::<Result<Vec<i64>, Box<dyn Error>>>()?;

        answer += match op {
            OP::ADD => current_operands.iter().sum::<i64>(),
            OP::MUL => current_operands.into_iter().product::<i64>(),
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
