use std::fs;

fn part1(s: &String) -> u64 {
    let mut sum: u64 = 0;

    let intervals: Vec<&str> = s.split(',').collect();
    for interval in intervals {
        let limits: Vec<&str> = interval.split('-').collect();
        let lower_limit: u64 = limits[0].trim().parse::<u64>().unwrap();
        let upper_limit: u64 = limits[1].trim().parse::<u64>().unwrap();
        for i in lower_limit..=upper_limit {
            let num_digits: u32 = i.ilog10() + 1;
            if num_digits % 2 == 0 {
                let pow_hnd: u64 = 10u32.pow(num_digits/2) as u64;
                if i % pow_hnd == i / pow_hnd {
                    sum += i;
                }
            }
        }
    }

    return sum;
}

fn part2(s: &String) -> u64 {
    let mut sum: u64 = 0;

    let intervals: Vec<&str> = s.split(',').collect();
    for interval in intervals {
        let limits: Vec<&str> = interval.split('-').collect();
        let lower_limit: u64 = limits[0].trim().parse::<u64>().unwrap();
        let upper_limit: u64 = limits[1].trim().parse::<u64>().unwrap();
        for i in lower_limit..=upper_limit {
            let num_digits: u32 = i.ilog10() + 1;
            for div in 2..=num_digits {
                if num_digits % div == 0 {
                    let pow_hnd: u64 = 10u32.pow(num_digits/div) as u64;
                    let mut cur: u64 = i;
                    let part: u64 = i % pow_hnd;
                    while cur != 0 {
                        if cur % pow_hnd != part {
                            break;
                        }
                        cur = cur / pow_hnd;
                    }
                    if cur == 0 {
                        sum += i;
                        break;
                    }
                }
            }
        }
    }

    return sum;
}

fn main() {
    let example: String = fs::read_to_string("example").unwrap();
    let input: String = fs::read_to_string("input").unwrap();

    println!("Part 1\n======");
    println!("    Example: {}", part1(&example));
    println!("    Input:   {}", part1(&input));
    println!("Part 2\n======");
    println!("    Example: {}", part2(&example));
    println!("    Input:   {}", part2(&input));
}
