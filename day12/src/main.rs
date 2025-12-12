use regex::Regex;
use utils::inputs::read_lines;

fn main() {
    let lines: Vec<_> = read_lines("./day12/input.txt")
        .map(|l| l.unwrap())
        .collect();

    let package_regex = Regex::new(r"^(\d+):$").unwrap();
    let region_regex = Regex::new(r"^(\d+)x(\d+): (.*)$").unwrap();

    let mut present_sizes: Vec<u32> = Vec::new();
    let mut regions: Vec<((u32, u32), Vec<u32>)> = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let package_caps = package_regex.is_match(&lines[i]);
        if package_caps {
            let mut present_size = 0;
            for j in 1..=3 {
                lines[i + j].chars().for_each(|c| {
                    if c == '#' {
                        present_size += 1;
                    }
                });
            }
            present_sizes.push(present_size);
            i += 5;
        } else {
            let region_caps = region_regex.captures(&lines[i]).unwrap();
            let width: u32 = region_caps[1].parse().unwrap();
            let length: u32 = region_caps[2].parse().unwrap();
            let present_counts: Vec<u32> = region_caps[3]
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect();
            regions.push(((width, length), present_counts));
            i += 1;
        }
    }

    let mut ok_regions = 0;
    for ((width, length), present_counts) in regions {
        // calculate minimum space needed to fit all presents
        let mut total_space_needed: u32 = 0;
        for (i, &count) in present_counts.iter().enumerate() {
            total_space_needed += count * present_sizes[i];
        }

        // calculate available area
        let available_area = width * length;

        // assuming perfect placement, if area can fit all presents, count this region as ok
        if total_space_needed <= available_area {
            ok_regions += 1;
        }
    }

    println!("day  12");
    println!("  - part 1: {}", ok_regions); // 546
}
