use utils::inputs::read_lines;

pub fn main() {
    let lines = read_lines("./day09/input.txt");

    let mut max_area: usize = 0;
    let mut max_red_green_area: usize = 0;

    // parse tile coordinates
    let tile_coords: Box<[(usize, usize)]> = lines
        .map(|line| {
            let line = line.unwrap();
            let coords: Box<[usize]> = line.split(",").map(|s| s.parse().unwrap()).collect();
            (coords[0], coords[1])
        })
        .collect();

    // trace edges of tiles
    let mut lines: Vec<(usize, usize, usize, usize)> = Vec::new();
    let (mut x1, mut y1) = *tile_coords.last().unwrap();
    for &(x2, y2) in tile_coords.iter() {
        lines.push((x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2)));
        (x1, y1) = (x2, y2);
    }

    // iterate over each pair of tiles
    for i in 0..tile_coords.len() {
        let (x1, y1) = tile_coords[i];
        for j in i + 1..tile_coords.len() {
            let (x2, y2) = tile_coords[j];

            let x_min = x1.min(x2);
            let x_max = x1.max(x2);
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);

            let area = (x_max - x_min + 1) * (y_max - y_min + 1);

            // part1: check for any rectangles
            if area > max_area {
                max_area = area;
            }

            // part2: check for red-green rectangles
            if area > max_red_green_area {
                let mut intersects = false;
                for &(lx1, ly1, lx2, ly2) in lines.iter() {
                    // check if line intersects rectangle
                    if ly1 > y_min && ly2 < y_max && lx1 < x_max && lx2 > x_min {
                        intersects = true;
                        break;
                    }
                }
                if !intersects {
                    max_red_green_area = area;
                }
            }
        }
    }

    println!("day  9");
    println!("  - part 1: {}", max_area); // 4764078684
    println!("  - part 2: {}", max_red_green_area); // 1652344888
}
