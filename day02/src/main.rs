use regex::Regex;
use utils::inputs::read_lines;

pub fn main() {
    let lines = read_lines("./day02/input.txt");

    let mut invalid_ids_sum: u64 = 0;
    let mut invalid_ids_sum2: u64 = 0;

    let re = Regex::new(r"(\d+-\d+)(,|$)").unwrap();

    for line in lines {
        let line = line.unwrap();
        let caps = re.captures_iter(&line);
        for cap in caps {
            let range = &cap[1];
            let bounds: Vec<u64> = range.split('-').map(|s| s.parse().unwrap()).collect();
            let start = bounds[0];
            let end = bounds[1];
            for id in start..=end {
                if is_invalid_id_part1(id) {
                    invalid_ids_sum += id;
                }

                if is_invalid_id_part2(id) {
                    invalid_ids_sum2 += id;
                }
            }
        }
    }

    println!("day  2");
    println!("  - part 1: {}", invalid_ids_sum); // 19386344315
    println!("  - part 2: {}", invalid_ids_sum2); // 34421651192
}

fn is_invalid_id_part1(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    if len % 2 != 0 {
        return false;
    }

    let first = &id_str[0..len / 2];
    let second = &id_str[len / 2..len];
    first == second
}

fn is_invalid_id_part2(id: u64) -> bool {
    let id_str = id.to_string();
    let id_len = id_str.len();

    for group_len in 1..=(id_len / 2) {
        if id_len % group_len != 0 {
            continue;
        }

        let first = &id_str[0..group_len];

        let mut mismatch_found = false;

        for offset in (group_len..id_len).step_by(group_len) {
            let next = &id_str[offset..offset + group_len];

            if first != next {
                mismatch_found = true;
                break;
            }
        }

        if !mismatch_found {
            return true;
        }
    }
    false
}
