use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

fn main() {
    let input = File::open("input.txt").expect("Unable to read file");
    let buffered = BufReader::new(input);
    let re = Regex::new(r"(\d*)-(\d*) ([a-z]): ([a-z]*)").unwrap();

    let mut password_matches = 0;

    for line in buffered.lines() {
        match line {
            Ok(line) => {
                for cap in re.captures_iter(&line) {

                    let low: usize = cap[1].parse().unwrap();
                    let high: usize = cap[2].parse().unwrap();
                    let search_char: char = cap[3].chars().nth(0).unwrap();
                    let haystack: &str = &cap[4];

                    let low_match = haystack.chars().nth(low - 1).unwrap() == search_char;
                    let high_match = haystack.chars().nth(high - 1).unwrap() == search_char;

                    let matched = low_match ^ high_match;

                    if matched {
                        password_matches += 1;
                    }

                    println!("Line: {}\nLow: {} High: {} Char: {} Pass: {} Matched: {}", &cap[0], &cap[1], &cap[2], &cap[3], &cap[4], matched);
                }
            },
            Err(_) => panic!("Unable to parse input file"),
        }
    }

    println!("Password Matches: {}", password_matches);

    // let output: Vec<std::string::String> = buffered
    //     .lines()
    //     .map(|l|
    //         match l {
    //             Ok(line) => {
    //                 for cap in re.captures_iter(line.trim()) {
    //                     format!("Low: {} High: {} Char: {} Pass: {}", &cap[0], &cap[1], &cap[2], &cap[3])
    //                 }
    //             },
    //             Err(_) => panic!("Unable to parse input file"),
    //         }
    //     )
    //     .collect();
}
