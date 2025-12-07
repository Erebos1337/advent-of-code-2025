use utils::inputs::read_lines;

pub fn main() {
    let lines = read_lines("./day06/input.txt");

    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    let mut operations: Vec<String> = Vec::new();
    let mut operants: Vec<Vec<u64>> = Vec::new();
    let mut operants2: Vec<Vec<u64>> = Vec::new();

    let mut char_grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let unwrapped_line = line.unwrap();

        // read lines horizontally for part 1
        let parts = unwrapped_line.split_whitespace().collect::<Vec<&str>>();

        if parts[0] == "+" || parts[0] == "*" {
            // read operators from last line
            for part in parts {
                operations.push(part.to_string());
            }
        } else {
            // read operants for part 1 from other lines
            for i in 0..parts.len() {
                if operants.len() <= i {
                    operants.push(Vec::new());
                }
                operants[i].push(parts[i].parse::<u64>().unwrap());
            }
        }

        // read lines into char grid for part 2
        let chars = unwrapped_line.chars().collect::<Vec<char>>();
        char_grid.push(chars);
    }

    let mut curr_equation: Vec<u64> = Vec::new();
    for col in 0..char_grid[0].len() {
        if col != 0 && char_grid[char_grid.len() - 1][col] != ' ' {
            operants2.push(curr_equation);
            curr_equation = Vec::new();
        }
        let mut num = 0;
        let mut col_empty = true;
        for row in 0..char_grid.len() - 1 {
            let c = char_grid[row][col];
            if c != ' ' {
                col_empty = false;
                num *= 10;
                num += c.to_digit(10).unwrap() as u64;
            }
        }
        if !col_empty {
            curr_equation.push(num);
        }
    }
    operants2.push(curr_equation);

    // do calculations for part 1
    for i in 0..operants.len() {
        let nums = &operants[i];
        let op = &operations[i];

        part1 += match op.as_str() {
            "+" => nums.iter().sum(),
            "*" => nums.iter().product(),
            _ => 0,
        };
    }

    // do calculations for part 2
    for i in 0..operants2.len() {
        let nums = &operants2[i];
        let op = &operations[i];

        part2 += match op.as_str() {
            "+" => nums.iter().sum(),
            "*" => nums.iter().product(),
            _ => 0,
        };
    }

    println!("day  6");
    println!("  - part 1: {}", part1); // 5782351442566
    println!("  - part 2: {}", part2); // 10194584711842
}
