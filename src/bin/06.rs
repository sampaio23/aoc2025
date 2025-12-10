use std::fs;

fn part1(s: &String) -> u64 {
    let mut matrix: Vec<Vec<u64>> = Vec::new();
    let mut results: Vec<u64> = Vec::new();

    for index in 0..s.lines().count() {
        let line = s.lines().nth(index).unwrap();
        let line_vec: Vec<&str> = line.split(" ").collect();
        let mut i = 0;
        for item in line_vec {
            if item.trim() == "" {
                continue;
            }

            if index == (s.lines().count()-1) {
                if item.trim() == "+" {
                    matrix.get_mut(i).unwrap().push(0);
                    results.push(0);
                } else {
                    matrix.get_mut(i).unwrap().push(1);
                    results.push(1);
                }
                i += 1;
                continue;
            }

            let item_num: u64 = item.trim().parse::<u64>().unwrap();
            if index == 0 {
                let mut v: Vec<u64> = Vec::new();
                v.push(item_num);
                matrix.push(v);
            } else {
                matrix.get_mut(i).unwrap().push(item_num);
                i += 1;
            }
        }
    }

    for i in 0..matrix.len() {
        let op = matrix.get(i).unwrap().last().unwrap();
        for j in 0..(matrix.get(0).unwrap().len()-1) {
            if *op == 0 {
                results[i] += matrix.get(i).unwrap().get(j).unwrap();
            } else {
                results[i] *= matrix.get(i).unwrap().get(j).unwrap();
            }
        }
    }

    return results.iter().sum::<u64>();
}

fn part2(s: &String) -> u32 {
    todo!();
}

fn main() {
    let example: String = fs::read_to_string("06/example").unwrap();
    let input: String = fs::read_to_string("06/input").unwrap();

    println!("Part 1\n======");
    println!("    Example: {}", part1(&example));
    println!("    Input:   {}", part1(&input));
    println!("Part 2\n======");
    println!("    Example: {}", part2(&example));
    println!("    Input:   {}", part2(&input));
}
