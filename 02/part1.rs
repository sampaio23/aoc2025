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
                    if num_digits % 2 == 0 {
                        let pow_hnd: u64 = 10u32.pow(num_digits/2) as u64;
                        if i % pow_hnd == i / pow_hnd {
                            sum += i;
                        }
                    }
                }
            }
        }
    };

    println!("Result: {}", sum);
}
