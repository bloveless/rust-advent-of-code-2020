use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let input = File::open("input.txt").expect("Unable to read file");
    let buffered = BufReader::new(input);

    let nums: Vec<u32> = buffered
        .lines()
        .map(|l|
            match l {
                Ok(line) => line.trim().parse().expect("Unable to parse line"),
                Err(_) => panic!("Unable to parse input file"),
            }
        )
        .collect();

    for num in &nums {
        for num2 in &nums {
            for num3 in &nums {
                if num + num2 + num3 == 2020 {
                    println!("Num {} + Num2 {} + Num3 {} == 2020", num, num2, num3);
                    println!("Num {} * Num2 {} * Num3 {} == {}", num, num2, num3, num * num2 * num3);
                    return;
                }
            }
        };
    }
}
