use std::fs;

fn part1(s: &String) -> u32 {
    todo!();
}

fn part2(s: &String) -> u32 {
    todo!();
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
