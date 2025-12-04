use utils::inputs::read_lines;

pub fn main() {
    let lines: Vec<_> = read_lines("./day04/input.txt").collect();

    let mut num_accessible_rolls: u64 = 0;
    let mut num_removed_rolls: u64 = 0;

    let size = lines.len();
    let mut grid = vec![vec!['.'; size]; size];

    for i in 0..size {
        let chars = lines[i].as_ref().unwrap().chars().collect::<Vec<char>>();
        for j in 0..size {
            grid[i][j] = chars[j];
        }
    }

    let mut is_round1 = true;
    let mut removed_roll = true;

    while removed_roll {
        removed_roll = false;
        for i in 0..size {
            for j in 0..size {
                if grid[i][j] == '@' {
                    let num_adjacents = count_adjacents(&grid, i, j);
                    if num_adjacents < 4 {
                        grid[i][j] = 'X';
                        removed_roll = true;
                    }
                }
            }
        }
        if removed_roll {
            num_removed_rolls += remove_rolls(&mut grid);
            if is_round1 {
                is_round1 = false;
                num_accessible_rolls = num_removed_rolls;
            }
        }
    }
    println!("day  4");
    println!("  - part 1: {}", num_accessible_rolls); // 1395
    println!("  - part 2: {}", num_removed_rolls); // 8451
}

fn count_adjacents(grid: &Vec<Vec<char>>, y: usize, x: usize) -> u64 {
    let len = grid.len();
    let y_min = if y == 0 { 0 } else { y - 1 };
    let y_max = if y == len - 1 { len - 1 } else { y + 1 };
    let x_min = if x == 0 { 0 } else { x - 1 };
    let x_max = if x == len - 1 { len - 1 } else { x + 1 };

    let mut count = 0;
    for i in y_min..=y_max {
        for j in x_min..=x_max {
            if !(i == y && j == x) && (grid[i][j] != '.') {
                count += 1;
            }
        }
    }
    count
}

fn remove_rolls(grid: &mut Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    let size = grid.len();
    for i in 0..size {
        for j in 0..size {
            if grid[i][j] == 'X' {
                grid[i][j] = '.';
                count += 1;
            }
        }
    }
    count
}
