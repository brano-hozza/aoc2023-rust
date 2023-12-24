use std::{fmt::Error, fs};

struct State {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() -> Result<(), Error> {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut total_power = 0;
    for line in contents.lines() {
        let res: Vec<&str> = line.split(":").collect();
        let ball_groups: Vec<&str> = res.get(1).unwrap().split(";").collect();
        let mut max_state = State {
            red: 0,
            green: 0,
            blue: 0,
        };
        for group in ball_groups {
            let balls = group.split(",");
            let mut state = State {
                red: 0,
                green: 0,
                blue: 0,
            };
            for ball in balls {
                let x: Vec<&str> = ball.trim().split(" ").collect();
                let amount = x.get(0).unwrap().parse::<u32>().unwrap();
                match x.get(1).unwrap() {
                    &"red" => state.red += amount,
                    &"green" => state.green += amount,
                    &"blue" => state.blue += amount,
                    _ => {}
                }
            }
            if state.red > max_state.red {
                max_state.red = state.red;
            }
            if state.green > max_state.green {
                max_state.green = state.green;
            }
            if state.blue > max_state.blue {
                max_state.blue = state.blue;
            }
        }
        total_power += max_state.red * max_state.green * max_state.blue;
    }
    println!("Total: {}", total_power);

    Ok(())
}
