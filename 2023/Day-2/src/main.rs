use std::collections::HashMap;
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
    let mut cube_sum: u32 = 0;
    let mut max_values: HashMap<&str, u32> = HashMap::new();
    max_values.insert("red", 12);
    max_values.insert("green", 13);
    max_values.insert("blue", 14);
    for game in file_contents.lines() {
        let game_and_sets = split_game_number_and_sets(game);
        if is_game_valid(game_and_sets.1, &max_values) {
            sum += game_and_sets.0;
        }
        cube_sum += get_cube_value_for_game(game);
        
    }

    println!("Total value: {}, Total cube sum: {}", sum, cube_sum);
}

fn split_game_number_and_sets(game_info: &str) -> (u32, &str) {
    // We know that ':' acts as a separator. Title will fit the format "Game #:".
    // Each set uses ';' as a separator. There is no final ';' for the last game.
    // Each color does not have a separator, but does have whitespace between the number
    // and the color.
    // These definitely assume correct input...
    let game_separator_index = game_info.find(':').unwrap();
    let game_substring = &game_info[..game_separator_index];
    let game_space_index = game_substring.find(' ').unwrap();
    let game_number = &game_substring[(game_space_index + 1)..].parse::<u32>().unwrap();
    // Doing +2 to get rid of the empty space of the first set.
    return (*game_number, &game_info[(game_separator_index + 2)..]);
}

fn is_game_valid(game_sets: &str, max_values: &HashMap<&str, u32>) -> bool {
    let sets: Vec<&str> = game_sets.split(';').collect();
    for set_info in sets {
        if !is_set_valid(set_info.trim(), max_values) {
            return false;
        }
    }
    return true;
}

fn get_max_colors_for_set(set_info: &str) -> HashMap<&str, u32> {
    let mut counts: HashMap<&str, u32> = HashMap::new();
    let colors: Vec<&str> = set_info.split(',').collect();
    for color in colors {
        let trimmed_color  = color.trim();
        let space_index = trimmed_color.find(' ').unwrap();
        let color_name = &trimmed_color[space_index+1..];
        let max_value = counts.get(color_name);
        let color_count = (&trimmed_color[..space_index]).parse::<u32>().unwrap();

        if max_value == None {
            counts.insert(color_name, color_count);
            continue;
        }
        else if max_value.unwrap() < &color_count {
            counts.insert(color_name, color_count);
        }
    }
    return counts;
}

fn get_cube_value_for_game(game_info: &str) -> u32 {
    let mut final_color_counts: HashMap<&str, u32> = HashMap::new();
    let (_, game_color_info) = split_game_number_and_sets(game_info);
    let sets: Vec<&str> = game_color_info.split(';').collect();
    for set_info in sets {
        let current_color_count = get_max_colors_for_set(set_info.trim());
        for (color_name, color_count) in current_color_count.into_iter() {
            let current_color_max = final_color_counts.get(color_name);
            if current_color_max == None || current_color_max.unwrap() < &color_count {
                final_color_counts.insert(color_name, color_count);
            }
        }
    }

    let mut cube = 1;
    for (_, color_count) in final_color_counts.into_iter() {
        cube *= color_count;
    }

    return cube;
}

fn is_set_valid(set_info: &str, max_values: &HashMap<&str, u32>) -> bool {
    let colors: Vec<&str> = set_info.split(',').collect();
    for color in colors {
        let trimmed_color  = color.trim();
        let space_index = trimmed_color.find(' ').unwrap();
        let color_name = &trimmed_color[space_index+1..];
        let max_value = max_values.get(color_name);
        // We don't have a max, so it shouldn't exist?
        if max_value == None {
            return false;
        }

        let color_count = (&trimmed_color[..space_index]).parse::<u32>().unwrap();
        // Admission of lack of knowledge, not sure why &color_cout is required other than it makes them both &u32.
        if max_value.unwrap() < &color_count {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{split_game_number_and_sets, is_game_valid};

    #[test]
    fn should_give_game_number_with_single_digit() {
        assert_eq!(split_game_number_and_sets("Game 2: 3 Blue, 2 Red; 6 Blue, 5 Red").0, 2);
    }

    #[test]
    fn should_give_game_number_with_multiple_digit() {
        assert_eq!(split_game_number_and_sets("Game 15: 3 Blue, 2 Red; 6 Blue, 5 Red").0, 15);
    }

    #[test]
    fn should_give_game_splits_string() {
        assert_eq!(split_game_number_and_sets("Game 1: 3 Blue, 5 Red; 7 Green, 2 Yellow").1, "3 Blue, 5 Red; 7 Green, 2 Yellow");
    }

    #[test]
    fn should_determine_set_with_too_many_red_as_invalid() {
        let mut conditions: HashMap<&str, u32> = HashMap::new();
        conditions.insert("Red", 6);
        conditions.insert("Yellow", 99);
        conditions.insert("Blue", 99);
        assert_eq!(is_game_valid("3 Blue, 6 Red, 2 Yellow; 2 Blue, 7 Red, 2 Yellow", &conditions), false)
    }

    #[test]
    fn should_determine_set_under_red_max_is_valid() {
        let mut conditions: HashMap<&str, u32> = HashMap::new();
        conditions.insert("Red", 6);
        conditions.insert("Yellow", 99);
        conditions.insert("Blue", 99);
        assert_eq!(is_game_valid("3 Blue, 5 Red, 8 Yellow", &conditions), true);
    }

    #[test]
    fn should_determine_set_is_invalid_if_color_is_not_in_conditions() {
        let mut conditions: HashMap<&str, u32> = HashMap::new();
        conditions.insert("Red", 6);
        conditions.insert("Yellow", 99);
        assert_eq!(is_game_valid("3 Blue, 5 Red, 8 Yellow", &conditions), false);
    }

    #[test]
    fn test_run_pass() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let mut max_values: HashMap<&str, u32> = HashMap::new();
        max_values.insert("red", 12);
        max_values.insert("green", 13);
        max_values.insert("blue", 14);
        let split = split_game_number_and_sets(game);
        assert_eq!(is_game_valid(split.1, &max_values), true);
    }
}