use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn retrieve(memory: &HashMap<String, i64>, ops: &[String]) -> i64 {
    if ops.len() >= 3 {
        *memory.get(&ops[2]).unwrap_or(&0)
    } else {
        *memory.get(&ops[1]).unwrap_or(&0)
    }
}

fn branch(labels: &HashMap<String, usize>, ops: &[String]) -> usize {
    if ops.len() >= 3 {
        *labels.get(&ops[2]).unwrap_or(&0)
    } else {
        *labels.get(&ops[1]).unwrap_or(&0)
    }
}

fn process(
    line: &str,
    accumulator: &mut i64,
    memory: &mut HashMap<String, i64>,
    labels: &HashMap<String, usize>,
    pc: usize,
) -> usize {
    let ops: Vec<String> = line.split_whitespace().map(String::from).collect();

    match ops.get(0).map(String::as_str) {
        Some("INP") => {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
            *accumulator = s.trim().parse::<i64>().unwrap();
        }
        Some("OUT") => println!("{}", accumulator),
        Some("STA") => {
            let key = if ops.len() >= 3 { &ops[2] } else { &ops[1] };
            memory.insert(key.clone(), *accumulator);
        }
        Some("LDA") => {
            *accumulator = retrieve(memory, &ops);
        }
        Some("ADD") => {
            if let Ok(value) = ops[1].parse::<i64>() {
                *accumulator += value;
            } else {
                *accumulator += retrieve(memory, &ops);
            }
        }
        Some("SUB") => {
            if let Ok(value) = ops[1].parse::<i64>() {
                *accumulator -= value;
            } else {
                *accumulator -= retrieve(memory, &ops);
            }
        }
        Some("BRA") => return branch(labels, &ops),
        Some("BRP") => {
            if *accumulator >= 0 {
                return branch(labels, &ops);
            }
        }
        Some("BRZ") => {
            if *accumulator == 0 {
                return branch(labels, &ops);
            }
        }
        Some("HLT") => std::process::exit(0),
        _ => {
            if ops.len() > 1 && ops[1] == "DAT" {
                memory.insert(
                    ops[0].clone(),
                    ops.get(2).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
                );
            }
        }
    }

    pc + 1
}

fn main() -> io::Result<()> {
    let file = File::open("code.asm")?;
    let reader = BufReader::new(file);

    let mut accumulator: i64 = 0;
    let mut memory: HashMap<String, i64> = HashMap::new();
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut code: Vec<String> = Vec::new();

    let valid: [&str; 10] = ["BRA", "BRZ", "BRP", "OUT", "STA", "SUB", "LDA", "ADD", "INP", "HLT"];

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let ops: Vec<String> = line.split_whitespace().map(String::from).collect();
        if ops.len() > 1 && ops[1] == "DAT" {
            labels.insert(ops[0].clone(), ops[2].parse::<usize>().unwrap_or(0));
        } else if ops.len() > 1 && !valid.contains(&&*ops[0]) {
            labels.insert(ops[0].clone(), index);
        }
        code.push(line);
    }

    let mut pc: usize = 1;
    while pc < code.len() {
        let line = &code[pc];
        pc = process(line, &mut accumulator, &mut memory, &labels, pc);
    }

    Ok(())
}
