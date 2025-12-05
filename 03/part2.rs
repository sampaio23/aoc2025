use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("input");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut sum: u64 = 0;

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
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
        }
    };

    println!("Result: {}", sum);
}
