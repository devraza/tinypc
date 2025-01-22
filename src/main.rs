use std::fs::File;
use std::io::{self, Result, prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> Result<()> {
    let code = File::open("code.asm")?;
    let buffer = BufReader::new(&code);
    let mut accumulator = 0;

    let mut memory: HashMap<String, i64> = HashMap::new();

    for instruction in buffer.lines() {
        let ops: Vec<String> = instruction?
            .split_whitespace()
            .map(String::from)
            .collect();

        match ops[0].as_str() {
            "INP" => {
                let mut s = String::new();
                io::stdin().read_line(&mut s).unwrap();
                accumulator = s.trim().parse::<i64>().unwrap();
            }
            "OUT" => println!("{}", accumulator),
            "STA" => {
                memory.insert(
                    ops[1].clone(),
                    accumulator
                );
            }
            "LDA" => {
                match memory.get(&ops[1].clone()) {
                    Some(value) => accumulator = *value,
                    None => panic!(),
                }
            }
            "ADD" => accumulator += &ops[1].clone().parse::<i64>().unwrap(),
            "SUB" => accumulator -= &ops[1].clone().parse::<i64>().unwrap(),
            "HLT" => std::process::exit(0),
            &_ => {
                match ops[1].as_str() {
                    "DAT" => {
                        memory.insert(
                            ops[0].clone(),
                            ops[2].clone().parse::<i64>().unwrap(),
                        );
                    }
                    &_ => {
                        panic!()
                    }
                }
            }
        }
    }

    Ok(())
}
