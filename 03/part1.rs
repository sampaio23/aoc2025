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
        }
    };

    println!("Result: {}", sum);
}
