use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let str_digit = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut sum_of_cal = 0;

    for line in reader.lines() {
        let line = &line?;
        if line.is_empty() {
            continue;
        }

        let mut replaced_line = String::from(line);

        let mut matches: BTreeMap<usize, usize> = Default::default();

        for (i, &digit_str) in str_digit.iter().enumerate().skip(1) {
            let digit_str_matches: Vec<_> = line.match_indices(digit_str).collect();

            for (index, _m) in digit_str_matches {
                matches.insert(index, i);
            }
        }

        if matches.len() > 0 {
            let matches_vec: Vec<_> = matches.iter().collect();

            let first_digit = matches_vec[0].1;
            replaced_line = replaced_line.replace(str_digit[*first_digit], &first_digit.to_string());

            let last_digit = matches_vec.last().unwrap().1;
            replaced_line = replaced_line.replace(str_digit[*last_digit], &last_digit.to_string());
        }

        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;

        let chars: Vec<char> = replaced_line.chars().collect();

        for c in chars.iter() {
            if c.is_digit(10) {
                first_digit = c.to_digit(10);
                break;
            }
        }

        for c in chars.iter().rev() {
            if c.is_digit(10) {
                last_digit = c.to_digit(10);
                break;
            }
        }

        if let(Some(first), Some(last)) = (first_digit, last_digit) {
            let val = (first * 10) + last;
            println!("Digits: {:?} & {:?} (= {val}) for line '{replaced_line}' (was '{line}')", first_digit, last_digit);
            sum_of_cal += val;
        }
    }
    println!("Calibration value: {sum_of_cal}");
    Ok(())
}
