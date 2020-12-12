use std::fs;
use std::collections::HashMap;

fn backtrack(memory: &mut HashMap<usize, i64>, input: &Vec<i32>, current_path: &mut Vec<i32>, start_index: usize) -> i64 {
    // the input is already sorted
    // start at the start index and try every number after it which is within 1 - 3 jolts

    if memory.contains_key(&start_index) {
        return *memory.get(&start_index).unwrap();
    }

    let start_number = input[start_index];
    let mut success_count = 0;
    let mut current_index = start_index + 1;

    while current_index < input.len() && input[current_index] - start_number <= 3 {
        current_path.push(input[current_index]);

        success_count += backtrack(memory, input, current_path, current_index);

        current_index += 1;

        if current_index == input.len() {
            success_count += 1;
        }

        current_path.pop();
    }

    if success_count > 0 {
        memory.insert(start_index, success_count);
    }

    success_count
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open input.txt file");
    let mut input: Vec<i32> = input
        .trim()
        .split("\n")
        .map(|l|
            l.parse().expect("Unable to parse number from line")
        )
        .collect();

    input.sort();

    // Part 1 --------------------------------------------------------------------------------------

    let mut prev_value = 0;
    let mut joltages = HashMap::new();

    // Three starts at one since your device will have a final three jolt jump
    joltages.insert(3, 1);

    for num in &input {
        let diff = num - prev_value;
        let current_value = joltages.entry(diff).or_insert(0);
        *current_value += 1;
        prev_value = *num;
    }   

    for (joltage, count) in &joltages {
        println!("Joltage: {} Count: {}", joltage, count);
    }

    println!("Final jolt 1's * jolt 3's: {}", joltages[&1] * joltages[&3]);

    println!("\n-----------------------------------------------------\n");

    // Part 2 --------------------------------------------------------------------------------------

    let mut memory = HashMap::new();
    let mut total_successes = 0;
    for (i, num)  in input.clone().into_iter().enumerate() { // input.into_iter().enumerate() {
        if num <= 3 {
            // println!("I: {} Num: {}", i, num);
            total_successes += backtrack(&mut memory, &input, &mut vec![0, num], i);
        }
    }

    println!("Total successes: {}", total_successes);
}
