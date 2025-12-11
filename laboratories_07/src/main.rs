use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_lines("input.txt")?;

    let mut beam_pos = HashMap::new();

    for line in lines {
        let line_chars = line?.chars().collect::<Vec<char>>();

        if beam_pos.is_empty() {
            if let Some(initial_beam_pos) = line_chars.iter().position(|&c| c == 'S') {
                beam_pos.insert(initial_beam_pos, 1);
            }
        } else {
            let mut new_beam_pos = HashMap::new();
            for (idx, &c) in line_chars.iter().enumerate() {
                if c == '^'
                    && let Some(quantity) = beam_pos.get(&idx).copied()
                {
                    beam_pos.remove(&idx);
                    *new_beam_pos.entry(idx - 1).or_insert(0) += quantity;
                    *new_beam_pos.entry(idx + 1).or_insert(0) += quantity;
                }
            }

            for (k, v) in new_beam_pos {
                *beam_pos.entry(k).or_insert(0) += v;
            }
        }
    }

    println!("The answer is {}", beam_pos.values().sum::<u64>());

    Ok(())
}

fn read_lines<T>(filename: T) -> io::Result<Lines<BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
