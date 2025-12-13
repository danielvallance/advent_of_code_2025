use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    num::ParseIntError,
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

fn _min_presses(target: u32, buttons: &[u32], lights: usize) -> Option<u32> {
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

fn parse_buttons(s: &[&str]) -> Result<Vec<Vec<i64>>, ParseIntError> {
    s.iter()
        .map(|button| {
            button
                .split(&[',', '(', ')'])
                .filter(|&c| !c.is_empty())
                .map(|button_val| button_val.parse::<i64>())
                .collect::<Result<Vec<i64>, ParseIntError>>()
        })
        .collect::<Result<Vec<Vec<i64>>, ParseIntError>>()
}

fn parse_joltage(s: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    s.split(&[',', '{', '}'])
        .filter(|&c| !c.is_empty())
        .map(|s| Ok(s.parse::<i64>()?))
        .collect()
}

fn min_joltage_presses(buttons: &[Vec<i64>], joltages: &[i64]) -> Option<u64> {
    let mut binary_map: HashMap<u64, Vec<(u64, Vec<i64>)>> = HashMap::new();

    for i in 0..(1 << buttons.len()) {
        let mut current_joltage_vals = vec![0i64; joltages.len()];

        let mut no_pressed = 0;

        for (idx, joltages_affected) in buttons.iter().enumerate() {
            if (i >> idx) & 1 == 1 {
                no_pressed += 1;
                for &joltage_affected in joltages_affected {
                    current_joltage_vals[joltage_affected as usize] += 1;
                }
            }
        }

        let binary_joltage = current_joltage_vals
            .iter()
            .map(|x| (x % 2).unsigned_abs())
            .fold(0, |acc, b| (acc << 1) + b);

        binary_map
            .entry(binary_joltage)
            .or_default()
            .push((no_pressed, current_joltage_vals));
    }

    struct Solver<'a> {
        binary_map: &'a HashMap<u64, Vec<(u64, Vec<i64>)>>,
        dp: HashMap<Vec<i64>, Option<u64>>,
    }

    impl<'a> Solver<'a> {
        fn solve(&mut self, targets: Vec<i64>) -> Option<u64> {
            if targets.iter().all(|&x| x == 0) {
                return Some(0);
            }

            if let Some(&res) = self.dp.get(&targets) {
                return res;
            }

            let current_binary_joltage = targets
                .iter()
                .map(|x| (x % 2).unsigned_abs())
                .fold(0, |acc, b| (acc << 1) + b);

            let candidates = match self.binary_map.get(&current_binary_joltage) {
                Some(c) => c,
                None => {
                    self.dp.insert(targets, None);
                    return None;
                }
            };

            let mut min_pressed = None;

            for (cost, joltage_diffs) in candidates {
                let mut next_targets = vec![];
                let mut solvable = true;

                for (target, joltage_diff) in targets.iter().zip(joltage_diffs) {
                    let remaining = target - joltage_diff;
                    if remaining < 0 {
                        solvable = false;
                        break;
                    }
                    next_targets.push(remaining / 2);
                }

                if solvable {
                    if let Some(sub_result) = self.solve(next_targets) {
                        let total = cost + 2 * sub_result;
                        min_pressed = Some(min_pressed.map_or(total, |m: u64| m.min(total)));
                    }
                }
            }

            self.dp.insert(targets, min_pressed);
            min_pressed
        }
    }

    let mut solver = Solver {
        binary_map: &binary_map,
        dp: HashMap::new(),
    };

    solver.solve(joltages.to_vec())
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
            let joltage_str =
                parse_joltage(portions.pop_back().ok_or(MyError::InvalidFileFormat)?)?;
            let buttons = parse_buttons(&portions.into_iter().collect::<Vec<&str>>())?;
            Ok((target, buttons, joltage_str, lights))
        })
        .collect::<Result<Vec<(u32, Vec<Vec<i64>>, Vec<i64>, usize)>, Box<dyn Error>>>()?;

    let mut answer = 0;
    for (_, buttons, joltages, _) in machine_defs {
        if let Some(min_buttons) = min_joltage_presses(&buttons, &joltages) {
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
