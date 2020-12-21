use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Unable to open input.txt file");
    let lines: Vec<&str> = input.split('\n').collect();
    let width = lines.get(0).unwrap().len();
    let height = lines.len();

    let mut board: Vec<Vec<bool>> = Vec::new();

    for line in &lines {
        let mut row: Vec<bool> = vec![false; width];

        for (index, char) in line.chars().enumerate() {
            row[index] = char == '#';
        }

        board.push(row);
    }

    println!("Size of input. Height: {} Width: {}", lines.len(), lines.get(0).unwrap().len());

    let mut trees_mult: u64 = 0;

    let offsets_to_check = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for current_increments in offsets_to_check.iter() {
        let mut current_x = current_increments.0;
        let mut current_y = current_increments.1;
        let mut trees_hit = 0;

        while current_y < height {
            if board[current_y][current_x % width] {
                trees_hit += 1;
            }

            current_x += current_increments.0;
            current_y += current_increments.1;
        }

        if trees_mult == 0 {
            trees_mult = trees_hit;
        } else {
            trees_mult *= trees_hit;
        }
    }

    println!("{}", trees_mult);
}
