use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    path::Path,
};

fn ranges_overlap(a: (i64, i64), b: (i64, i64)) -> bool {
    if a.0 > b.0 {
        return ranges_overlap(b, a);
    }

    b.0 - 1 <= a.1
}

fn add_range(ranges: &mut Vec<(i64, i64)>, start: i64, end: i64) {
    let new_range = (start, end);
    let mut left = 0;
    let mut right = ranges.len();
    let mut idx = left + (right - left) / 2;

    while left < right {
        if let Some(range) = ranges.get(idx) {
            if new_range.0 == range.0 {
                break;
            } else if new_range.0 > range.0 {
                left = idx + 1;
            } else {
                right = idx;
            }
        } else {
            break;
        }
        idx = left + (right - left) / 2;
    }

    ranges.insert(idx, new_range);

    while let Some(&other) = ranges.get(idx + 1)
        && let Some(new_range) = ranges.get_mut(idx)
        && ranges_overlap(*new_range, other)
    {
        new_range.1 = new_range.1.max(other.1);
        ranges.remove(idx + 1);
    }

    while let Some(&other) = ranges.get(idx.wrapping_sub(1))
        && let Some(new_range) = ranges.get_mut(idx)
        && ranges_overlap(*new_range, other)
    {
        new_range.0 = other.0;
        new_range.1 = new_range.1.max(other.1);
        ranges.remove(idx.wrapping_sub(1));
        idx -= 1;
    }
}

fn _is_in_ranges(ranges: &[(i64, i64)], num: i64) -> bool {
    let mut left = 0;
    let mut right = ranges.len() as i64 - 1;

    while left <= right {
        let idx = left + (right - left) / 2;

        if let Some(range) = ranges.get(idx as usize) {
            if num > range.1 {
                left = idx + 1;
            } else if num < range.0 {
                right = idx.saturating_sub(1);
            } else {
                return true;
            }
        } else {
            break;
        }
    }

    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = read_lines("input.txt")?;

    let ranges: Vec<(i64, i64)> = lines
        .by_ref()
        .take_while(|line| match line {
            Ok(line) => !line.is_empty(),
            Err(_) => true,
        })
        .map(|line| {
            let line = line?;
            let mut limits = line.split('-');

            let start = limits.next().ok_or("Invalid format")?.parse::<i64>()?;
            let end = limits.next().ok_or("Invalid format")?.parse::<i64>()?;

            if limits.next().is_some() {
                return Err("Invalid format".into());
            }

            Ok((start, end))
        })
        .collect::<Result<_, Box<dyn Error>>>()?;

    let mut fresh_ranges = vec![];

    for (start, end) in ranges {
        add_range(&mut fresh_ranges, start, end);
    }

    let answer = fresh_ranges
        .into_iter()
        .fold(0, |acc, (start, end)| acc + end - start + 1);

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
    fn range_overlap_test() {
        let test_input = [
            ((0, 2), (4, 5), false),
            ((-10, 2), (4, 400), false),
            ((0, 2), (2, 5), true),
            ((4, 16), (4, 16), true),
            ((5, 20), (19, 20), true),
            ((0, 2), (1, 3), true),
            ((1, 1), (2, 2), true),
            ((1, 6), (7, 9), true),
        ];

        for (a, b, expected) in test_input {
            assert_eq!(ranges_overlap(a, b), expected);
            assert_eq!(ranges_overlap(b, a), expected);
        }
    }

    #[test]
    fn add_empty_range_test() {
        let mut ranges = vec![];
        add_range(&mut ranges, 0, 1);
        assert_eq!(ranges, vec![(0, 1)]);
    }

    #[test]
    fn add_range_test() {
        let mut ranges = vec![];

        let ranges_to_add = [
            (0, 4),
            (9, 13),
            (25, 32),
            (5, 5),
            (11, 14),
            (52, 57),
            (49, 51),
            (26, 27),
        ];

        for (start, end) in ranges_to_add {
            add_range(&mut ranges, start, end);
        }

        assert_eq!(ranges, vec![(0, 5), (9, 14), (25, 32), (49, 57)])
    }

    #[test]
    fn is_in_empty_range_test() {
        assert!(!is_in_ranges(&[], 4));
    }

    #[test]
    fn is_in_range_test() {
        let ranges = vec![(0, 5), (9, 14), (25, 32), (49, 57)];

        let in_range_test_data = [0, 3, 5, 9, 11, 14, 25, 28, 32, 49, 53, 57];
        let not_in_range_test_data = [-1, 6, 8, 15, 20, 33, 40, 58, 100];

        for num in in_range_test_data {
            assert!(is_in_ranges(&ranges, num));
        }

        for num in not_in_range_test_data {
            assert!(!is_in_ranges(&ranges, num));
        }
    }
}
