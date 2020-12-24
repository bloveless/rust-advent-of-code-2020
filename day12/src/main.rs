use std::fs;

const NORTH: i32 = 0;
const EAST: i32 = 90;
const SOUTH: i32 = 180;
const WEST: i32 = 270;
const DEBUG: bool = true;

#[derive(PartialOrd, PartialEq)]
enum Direction {
    Clockwise,
    CounterClockwise
}

fn determine_final_position(input: Vec<(&str, i32)>) -> (i32, i32) {
    let mut ship_x = 0;
    let mut ship_y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;

    for (direction, distance) in input {
        if DEBUG {
            println!("Current ship pos ({}, {}) waypoint pos ({}, {})", ship_x, ship_y, waypoint_x, waypoint_y);
            println!("  - Moving Direction - {} Distance - {}", direction, distance);
        }

        match direction {
            "N" => {
                waypoint_y += distance;
            },
            "S" => {
                waypoint_y -= distance;
            },
            "E" => {
                waypoint_x += distance;
            },
            "W" => {
                waypoint_x -= distance;
            },
            "L" => {
                let (new_x, new_y) = rotate_around_point(ship_x, ship_y, waypoint_x, waypoint_y, Direction::CounterClockwise, distance);
                waypoint_x = new_x;
                waypoint_y = new_y;
            },
            "R" => {
                let (new_x, new_y) = rotate_around_point(ship_x, ship_y, waypoint_x, waypoint_y, Direction::Clockwise, distance);
                waypoint_x = new_x;
                waypoint_y = new_y;
            },
            "F" => {
                let x_diff = waypoint_x - ship_x;
                let y_diff = waypoint_y - ship_y;

                ship_x = (x_diff * distance) + ship_x;
                ship_y = (y_diff * distance) + ship_y;

                waypoint_x = (x_diff * distance) + waypoint_x;
                waypoint_y = (y_diff * distance) + waypoint_y;
            },
            _ => panic!("Invalid input"),
        }
    }

    (ship_x, ship_y)
}

fn rotate_around_point(
    x_origin: i32,
    y_origin: i32,
    x_point: i32,
    y_point: i32,
    direction: Direction,
    degrees: i32
) -> (i32, i32) {
    let mut x_point_adjusted = x_point - x_origin;
    let mut y_point_adjusted = y_point - y_origin;
    let adjusted_degrees = if direction == Direction::CounterClockwise {
        360 - degrees
    } else {
        degrees
    };

    match adjusted_degrees {
        90 => {
            // (x, y) = (y, -x)
            let t = x_point_adjusted;
            x_point_adjusted = y_point_adjusted;
            y_point_adjusted = t * -1;
        }
        180 => {
            // (x, y) = (-x, -y)
            x_point_adjusted = x_point_adjusted * -1;
            y_point_adjusted = y_point_adjusted * -1;
        }
        270 => {
            // (x, y) = (-y, x)
            let t = x_point_adjusted;
            x_point_adjusted = y_point_adjusted * -1;
            y_point_adjusted = t;
        }
        _ => panic!("Invalid degrees")
    }

    (x_point_adjusted + x_origin, y_point_adjusted + y_origin)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input file");
    let input: Vec<(&str, i32)> = input
        .trim()
        .split("\n")
        .map(|l| {
            l
                .trim()
                .split_at(1)
        })
        .map(|(direction, length)| {
            (direction, length.parse().expect("Unable to process input"))
        })
        .collect();

    let (x_pos, y_pos) = determine_final_position(input);

    println!("X Pos: {}, Y Pos: {}", x_pos, y_pos);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotation_clockwise() {
        let (x, y) = rotate_around_point(-1, 1, 4, 5, Direction::Clockwise, 90);
        assert_eq!(x, 3);
        assert_eq!(y, -4);

        let (x, y) = rotate_around_point(-1, 1, 4, 5, Direction::Clockwise, 180);
        assert_eq!(x, -6);
        assert_eq!(y, -3);

        let (x, y) = rotate_around_point(-1, 1, 4, 5, Direction::Clockwise, 270);
        assert_eq!(x, -5);
        assert_eq!(y, 6);
    }

    #[test]
    fn test_rotation_counterclockwise() {
        let (x, y) = rotate_around_point(-1, 1, 4, 5, Direction::CounterClockwise, 90);
        assert_eq!(x, -5);
        assert_eq!(y, 6);

        let (x, y) = rotate_around_point(-1, 1, 4, 5, Direction::CounterClockwise, 180);
        assert_eq!(x, -6);
        assert_eq!(y, -3);

        let (x, y) = rotate_around_point(-1, 1, 4, 5, Direction::CounterClockwise, 270);
        assert_eq!(x, 3);
        assert_eq!(y, -4);
    }
}
