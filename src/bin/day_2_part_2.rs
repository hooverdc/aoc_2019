use std::{fs, usize};

fn calc (mut opcodes: Vec<usize>, noun: usize, verb: usize) -> usize {
    opcodes[1] = noun;
    opcodes[2] = verb;

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

    opcodes[0]

}

fn main() -> Result<(), ()> {
    let input = fs::read_to_string("./inputs/day_2.txt").expect("???");
    let opcodes: Vec<usize> = input
        .split(",")
        .map(|s| {
            println!("value {}", s);
            s.parse::<usize>().expect("nan")
        })
        .collect();
    
    let mut soln = 0;

    for i in 0..100 {
        for j in 0..100 {
            let output = calc(opcodes.clone(), i,j);
            if output == 19690720 {
                soln = (100 * i) + j;
            }
        }
    }
    
    println!("part 2 is {}", soln);

    Ok(())
}
