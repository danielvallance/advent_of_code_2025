use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut grid: Vec<Vec<i32>> = read_lines("input.txt")?
        .map(|line| {
            let line = line?;
            Ok(line.chars().map(|c| if c == '@' { 1 } else { 0 }).collect())
        })
        .collect::<Result<_, Box<dyn Error>>>()?;

    let prefix_lists: Vec<Vec<i32>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .scan(0, |state, &cell| {
                    *state += cell;
                    Some(*state)
                })
                .collect()
        })
        .collect();

    let mut answer = 0;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != 1 {
                continue;
            }
            let mut surrounding_rolls = -1;

            surrounding_rolls += (row.saturating_sub(1)..=(row + 1).min(grid.len() - 1))
                .map(|idx| {
                    let mut value = prefix_lists[idx][(col + 1).min(grid[0].len() - 1)];
                    if col > 1 {
                        value -= prefix_lists[idx][col - 2];
                    }
                    value
                })
                .sum::<i32>();

            if surrounding_rolls < 4 {
                answer += 1;
                grid[row][col] = 0;
                for r in row.saturating_sub(1)..=(row + 1).min(grid.len() - 1) {
                    for c in col.saturating_sub(1)..=(col + 1).min(grid[0].len() - 1) {
                        q.push_back((r, c));
                    }
                }
            }
        }
    }

    while let Some((row, col)) = q.pop_front() {
        if grid[row][col] != 1 {
            continue;
        }

        let mut surrounding_rolls = -1;
        for r in row.saturating_sub(1)..=(row + 1).min(grid.len() - 1) {
            for c in col.saturating_sub(1)..=(col + 1).min(grid[0].len() - 1) {
                if grid[r][c] == 1 {
                    surrounding_rolls += 1;
                }
            }
        }

        if surrounding_rolls < 4 {
            answer += 1;
            grid[row][col] = 0;
            for r in row.saturating_sub(1)..=(row + 1).min(grid.len() - 1) {
                for c in col.saturating_sub(1)..=(col + 1).min(grid[0].len() - 1) {
                    q.push_back((r, c));
                }
            }
        }
    }

    println!("The answer is {answer}");
    Ok(())
}

fn read_lines<T>(filepath: T) -> Result<io::Lines<BufReader<File>>, Box<dyn Error>>
where
    T: AsRef<Path>,
{
    let file = File::open(filepath)?;

    Ok(io::BufReader::new(file).lines())
}
