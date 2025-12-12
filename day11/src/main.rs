use std::collections::HashMap;

use utils::inputs::read_lines;

pub fn main() {
    let lines = read_lines("./day11/input.txt");

    let part1: u64;
    let part2: u64;

    let connections: HashMap<String, Vec<String>> = lines
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(": ").collect();
            let node = parts[0].to_string();
            let conns: Vec<String> = parts[1].split(' ').map(|s| s.to_string()).collect();
            (node, conns)
        })
        .collect();

    // initialize caches for memoization, can be reused between different path counts
    let mut out_cache = init_cache("out");
    let mut dac_cache = init_cache("dac");
    let mut fft_cache = init_cache("fft");

    part1 = count_connections_from_to(&mut out_cache, &connections, "you", "out");

    // count subpaths: svr -> dac -> fft -> out
    let svr_dac = count_connections_from_to(&mut dac_cache, &connections, "svr", "dac");
    let dac_fft = count_connections_from_to(&mut fft_cache, &connections, "dac", "fft");
    let fft_out = count_connections_from_to(&mut out_cache, &connections, "fft", "out");

    // count subpaths: svr -> fft -> dac -> out
    let svr_fft = count_connections_from_to(&mut fft_cache, &connections, "svr", "fft");
    let fft_dac = count_connections_from_to(&mut dac_cache, &connections, "fft", "dac");
    let dac_out = count_connections_from_to(&mut out_cache, &connections, "dac", "out");

    // total paths is sum of both subpath combinations
    part2 = svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out;

    println!("day  11");
    println!("  - part 1: {}", part1); // 585
    println!("  - part 2: {}", part2); // 349322478796032
}

fn count_connections_from_to(
    cache: &mut HashMap<String, u64>,
    connections: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
) -> u64 {
    if let Some(value) = cache.get(start) {
        return *value;
    }

    let mut count: u64 = 0;
    for conn in connections.get(start).unwrap() {
        count += count_connections_from_to(cache, connections, conn, end);
    }
    cache.insert(start.to_string(), count);

    count
}

fn init_cache(target: &str) -> HashMap<String, u64> {
    // initialize cache with target node
    let mut cache = HashMap::from([(target.to_string(), 1)]);
    if target != "out" {
        // out is terminal node, so it has no outgoing connections
        cache.insert("out".to_string(), 0);
    }
    cache
}
