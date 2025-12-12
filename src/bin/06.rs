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

fn part2(s: &String) -> u64 {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    for index in 0..s.lines().count() {
        let line = s.lines().nth(index).unwrap();
        let mut i = 0;
        for c in line.chars() {
            if index == 0 {
                let mut v: Vec<char> = Vec::new();
                v.push(c);
                matrix.push(v);
            } else {
                matrix.get_mut(i).unwrap().push(c);
                i += 1;
            }
        }
    }

    let width = matrix.len();
    let height = matrix[0].len();

    let mut result: Vec<u64> = Vec::new();
    let mut current: Vec<u64> = Vec::new();
    let mut current_num: Vec<u64> = Vec::new();
    for i in 0..width {
        for j in 0..height {
            let cur = matrix[width-1-i][j];
            if cur == '*' {
                let mut cur_num = 0;
                for num in &current_num {
                    cur_num = cur_num * 10 + num;
                }
                current.push(cur_num);
                current_num.clear();
                result.push(current.iter().product::<u64>());
                current.clear();
            } else if cur == '+' {
                let mut cur_num = 0;
                for num in &current_num {
                    cur_num = cur_num * 10 + num;
                }
                current.push(cur_num);
                current_num.clear();
                result.push(current.iter().sum::<u64>());
                current.clear();
            } else if cur != ' ' {
                current_num.push(cur.to_digit(10).unwrap() as u64);
            }
        }
        let mut cur_num = 0;
        if current_num.len() > 0 {
            for num in &current_num {
                cur_num = cur_num * 10 + num;
            }
            current.push(cur_num);
            current_num.clear();
        }
    }

    return result.iter().sum::<u64>();
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
