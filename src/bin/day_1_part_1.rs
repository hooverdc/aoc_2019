use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./inputs/day_1_part_1.txt")?;
    let reader = BufReader::new(file);

    let mut fuel: u32 = 0;

    for line in reader.lines() {
        if let Ok(s) = line {
            let n = s.parse::<u32>().expect("nan");
            fuel += (n / 3) - 2

        }
    }

    println!("part 1 is {}", fuel);

    Ok(())
}
