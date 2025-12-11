use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let lines = read_lines("input.txt")?;

    let mut beam_pos = HashSet::new();

    let mut answer = 0;

    for line in lines {
        let line_chars = line?.chars().collect::<Vec<char>>();

        if beam_pos.is_empty() {
            if let Some(initial_beam_pos) = line_chars.iter().position(|&c| c == 'S') {
                beam_pos.insert(initial_beam_pos);
            }
        } else {
            let mut new_beam_pos = HashSet::new();
            for (idx, &c) in line_chars.iter().enumerate() {
                if c == '^' && beam_pos.contains(&idx) {
                    answer += 1;
                    beam_pos.remove(&idx);
                    new_beam_pos.insert(idx - 1);
                    new_beam_pos.insert(idx + 1);
                }
            }
            beam_pos.extend(new_beam_pos);
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
