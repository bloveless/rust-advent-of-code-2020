#[macro_use]
extern crate lazy_static;

use std::fs;
use regex::Regex;

lazy_static! {
    static ref HEIGHT_REGEX: Regex = Regex::new(r"^([0-9]*)(in|cm)$").unwrap();
    static ref HAIR_COLOR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PASSPORT_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

struct Identification<'a> {
    birth_year: Option<i32>,
    issue_year: Option<i32>,
    expiration_year: Option<i32>,
    height: Option<i32>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<&'a str>,
}

impl<'a> Identification<'a> {
    pub fn new() -> Identification<'a> {
        Identification{
            birth_year: None,
            issue_year: None,
            expiration_year: None,
            height: None,
            hair_color: None,
            eye_color: None,
            passport_id: None,
            country_id: None
        }
    }

    fn is_valid(&self) -> bool {
        self.birth_year != None &&
            self.issue_year != None &&
            self.expiration_year != None &&
            self.height != None &&
            self.hair_color != None &&
            self.eye_color != None &&
            self.passport_id != None
    }

    fn set_birth_year(&mut self, year: &str) {
        let year: i32 = match year.parse() {
            Ok(year) => year,
            Err(_) => 0,
        };

        if year >= 1920 && year <= 2002 {
            self.birth_year = Some(year);
        }
    }

    fn set_issue_year(&mut self, year: &str) {
        let year: i32 = match year.parse() {
            Ok(year) => year,
            Err(_) => 0,
        };

        if year >= 2010 && year <= 2020 {
            self.issue_year = Some(year);
        }
    }

    fn set_expiration_year(&mut self, year: &str) {
        let year: i32 = match year.parse() {
            Ok(year) => year,
            Err(_) => 0,
        };

        if year >= 2020 && year <= 2030 {
            self.expiration_year = Some(year);
        }
    }

    fn set_height(&mut self, height: &str) {
        for cap in HEIGHT_REGEX.captures_iter(height) {
            let height: i32 = cap[1].parse().unwrap();
            if &cap[2] == "cm" && height >= 150 && height <= 193 {
                self.height = Some(height);
            }

            if &cap[2] == "in" && height >= 59 && height <= 76 {
                self.height = Some(height);
            }
        }
    }

    fn set_hair_color(&mut self, hair_color: &'a str) {
        if HAIR_COLOR_REGEX.is_match(hair_color) {
            self.hair_color = Some(hair_color);
        }
    }

    fn set_eye_color(&mut self, eye_color: &'a str) {
        if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&eye_color) {
            self.eye_color = Some(eye_color);
        }
    }

    fn set_passport_id(&mut self, passport_id: &'a str) {
        if PASSPORT_REGEX.is_match(passport_id) {
            self.passport_id = Some(passport_id);
        }
    }

    fn set_country_id(&mut self, country_id: &'a str) {
        self.country_id = Some(country_id);
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Unable to open input.txt file");
    let lines: Vec<&str> = input.split("\n\n").collect();
    let mut ids: Vec<Identification> = Vec::new();

    for input in lines {
        let mut id = Identification::new();

        for group in input.split_whitespace() {
            let key_value: Vec<&str> = group.split(':').collect();

            match key_value[0] {
                "hcl" => id.set_hair_color(key_value[1]),
                "ecl" => id.set_eye_color(key_value[1]),
                "pid" => id.set_passport_id(key_value[1]),
                "cid" => id.set_country_id(key_value[1]),
                "eyr" => id.set_expiration_year(key_value[1]),
                "hgt" => id.set_height(key_value[1]),
                "byr" => id.set_birth_year(key_value[1]),
                "iyr" => id.set_issue_year(key_value[1]),
                _ => panic!("Unknown key found"),
            };
        }

        ids.push(id);
    }

    let mut valid_ids = 0;
    for id in &ids {
        if id.is_valid() {
            valid_ids += 1;
        }
    }

    println!("Valid IDs: {} Total IDs: {}", valid_ids, ids.len());
}