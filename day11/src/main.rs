use std::fs;
use core::fmt;

#[derive(PartialEq, Clone)]
enum Point {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Point::Floor => write!(f, "{}", '.'),
            Point::Empty => write!(f, "{}", 'L'),
            Point::Occupied => write!(f, "{}", '#'),
        }
    }
}

fn check_immediate_neighbor(input: &Vec<Vec<Point>>, row: i32, col: i32) -> i32 {
    let mut neighbor_count = 0;
    for row_offset in -1..2 {
        for col_offset in -1..2 {
            if row_offset == 0 && col_offset == 0 {
                continue;
            }

            let check_row = row + row_offset;
            let check_col = col + col_offset;

            if check_row < 0 || check_row >= (input.len() as i32) {
                continue;
            }

            if check_col < 0 || check_col >= (input[check_row as usize].len() as i32) {
                continue;
            }

            if input[(row + row_offset) as usize][(col + col_offset) as usize] == Point::Occupied {
                neighbor_count += 1;
            }
        }
    }

    neighbor_count
}

fn check_visible_neighbor(input: &Vec<Vec<Point>>, row: i32, col: i32) -> i32 {
    let mut neighbor_count = 0;

    for d_row in -1..=1 {
        for d_col in -1..=1 {
            if d_row == 0 && d_col == 0 {
                continue;
            }

            let mut iteration = 1;
            loop {
                let current_row = row + (d_row * iteration);
                let current_col = col + (d_col * iteration);

                if current_row < 0 || current_col < 0 || current_row >= (input.len() as i32) || current_col >= (input[current_row as usize].len() as i32) {
                    break;
                }

                match input[current_row as usize][current_col as usize] {
                    Point::Occupied => {
                        neighbor_count += 1;
                        break;
                    }
                    Point::Empty => {
                        break;
                    }
                    _ => ()
                };

                iteration += 1;
            }
        }
    }

    neighbor_count
}

fn count_occupied_neighbors(input: &Vec<Vec<Point>>, row: i32, col: i32) -> i32 {
    if false {
        check_immediate_neighbor(input, row, col)
    } else {
        check_visible_neighbor(input, row, col)
    }
}

fn print_seat_map(seats: &Vec<Vec<Point>>) {
    println!("-- Seat Map ----------------------------------");
    for row in 0..seats.len() {
        for col in 0..seats[row].len() {
            print!("{}", seats[row][col]);
        }
        println!();
    }
    println!("----------------------------------------------");
}

fn string_to_seat_map(input: String) -> Vec<Vec<Point>> {
    input
        .trim()
        .split("\n")
        .map(|l|
            l
                .trim()
                .chars()
                .map(|c|
                    match c {
                        'L' => Point::Empty,
                        '.' => Point::Floor,
                        '#' => Point::Occupied,
                        _ => panic!("Input was invalid")
                    }
                )
                .collect()
        ).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let mut seats: Vec<Vec<Point>> = string_to_seat_map(input);

    // Part 1 --------------------------------------------------------------------------------------
    let mut prev_seat_updates = 0;
    loop {
        let mut new_seats = seats.clone();
        let mut changed = false;

        print_seat_map(&new_seats);

        for row in 0..new_seats.len() {
            for col in 0..new_seats[row].len() {
                if new_seats[row][col] != Point::Floor {
                    // Part 1: check_immediate_neighbor
                    // Part 2: count_occupied_neighbors
                    let neighbors = count_occupied_neighbors(&seats, row as i32, col as i32);

                    // if the seat is empty (L) and there are no occupied seats adjacent to it,
                    // the seat becomes occupied

                    if seats[row][col] == Point::Empty && neighbors == 0 {
                        new_seats[row][col] = Point::Occupied;
                        changed = true;
                    }

                    // if the seat is occupied (#) and four or more seats adjacent to it are also occupied,
                    // the seat becomes empty (L)

                    // Part 1: neighbors >= 4
                    // Part 2: neighbors >= 5

                    if seats[row][col] == Point::Occupied && neighbors >= 5 {
                        new_seats[row][col] = Point::Empty;
                        changed = true;
                    }
                }
            }
        }

        if changed {
            seats = new_seats;
        } else {
            println!("Seating map has stabilized");
            break;
        }
    }

    let mut occupied_seats = 0;
    for row in 0..seats.len() {
        for col in 0..seats[row].len() {
            if seats[row][col] == Point::Occupied {
                occupied_seats += 1;
            }
        }
    }

    println!("Occupied Seats: {}", occupied_seats);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let input = "
            .......#.
            ...#.....
            .#.......
            .........
            ..#L....#
            ....#....
            .........
            #........
            ...#.....
        ".to_string();

        let input = string_to_seat_map(input);

        let neighbors = check_visible_neighbor(&input, 4, 3);

        assert_eq!(neighbors, 8);
    }

    #[test]
    fn test_2() {
        let input = "
            .............
            .L.L.#.#.#.#.
            .............
        ".to_string();

        let input = string_to_seat_map(input);

        let neighbors = check_visible_neighbor(&input, 1, 1);

        assert_eq!(neighbors, 0);
    }

    #[test]
    fn test_3() {
        let input = "
            .............
            .L.#.L.#.#.#.
            .............
        ".to_string();

        let input = string_to_seat_map(input);

        let neighbors = check_visible_neighbor(&input, 1, 1);

        assert_eq!(neighbors, 1);
    }

    #[test]
    fn test_4() {
        let input = "
            .##.##.
            #.#.#.#
            ##...##
            ...L...
            ##...##
            #.#.#.#
            .##.##.
        ".to_string();

        let input = string_to_seat_map(input);

        let neighbors = check_visible_neighbor(&input, 3, 3);

        assert_eq!(neighbors, 0);
    }

    #[test]
    fn test_5() {
        let input = "
            ###
            #L#
            ###
        ".to_string();

        let input = string_to_seat_map(input);

        let neighbors = check_visible_neighbor(&input, 1, 1);

        assert_eq!(neighbors, 8);
    }

    #[test]
    fn test_6() {
        let input = "
            #.#.#
            .....
            #.L.#
            .....
            #.#.#
        ".to_string();

        let input = string_to_seat_map(input);

        let neighbors = check_visible_neighbor(&input, 2, 2);

        assert_eq!(neighbors, 8);
    }

    #[test]
    fn test_7() {
        let input = "
            #.#.#
            .LLL.
            #LLL#
            .LLL.
            #.#.#
        ".to_string();

        let input = string_to_seat_map(input);

        let neighbors = check_visible_neighbor(&input, 2, 2);

        assert_eq!(neighbors, 0);
    }

    #[test]
    fn test_8() {
        let input = "
            #..#..#
            .L.L.L.
            .......
            #L.L.L#
            .......
            .L.L.L.
            #..#..#
        ".to_string();

        let input = string_to_seat_map(input);

        let neighbors = check_visible_neighbor(&input, 3, 3);

        assert_eq!(neighbors, 0);
    }
}
