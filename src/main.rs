use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

fn process(line: String, accumulator: &mut i64, memory: &mut HashMap<String, i64>, labels: &HashMap<String, usize>) -> usize {
    let ops: Vec<String> = line.trim()
        .split_whitespace()
        .map(String::from)
        .collect();

    match ops[0].as_str() {
        "INP" => {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
            *accumulator = s.trim().parse::<i64>().unwrap();
        }
        "OUT" => println!("{}", accumulator),
        "STA" => {
            memory.insert(ops[1].clone(), *accumulator);
        }
        "LDA" => {
            *accumulator = *memory.get(&ops[1]).unwrap_or(&0);
        }
        "ADD" => {
            if let Ok(value) = ops[1].parse::<i64>() {
                *accumulator += value;
            } else {
                *accumulator += *memory.get(&ops[1]).unwrap_or(&0);
            }
        }
        "SUB" => {
            if let Ok(value) = ops[1].parse::<i64>() {
                *accumulator -= value;
            } else {
                *accumulator -= *memory.get(&ops[1]).unwrap_or(&0);
            }
        }
        "BRA" => {
            return *labels.get(&ops[1]).unwrap_or(&0);
        }
        "HLT" => std::process::exit(0),
        _ => {
            if ops.len() > 1 && ops[1] == "DAT" {
                memory.insert(ops[0].clone(), ops.get(2).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0));
            }
        }
    }

    return 0;
}

fn main() -> io::Result<()> {
    let file = File::open("code.asm")?;
    let reader = BufReader::new(file);
    let mut accumulator: i64 = 0;
    let mut memory: HashMap<String, i64> = HashMap::new();
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut code: Vec<String> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let ops: Vec<String> = line.trim().split_whitespace().map(String::from).collect();
        if ops.len() > 1 && ops[1] == "DAT" {
            labels.insert(ops[0].clone(), index);
        } else if ops.len() > 1 {
            labels.insert(ops[0].clone(), index);
        }
        code.push(line);
    }

    let mut pc: usize = 0;
    while pc < code.len() {
        let line = &code[pc];
        let next_pc = process(line.to_string(), &mut accumulator, &mut memory, &labels);
        if next_pc != 0 {
            pc = next_pc;
        } else {
            pc += 1;
        }
    }

    Ok(())
}
