use std::{cmp, fmt::Error, fs};

type Buffer = [String; 3];

fn add_to_buffer(buff: &mut Buffer, line: String) {
    buff[0] = buff[1].clone();
    buff[1] = buff[2].clone();
    buff[2] = line.clone();
}
/**
 * Check if string is next to a special character (non-numeric and different from '.')
 * @param buff: Buffer of 3 lines to check, string is in middle line
 * @param str_idx: Index of the first character of the string
 * @param str_size: Size of the string
 */
fn check_string(buff: &Buffer, str_idx: usize, str_size: usize) -> bool {
    let before_idx = if str_idx > 0 { str_idx - 1 } else { str_idx };

    if buff[0].len() > 0 {
        // Check first line
        for i in before_idx..=cmp::min(str_idx + str_size, buff[0].len()) {
            if let Some(x) = buff[0].chars().nth(i) {
                if !x.is_numeric() && x != '.' {
                    return true;
                }
            }
        }
    }

    // Check second line

    if let Some(before) = buff[1].chars().nth(before_idx) {
        if !before.is_numeric() && before != '.' {
            return true;
        }
    }

    if let Some(after) = buff[1].chars().nth(str_idx + str_size) {
        if !after.is_numeric() && after != '.' {
            return true;
        }
    }

    // Check third line
    for i in before_idx..=cmp::min(str_idx + str_size, buff[2].len()) {
        if let Some(x) = buff[2].chars().nth(i) {
            if !x.is_numeric() && x != '.' {
                return true;
            }
        }
    }

    return false;
}

fn main() -> Result<(), Error> {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let mut buffer: Buffer = [String::from(""), String::from(""), String::from("")];
    let mut counter = 0;

    let mut lines = contents.lines();
    let mut last_line = String::from(lines.next().unwrap());
    add_to_buffer(&mut buffer, String::from(last_line.clone()));
    for line in contents.lines() {
        add_to_buffer(&mut buffer, String::from(line));
        let mut raw_number = String::new();
        let mut start_idx: Option<usize> = None;
        for (idx, char) in last_line.chars().enumerate() {
            if !char.is_numeric() {
                if let Some(i) = start_idx {
                    if check_string(&buffer, i, raw_number.len()) {
                        let number = raw_number.parse::<u32>().unwrap();
                        counter += number;
                        print!("{} ", number);
                    }
                    raw_number.clear();
                    start_idx = None;
                }
                continue;
            }
            if char.is_numeric() {
                raw_number.push(char);
                if start_idx == None {
                    start_idx = Some(idx)
                }
            }
        }
        if let Some(i) = start_idx {
            if check_string(&buffer, i, raw_number.len()) {
                let number = raw_number.parse::<u32>().unwrap();
                counter += number;
                print!("{} ", number);
            }
        }
        last_line = String::from(line);
    }
    add_to_buffer(&mut buffer, String::from(""));
    let mut raw_number = String::new();
    let mut start_idx: Option<usize> = None;
    for (idx, char) in last_line.chars().enumerate() {
        if !char.is_numeric() {
            if let Some(i) = start_idx {
                if check_string(&buffer, i, raw_number.len()) {
                    let number = raw_number.parse::<u32>().unwrap();
                    counter += number;
                    print!("{} ", number);
                }
                raw_number.clear();
                start_idx = None;
            }
            continue;
        }
        if char.is_numeric() {
            raw_number.push(char);
            if start_idx == None {
                start_idx = Some(idx)
            }
        }
    }
    if let Some(i) = start_idx {
        if check_string(&buffer, i, raw_number.len()) {
            let number = raw_number.parse::<u32>().unwrap();
            counter += number;
            print!("{} ", number);
        }
    }
    println!("\nTotal: {}", counter);
    Ok(())
}
