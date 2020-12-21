use std::fs;
use std::collections::HashSet;

#[derive(Clone)]
struct Code<'a> {
    instruction: &'a str,
    argument: i32,
}

fn process_codes(codes: &Vec<Code>) -> (i32, bool) {
    let mut accumulator = 0;
    let mut instruction_index = 0;
    let mut visited_lines: HashSet<i32> = HashSet::new();
    let mut found_loop = false;

    loop {
        if visited_lines.contains(&(instruction_index as i32)) {
            found_loop = true;
            break;
        } else {
            visited_lines.insert(instruction_index as i32);
        }

        if instruction_index == codes.len() {
            break;
        }

        match codes[instruction_index].instruction {
            "acc" => {
                accumulator += codes[instruction_index].argument;
                instruction_index += 1;
            }
            "jmp" => {
                if codes[instruction_index].argument >= 0 {
                    instruction_index += codes[instruction_index].argument as usize;
                } else {
                    instruction_index -= codes[instruction_index].argument.abs() as usize;
                }
            }
            "nop" => {
                instruction_index += 1;
            }
            _ => {}
        }
    }

    return (accumulator, found_loop);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input file");
    let mut codes: Vec<Code> = input.trim().split("\n").map(|l| {
        let parts: Vec<&str> = l.split(' ').collect();

        Code {
            instruction: parts[0].clone(),
            argument: parts[1].parse().expect("Unable to parse argument"),
        }
    }).collect();


    // Part 1 --------------------------------------------------------------------------------------

    let (accumulator, _) = process_codes(&codes);

    println!("Accumulator: {}", accumulator);

    // Part 2 --------------------------------------------------------------------------------------

    for (i, _) in codes.iter().enumerate() {
        let mut codes_copy = codes.clone();

        if codes_copy[i].instruction == "nop" {
            codes_copy[i].instruction = "jmp";
        }

        if codes_copy[i].instruction == "jmp" {
            codes_copy[i].instruction = "nop";
        }

        // check for loops
        let (accumulator, found_loop) = process_codes(&codes_copy);

        if !found_loop {
            println!("Fixed code at index {} - accumulator is now {}", i, accumulator);
        }
    }
}
