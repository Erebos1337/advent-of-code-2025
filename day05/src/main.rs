use utils::inputs::read_lines;

pub fn main() {
    let lines = read_lines("./day05/input.txt");

    let mut num_fresh_ingredients: u64 = 0;
    let mut num_available_ingredients: u64 = 0;

    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut ingredients: Vec<u64> = vec![];

    let mut reading_ranges = true;
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            reading_ranges = false;
            continue;
        }

        if reading_ranges {
            let parts: Vec<&str> = line.split('-').collect();
            let start: u64 = parts[0].parse().unwrap();
            let end: u64 = parts[1].parse().unwrap();
            ranges.push((start, end));
        } else {
            let ingredient: u64 = line.parse().unwrap();
            ingredients.push(ingredient);
        }
    }

    let available_ranges_merged = merge_ranges(&ranges);
    
    // count fresh ingredients
    for ingredient in &ingredients {
        for (start, end) in &available_ranges_merged {
            // if range contains ingredient, count it as fresh and break
            if ingredient >= start && ingredient <= end {
                num_fresh_ingredients += 1;
                break;
            }
            // ranges are sorted,
            // so if current range is higher than ingredient, there can't be any match
            if ingredient < start {
                break;
            }
        }
    }

    // count total available ingredients
    for range in &available_ranges_merged {
        let (start, end) = *range;
        num_available_ingredients += end - start + 1;
    }

    println!("day  5");
    println!("  - part 1: {}", num_fresh_ingredients); // 707
    println!("  - part 2: {}", num_available_ingredients); // 361615643045059
}

fn merge_ranges(original_ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    // sort ranges by start value
    let mut sorted_ranges = original_ranges.to_vec();
    sorted_ranges.sort_by_key(|k| k.0);

    let mut merged: Vec<(u64, u64)> = vec![];
    let mut i = 0usize;
    while i < sorted_ranges.len() {
        // start new range
        let (start, mut end) = sorted_ranges[i];
        i += 1;
        // as long as next range overlaps or touches current range, extend current range
        while i < sorted_ranges.len() && sorted_ranges[i].0 <= end + 1 {
            end = end.max(sorted_ranges[i].1);
            i += 1;
        }
        // next range is not connected to current range or no more ranges left,
        // so push merged range to merged list
        merged.push((start, end));
    }
    merged
}
