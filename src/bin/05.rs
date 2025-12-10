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

fn part2(s: &String) -> u64 {
    let mut fresh: Vec<(u64, u64)> = Vec::new();

    for line in s.lines() {
        if line.trim().is_empty() {
            break;
        }
        
        let limits: Vec<&str> = line.split('-').collect();
        let l1: u64 = limits[0].trim().parse::<u64>().unwrap();
        let h1: u64 = limits[1].trim().parse::<u64>().unwrap();

        fresh.push((l1, h1));
    }

    loop {
        let mut changed: bool = false;

        for i in 0..fresh.len() {
            for j in (i+1)..fresh.len() {
                let l1 = fresh.get(i).unwrap().0;
                let h1 = fresh.get(i).unwrap().1;
                let l2 = fresh.get(j).unwrap().0;
                let h2 = fresh.get(j).unwrap().1;

                if h1 >= h2 && l1 <= l2 {
                    fresh.remove(j);
                    changed = true;
                    break;
                }

                if h1 <= h2 && l1 >= l2 {
                    fresh.remove(i);
                    changed = true;
                    break;
                }

                if h1 >= h2 && l1 >= l2 && h2 >= l1 {
                    if i < j {
                        fresh.remove(j);
                        fresh.remove(i);
                    } else {
                        fresh.remove(i);
                        fresh.remove(j);
                    }
                    fresh.push((l2, h1));
                    changed = true;
                    break;
                }

                if h1 <= h2 && l1 <= l2 && h1 >= l2 {
                    if i < j {
                        fresh.remove(j);
                        fresh.remove(i);
                    } else {
                        fresh.remove(i);
                        fresh.remove(j);
                    }
                    fresh.push((l1, h2));
                    changed = true;
                    break;
                }
            }

            if changed {
                break;
            }
        }

        if !changed {
            break;
        }
    }

    let mut count: u64 = 0;
    for intervals in fresh {
        count += intervals.1 - intervals.0 + 1;
    }
 
    return count;
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
