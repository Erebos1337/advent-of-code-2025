use std::collections::HashMap;
use utils::inputs::read_lines;

pub fn main() {
    let lines = read_lines("./day07/input.txt");

    let mut num_splits: u64 = 0;

    let mut timelines: HashMap<usize, u64> = HashMap::new();

    let mut is_first_line = true;
    for line in lines {
        let unwrapped_line = line.unwrap();

        if is_first_line {
            is_first_line = false;
            let start = unwrapped_line.chars().position(|c| c == 'S').unwrap();
            timelines.insert(start, 1);
        } else {
            let chars = unwrapped_line.chars().collect::<Vec<char>>();

            let mut new_timelines: HashMap<usize, u64> = HashMap::new();

            for (pos, count) in timelines {
                if chars[pos] == '^' {
                    num_splits += 1;
                    *new_timelines.entry(pos - 1).or_insert(0) += count;
                    *new_timelines.entry(pos + 1).or_insert(0) += count;
                } else {
                    *new_timelines.entry(pos).or_insert(0) += count;
                }
            }
            timelines = new_timelines;
        }
    }

    let num_timelines: u64 = timelines.iter().map(|(_pos, count)| count).sum();

    println!("day  7");
    println!("  - part 1: {}", num_splits); // 1560
    println!("  - part 2: {}", num_timelines); // 25592971184998
}
