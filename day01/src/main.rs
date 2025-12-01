use regex::Regex;
use utils::inputs::read_lines;

pub fn main() {
    let lines = read_lines("./day01/input.txt").unwrap();

    let mut count_on_zero: i32 = 0;
    let mut count_over_zero: i32 = 0;
    let mut position: i32 = 50;

    let re = Regex::new(r"(?<direction>L|R)(?<steps>\d+)").unwrap();
    for line in lines {
        let line = line.unwrap();
        let caps = re.captures(&line).unwrap();
        let direction = &caps["direction"];
        let mut steps: i32 = caps["steps"].parse().unwrap();

        if steps >= 100 {
            let full_rotations = steps / 100;
            count_over_zero += full_rotations;
            steps = steps % 100;
        }

        match direction {
            "L" => {
                if position != 0 && steps >= position {
                    count_over_zero += 1;
                }
                position = position - steps;
                if position < 0 {
                    position += 100;
                }
            }
            "R" => {
                if position + steps >= 100 {
                    count_over_zero += 1;
                }
                position = (position + steps) % 100;
            }
            _ => panic!("Invalid direction"),
        }

        if position == 0 {
            count_on_zero += 1;
        }
    }

    println!("day  1");
    println!("  - part 1: {}", count_on_zero); // 1011
    println!("  - part 2: {}", count_over_zero); // 5937
}
