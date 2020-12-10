use std::fs;
use regex::Regex;
use std::collections::HashMap;

struct Bag {
    color: String,
    count: i32,
}

fn can_contain(bags: &HashMap<String, Vec<String>>, bag: String) -> bool {
    match bags.get(&bag) {
        Some(inner_bags) => {
            for inner_bag in inner_bags {
                if inner_bag == "shiny gold" || can_contain(bags, inner_bag.to_string()) {
                    return true;
                }
            }

            false
        }
        None => false
    }
}

fn main() {
    let bag_regex = Regex::new(r"^([0-9]+) ([\w ]*) bags?\.?$").expect("Unable to compile regex");
    let input = fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let lines: Vec<&str> = input.split('\n').collect();

    let mut parent_bags: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" bags contain ").collect();
        let inner_bags_raw: Vec<&str> = parts[1].split(", ").collect();

        for inner_bag in inner_bags_raw {
            for cap in bag_regex.captures(inner_bag) {
                let parent_bag = parent_bags.entry(parts[0].to_string()).or_insert(Vec::new());
                parent_bag.push(cap[2].to_string());
            }
        }
    }

    let mut bags_that_can_contain_shiny_gold = 0;
    for parent_bag in parent_bags.keys() {
        if parent_bag != "shiny gold" {
            if can_contain(&parent_bags, parent_bag.to_string()) {
                bags_that_can_contain_shiny_gold += 1;
            }
        }
    }

    println!("Bags that can contain shiny gold: {}", bags_that_can_contain_shiny_gold);
}
