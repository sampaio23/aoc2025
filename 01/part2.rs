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

    let mut dial: i32 = 50;
    let mut count: u32 = 0;

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {
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
        }
    };

    println!("Result: {}", count);
}
