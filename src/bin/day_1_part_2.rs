use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./inputs/day_1.txt")?;
    let reader = BufReader::new(file);

    let mut fuel: i64 = 0;

    for line in reader.lines() {
        if let Ok(s) = line {
            let mut n = s.parse::<i64>().expect("nan");
            while n >= 0 {
                let f = (n / 3) - 2;
                if f < 0 {
                    break;
                }
                n = f;
                fuel += f;
            }
        }
    }

    println!("part 2 is {}", fuel);

    Ok(())
}
