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

    let mut dial: i16 = 50;
    let mut count: u32 = 0;

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
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
        }
    };

    println!("Result: {}", count);
}
