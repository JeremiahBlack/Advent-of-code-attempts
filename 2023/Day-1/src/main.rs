use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No argument provided");
        return;
    }

    let filename = &args[1];

    println!("Reading filename {}", filename);

    let file_contents = fs::read_to_string(filename)
        .expect("Invalid file given.");

    let mut sum: u32 = 0;
    for l in file_contents.lines() {
        sum += get_line_value(l);
    }

    println!("Final result: {}", sum);
}

fn get_line_value(line: &str) -> u32 {
    let mut first: char = 'a';
    let mut second: char = 'a';

    for c in line.chars() {
        if c.is_numeric() {
            second = c;
            if first == 'a' {
                first = c;
            }
        }
    }

    let result = format!("{}{}", first, second).parse::<u32>();

    return match result {
        Ok(good_val) => good_val,
        Err(_e) => 0
    }
}

#[cfg(test)]
mod test {
    use crate::get_line_value;

    #[test]
    fn finds_the_same_digit_if_only_one_exists() {
        assert_eq!(get_line_value("abc2def"), 22);
    }

    #[test]
    fn finds_different_digits_if_more_than_one_exists() {
        assert_eq!(get_line_value("abc2def3ghi"), 23);
    }

    #[test]
    fn returns_zero_if_no_digits_were_found() {
        assert_eq!(get_line_value("abcdef"), 0);
    }

    #[test]
    fn only_finds_the_first_and_last_digit_if_more_than_2_exist() {
        assert_eq!(get_line_value("abc2def3ghi4"), 24);
    }
}