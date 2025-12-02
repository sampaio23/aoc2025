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
        }
    };

    println!("Result: {}", sum);
}
