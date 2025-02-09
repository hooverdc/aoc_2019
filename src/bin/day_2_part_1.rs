use std::fs;

fn main() -> Result<(), ()> {
    let input = fs::read_to_string("./inputs/day_2.txt").expect("???");
    let mut opcodes: Vec<usize> = input
        .split(",")
        .map(|s| {
            println!("value {}", s);
            s.parse::<usize>().expect("nan")
        })
        .collect();

    opcodes[1] = 12;
    opcodes[2] = 2;

    let mut ptr = 0;

    loop {
        let opcode = opcodes[ptr];
        match opcode {
            1 => {
                let a = opcodes[ptr + 1];
                let b = opcodes[ptr + 2];
                let c = opcodes[ptr + 3];
                opcodes[c] = opcodes[a] + opcodes[b];
                ptr += 4
            }
            2 => {
                let a = opcodes[ptr + 1];
                let b = opcodes[ptr + 2];
                let c = opcodes[ptr + 3];
                opcodes[c] = opcodes[a] * opcodes[b];
                ptr += 4
            }
            99 => {
                break;
            }
            _ => {
                panic!("intcode machine broke")
            }
        }
    }

    println!("part 1 is {}", opcodes[0]);

    Ok(())
}
