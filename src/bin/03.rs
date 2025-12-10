use std::fs;

fn part1(s: &String) -> u64 {
    let mut sum: u64 = 0;

    for line in s.lines() {
        let length = line.trim().chars().count();
        let mut first_index: usize = 0;

        let mut max: u32 = 0;
        for i in 0..(length-1) {
            if line.trim().chars().nth(i).unwrap().to_digit(10).unwrap() > max {
                first_index = i;
                max = line.trim().chars().nth(i).unwrap().to_digit(10).unwrap();
            }
        }
        let first = max;
        max = 0;
        for i in (first_index+1)..length {
            if line.trim().chars().nth(i).unwrap().to_digit(10).unwrap() > max {
                max = line.trim().chars().nth(i).unwrap().to_digit(10).unwrap();
            }
        }

        sum += 10*(first as u64) + (max as u64);
    }

    return sum;
}

fn part2(s: &String) -> u64 {
    let mut sum: u64 = 0;

    for line in s.lines() {
        let length = line.trim().chars().count();
        let mut first_index: usize = 0;
        let mut max: u64 = 0;
        let mut new_max: u32 = 0;

        let digits: usize = 12;
        for j in 1..=digits {
            for i in first_index..(length-(digits-j)) {
                if line.trim().chars().nth(i).unwrap().to_digit(10).unwrap() > new_max {
                    first_index = i;
                    new_max = line.trim().chars().nth(i).unwrap().to_digit(10).unwrap();
                }
            }
            max = 10*max + (new_max as u64);
            new_max = 0;
            first_index += 1;
        }

        sum += max as u64;
    }

    return sum;
}

fn main() {
    let example: String = fs::read_to_string("03/example").unwrap();
    let input: String = fs::read_to_string("03/input").unwrap();

    println!("Part 1\n======");
    println!("    Example: {}", part1(&example));
    println!("    Input:   {}", part1(&input));
    println!("Part 2\n======");
    println!("    Example: {}", part2(&example));
    println!("    Input:   {}", part2(&input));
}
