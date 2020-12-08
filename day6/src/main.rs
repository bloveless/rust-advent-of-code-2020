use std::fs;
use std::collections::HashMap;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Unable to open input.txt file");
    let groups: Vec<&str> = input.split("\n\n").collect();

    let mut answer_count = 0;
    for group in groups {
        let mut individual_group_chars = HashMap::new();

        let people_count = group.split_whitespace().collect::<Vec<&str>>().len();

        for c in group.chars().into_iter() {
            if c.is_alphabetic() {
                let c_count = individual_group_chars.entry(c).or_insert(0);
                *c_count += 1;
            }
        }

        for (unique_c, c_count) in &individual_group_chars {
            if *c_count == people_count {
                answer_count += 1;
            }
        }
    }

    println!("Answer count: {}", answer_count);
}
