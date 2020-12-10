use std::fs;

fn process_encryption(lines: &Vec<i64>, preamble_length: usize) {
    let mut current_index = preamble_length;

    'outer: while current_index < lines.len() {
        let mut found_sum = false;
        let start_index = current_index - preamble_length;

        'inner: for i in start_index..current_index{
            for j in start_index..current_index{
                if i != j && lines[i] + lines[j] == lines[current_index] {
                    found_sum = true;
                    break 'inner;
                }
            }
        }

        if !found_sum {
            println!("[{}], {} had no sum in the last {} numbers", current_index, lines[current_index], preamble_length);

            for i in 0..lines.len() {
                let mut sum = lines[i];
                let mut smallest_number = lines[i];
                let mut largest_number = lines[i];

                for j in (i + 1)..lines.len() {
                    sum += lines[j];

                    if lines[j] > largest_number {
                        largest_number = lines[j];
                    }

                    if lines[j] < smallest_number {
                        smallest_number = lines[j];
                    }

                    if sum == lines[current_index] {
                        println!("Found contiguous sum between [{}] {} and [{}] {} - sum of largest and smallest: {}", i, lines[i], j, lines[j], largest_number + smallest_number);
                        break 'outer;
                    }
                }
            }

            break;
        }

        current_index += 1;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input.txt file");
    let lines: Vec<i64> = input
        .trim()
        .split("\n")
        .map(|l| l.parse().expect("Unable to convert line to number"))
        .collect();

    process_encryption(&lines, 25);
}
