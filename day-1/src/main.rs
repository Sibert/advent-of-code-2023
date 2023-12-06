use std::fs::File;
use std::io::Read;
use std::collections::BTreeMap;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut input = String::new();

    let str_digit = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    file.read_to_string(&mut input)?;

    let lines = input.split('\n');

    let mut sum_of_cal = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut replaced_line = String::from(line);

        let mut matches: BTreeMap<usize, usize> = Default::default();

        for i in 1..10 {
            //let m = line.find(str_digit[i]);
            let digit_str_matches: Vec<_> = line.match_indices(str_digit[i]).collect();

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

        let val = (first_digit.unwrap() * 10) + last_digit.unwrap();
        println!("Digits: {:?} & {:?} (= {val}) for line '{replaced_line}' (was '{line}')", first_digit, last_digit);
        sum_of_cal += val;
    }
    println!("Calibration value: {sum_of_cal}");
    Ok(())
}
