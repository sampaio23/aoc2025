use std::fs;

fn part1(s: &String) -> u32 {
    let mut dial: i16 = 50;
    let mut count: u32 = 0;

    for line in s.lines() {
        let cur: i16 = line.get(1..).unwrap().parse::<i16>().unwrap();
        if line.get(0..1).unwrap() == "R" {
            dial += cur;
        } else {
            dial -= cur;
        }
        dial = dial % 100;

        if dial == 0 {
            count += 1;
        }
    }

    return count;
}

fn part2(s: &String) -> u32 {
    let mut dial: i32 = 50;
    let mut count: u32 = 0;

    for line in s.lines() {
        let cur: u16 = line.get(1..).unwrap().parse::<u16>().unwrap();
        let full_rotations: u16 = cur / 100;
        count += full_rotations as u32;

        let last_rotation: u16 = cur % 100;
        let cur_dial: i32 = dial;

        if line.get(0..1).unwrap() == "R" {
            dial += last_rotation as i32;
            if dial >= 100 && cur_dial != 0 {
                count += 1;
            }
        } else {
            dial -= last_rotation as i32;
            if dial <= 0 && cur_dial != 0 {
                count += 1;
            }
        }
        dial = dial % 100;
        if dial < 0 {
            dial += 100
        }
    }

    return count;
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
