use std::fs;

fn part1(s: &String) -> u32 {
    let mut total = 0;

    let width = s.lines().nth(0).unwrap().len();
    let height = s.lines().count();

    let mut vec = vec![0; (width + 2) * (height + 2)];

    for i in 1..=height {
        for j in 1..=width {
            if s.lines().nth(i-1).unwrap().chars().nth(j-1).unwrap() == '@' {
                vec[i*(width+2) + j] = 1;
            }
        }
    }

    for i in 1..=height {
        for j in 1..=width {
            if vec[i*(width+2) + j] == 0 {
                continue;
            }

            let mut sum = 0;
            for k in 0..=2 {
                for l in 0..=2 {
                    if k != 1 || l != 1 {
                        sum += vec[(i+k-1)*(width+2) + (j+l-1)];
                    }
                }
            }
            if sum < 4 {
                total += 1;
            }
        }
    }

    return total;
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
