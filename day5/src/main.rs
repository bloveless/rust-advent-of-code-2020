use std::fs;
use std::collections::BTreeSet;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Unable to open input.txt file");
    let lines: Vec<&str> = input.split('\n').collect();
    let mut seats = BTreeSet::new();

    for line in lines {
        let mut row_high = 127.0;
        let mut row_low = 0.0;
        let mut col_high = 7.0;
        let mut col_low = 0.0;

        for c in line.chars() {
            match c {
                'F' => row_high = (((row_high + row_low) / 2.0) as f64).floor(),
                'B' => row_low = ((row_low + ((row_high - row_low) / 2.0)) as f64).ceil(),
                'L' => col_high = (((col_high + col_low) / 2.0) as f64).floor(),
                'R' => col_low = ((col_low + ((col_high - col_low) / 2.0)) as f64).ceil(),
                _ => panic!("Invalid code"),
            }
        }

        if row_low != row_high && col_low != col_high {
            panic!("Ambiguous line detected: {}", line);
        } else {
            let seat_id = (row_low * 8.0) + col_low;

            seats.insert(seat_id as i32);
        }
    }

    let mut previous_seat_id = -1;

    for current_seat in seats {
        if previous_seat_id == -1 {
            previous_seat_id = current_seat;
            continue;
        }

        if current_seat - previous_seat_id > 1 {
            println!("Your seat is {}", current_seat - 1);
        }

        previous_seat_id = current_seat;
    }

    println!("Highest seat ID: {}", previous_seat_id);
}
