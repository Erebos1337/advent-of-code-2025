use utils::inputs::read_lines;

pub fn main() {
    let lines = read_lines("./day08/input.txt");

    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    // parse junction coordinates
    let junction_coords: Box<[(u64, u64, u64)]> = lines
        .map(|line| {
            let line = line.unwrap();
            let coords: Box<[u64]> = line.split(",").map(|s| s.parse().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        })
        .collect();

    // calculate and sort all pairwise distances between junctions
    let mut junction_distances: Vec<(u64, usize, usize)> = Vec::new();
    for i in 0..junction_coords.len() {
        let (x1, y1, z1) = junction_coords[i];
        for j in i + 1..junction_coords.len() {
            let (x2, y2, z2) = junction_coords[j];
            // no need to calculate sqrt since we only care about relative distances
            let distance = x2.abs_diff(x1).pow(2) + y2.abs_diff(y1).pow(2) + z2.abs_diff(z1).pow(2);
            junction_distances.push((distance, i, j));
        }
    }
    junction_distances.sort_by_key(|k| k.0);

    // assign each junction to its own circuit initially
    let mut circuit_assignments: Box<[usize]> = (0..junction_coords.len()).collect();

    let mut connections_attempted: usize = 0;
    let mut connections_made: usize = 0;
    for (_distance, i, j) in junction_distances {
        let (circuit_i, circuit_j) = (circuit_assignments[i], circuit_assignments[j]);

        // if junctions are in different circuits, merge circuits (Kruskal's algorithm)
        if circuit_i != circuit_j {
            // assign all junctions to first circuit regardless of efficiency
            for circuit in &mut circuit_assignments {
                if *circuit == circuit_j {
                    *circuit = circuit_i;
                }
            }
            connections_made += 1;
        }

        // after n-1 connections we have a spanning tree
        if connections_made == junction_coords.len() - 1 {
            part2 = calc_part2(junction_coords[i], junction_coords[j]);
            break;
        }

        // calculate part 1 after 1000 connections attempted
        connections_attempted += 1;
        if connections_attempted == 1000 {
            part1 = calc_part1(&circuit_assignments);
        }
    }

    println!("day  8");
    println!("  - part 1: {}", part1); // 244188
    println!("  - part 2: {}", part2); // 8361881885
}

fn calc_part1(circuit_assignments: &[usize]) -> u64 {
    let mut group_sizes = vec![0; circuit_assignments.len()];
    for &circuit in circuit_assignments {
        group_sizes[circuit] += 1;
    }
    group_sizes.sort_unstable();
    group_sizes.iter().rev().take(3).product()
}

fn calc_part2(junction1_coords: (u64, u64, u64), junction2_coords: (u64, u64, u64)) -> u64 {
    junction1_coords.0 * junction2_coords.0
}
