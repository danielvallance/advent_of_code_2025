use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

#[derive(Debug)]
enum MyError {
    InvalidFileFormat,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid file format")
    }
}

impl std::error::Error for MyError {}

fn min_presses(target: u32, buttons: &[u32], lights: usize) -> Option<u32> {
    let mut core_nos = vec![0u32; lights];
    let mut core_nos_masks = vec![0u32; lights];

    let mut zero_sets = vec![];
    for (idx, &button_set_val) in buttons.iter().enumerate() {
        let mut button_set_val = button_set_val;
        let mut buttons_mask = 1 << idx;
        let mut used = false;
        for light in (0..lights).rev() {
            if button_set_val & (1 << light) == 0 {
                continue;
            }

            if core_nos[light] == 0 {
                core_nos[light] = button_set_val;
                core_nos_masks[light] = buttons_mask;
                button_set_val = 0;
                used = true;
                break;
            } else {
                button_set_val ^= core_nos[light];
                buttons_mask ^= core_nos_masks[light];
            }
        }

        if !used && button_set_val == 0 {
            zero_sets.push(buttons_mask)
        }
    }

    let mut current_target = target;
    let mut solution_mask = 0u32;

    for light in (0..lights).rev() {
        if current_target & (1 << light) != 0 {
            if core_nos[light] == 0 {
                return None;
            }
            current_target ^= core_nos[light];
            solution_mask ^= core_nos_masks[light];
        }
    }

    let mut min_buttons = solution_mask.count_ones();

    for i in 1..1 << zero_sets.len() {
        let mut candidate_solution_mask = solution_mask;

        for (j, &zero_set) in zero_sets.iter().enumerate() {
            if i & (1 << j) != 0 {
                candidate_solution_mask ^= zero_set;
            }
        }

        min_buttons = min_buttons.min(candidate_solution_mask.count_ones());
    }

    Some(min_buttons)
}

fn parse_target(s: &str) -> (u32, usize) {
    s.chars()
        .skip(1)
        .take_while(|&c| c == '#' || c == '.')
        .enumerate()
        .fold((0, 0), |(acc, _), (idx, c)| {
            ((acc << 1) + if c == '#' { 1 } else { 0 }, idx + 1)
        })
}

fn parse_buttons(s: &[&str], lights: usize) -> Vec<u32> {
    s.iter()
        .map(|button| {
            button
                .chars()
                .skip(1)
                .take_while(|&c| c == ',' || c.is_ascii_digit())
                .fold(0u32, |acc, c| {
                    if let Some(digit) = c.to_digit(10) {
                        acc | (1 << (lights as u32 - digit - 1))
                    } else {
                        acc
                    }
                })
        })
        .collect::<Vec<u32>>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_lines("input.txt")?;

    let machine_defs = lines
        .into_iter()
        .map(|line| {
            let line = line?;
            let mut portions = line.split_whitespace().collect::<VecDeque<&str>>();
            let (target, lights) =
                parse_target(portions.pop_front().ok_or(MyError::InvalidFileFormat)?);
            let joltage_str = String::from(portions.pop_back().ok_or(MyError::InvalidFileFormat)?);
            let buttons = parse_buttons(&portions.into_iter().collect::<Vec<&str>>(), lights);
            Ok((target, buttons, joltage_str, lights))
        })
        .collect::<Result<Vec<(u32, Vec<u32>, String, usize)>, Box<dyn Error>>>()?;

    let mut answer = 0;
    for (target, buttons, _, lights) in machine_defs {
        if let Some(min_buttons) = min_presses(target, &buttons, lights) {
            answer += min_buttons;
        } else {
            return Err(MyError::InvalidFileFormat.into());
        }
    }

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
