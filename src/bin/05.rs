use std::fs;

fn part1(s: &String) -> u32 {
    let mut count = 0;

    let mut reading_ids: bool = false;
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in s.lines() {
        if line.trim().is_empty() {
            reading_ids = true;
            continue;
        }
        if !reading_ids {
            let limits: Vec<&str> = line.split('-').collect();
            let lower_limit: u64 = limits[0].trim().parse::<u64>().unwrap();
            let upper_limit: u64 = limits[1].trim().parse::<u64>().unwrap();
            ranges.push((lower_limit, upper_limit));
        } else {
            let id: u64 = line.trim().parse::<u64>().unwrap();
            for range in &ranges {
                if id >= range.0 && id <= range.1 {
                    count += 1;
                    break;
                }
            }
        }
    }
    
    return count;
}

fn part2(s: &String) -> u32 {
    todo!();
}

fn main() {
    let example: String = fs::read_to_string("05/example").unwrap();
    let input: String = fs::read_to_string("05/input").unwrap();

    println!("Part 1\n======");
    println!("    Example: {}", part1(&example));
    println!("    Input:   {}", part1(&input));
    println!("Part 2\n======");
    println!("    Example: {}", part2(&example));
    println!("    Input:   {}", part2(&input));
}
