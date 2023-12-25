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
    // Using 'a' as an invalid value placeholder.
    let mut first: char = 'a';
    let mut second: char = 'a';

    for (index, possible_digit) in line.chars().enumerate() {
        if possible_digit.is_digit(10) {
            second = possible_digit;
            if first == 'a' {
                first = possible_digit;
            }
            continue;
        }
        let str_as_digit = match &line[index..line.len()] {
            l if l.starts_with("one") => '1',
            l if l.starts_with("two") => '2',
            l if l.starts_with("three") => '3',
            l if l.starts_with("four") => '4',
            l if l.starts_with("five") => '5',
            l if l.starts_with("six") => '6',
            l if l.starts_with("seven") => '7',
            l if l.starts_with("eight") => '8',
            l if l.starts_with("nine") => '9',
            _ => 'a'
        };

        if str_as_digit.is_digit(10) {
            second = str_as_digit;
            if first == 'a' {
                first = str_as_digit;
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

    #[test]
    fn should_use_one_written_out_as_a_digit() {
        assert_eq!(get_line_value("abconedef"), 11);
    }

    #[test]
    fn should_use_two_written_out_as_a_digit() {
        assert_eq!(get_line_value("abctwodef"), 22);
    }

    #[test]
    fn should_use_three_written_out_as_a_digit() {
        assert_eq!(get_line_value("abcthreedef"), 33);
    }

    #[test]
    fn should_use_four_written_out_as_a_digit() {
        assert_eq!(get_line_value("abcfourdef"), 44);
    }

    #[test]
    fn should_use_five_written_out_as_a_digit() {
        assert_eq!(get_line_value("abcfivedef"), 55);
    }

    #[test]
    fn should_use_six_written_out_as_a_digit() {
        assert_eq!(get_line_value("abcsixdef"), 66);
    }

    #[test]
    fn should_use_seven_written_out_as_a_digit() {
        assert_eq!(get_line_value("abcsevendef"), 77);
    }

    #[test]
    fn should_use_eight_written_out_as_a_digit() {
        assert_eq!(get_line_value("abceightdef"), 88);
    }

    #[test]
    fn should_use_nine_written_out_as_a_digit() {
        assert_eq!(get_line_value("abcninedef"), 99);
    }
}