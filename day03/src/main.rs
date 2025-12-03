use utils::inputs::read_lines;

pub fn main() {
    let lines = read_lines("./day03/input.txt");

    let mut capacity_sum: u64 = 0;
    let mut capacity_sum2: u64 = 0;

    for line in lines {
        let line = line.unwrap();
        let digits: Vec<u64> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        capacity_sum += find_capacity(&digits, 2);
        capacity_sum2 += find_capacity(&digits, 12);
    }

    println!("day  3");
    println!("  - part 1: {}", capacity_sum); // 17535
    println!("  - part 2: {}", capacity_sum2); // 173577199527257
}

fn find_largest_in_range(numbers: &Vec<u64>, start: usize, end: usize) -> (u64, usize) {
    let mut biggest: u64 = numbers[start];
    let mut biggest_index: usize = start;
    for index in start + 1..end {
        if numbers[index] > biggest {
            biggest = numbers[index];
            biggest_index = index;
        }
    }
    (biggest, biggest_index)
}

fn find_capacity(numbers: &Vec<u64>, num_batteries: usize) -> u64 {
    let mut batteries = vec![0u64; num_batteries as usize];
    let mut offset = 0;
    for i in 0..num_batteries {
        let (num, idx) =
            find_largest_in_range(numbers, offset, numbers.len() - num_batteries + i + 1);
        batteries[i] = num;
        offset = idx + 1;
    }
    let mut capacity: u64 = 0;
    for battery in batteries {
        capacity = capacity * 10 + battery;
    }
    capacity
}
