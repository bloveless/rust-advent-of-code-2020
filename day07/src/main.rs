use std::fs;
use regex::Regex;
use std::collections::HashMap;

struct Bag {
    color: String,
    count: i32,
}

fn can_contain(bags: &HashMap<String, Vec<Bag>>, bag: String) -> bool {
    match bags.get(&bag) {
        Some(inner_bags) => {
            for inner_bag in inner_bags {
                if inner_bag.color == "shiny gold" || can_contain(bags, inner_bag.color.clone()) {
                    return true;
                }
            }

            false
        }
        None => false
    }
}

fn get_bag_child_count(bags: &HashMap<String, Vec<Bag>>, bag: String) -> i32 {
    match bags.get(&bag) {
        Some(inner_bags) => {
            let mut child_count = 0;

            for inner_bag in inner_bags {
                let child_bag_count = get_bag_child_count(&bags, inner_bag.color.clone());
                child_count += inner_bag.count + inner_bag.count * child_bag_count;
            }

            child_count
        }
        None => 0
    }
}

fn main() {
    let bag_regex = Regex::new(r"^([0-9]+) ([\w ]*) bags?\.?$").expect("Unable to compile regex");
    let input = fs::read_to_string("input.txt").expect("Unable to read input.txt");
    let lines: Vec<&str> = input.split('\n').collect();

    let mut parent_bags: HashMap<String, Vec<Bag>> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" bags contain ").collect();
        let inner_bags_raw: Vec<&str> = parts[1].split(", ").collect();

        for inner_bag in inner_bags_raw {
            for cap in bag_regex.captures(inner_bag) {
                let parent_bag = parent_bags.entry(parts[0].to_string()).or_insert(Vec::new());
                parent_bag.push(Bag {
                    color: cap[2].to_string(),
                    count: cap[1].parse().expect("Found an invalid bag count"),
                });
            }
        }
    }

    // Part 1 --------------------------------------------------------------------------------------
    let mut bags_that_can_contain_shiny_gold = 0;
    for parent_bag in parent_bags.keys() {
        if parent_bag != "shiny gold" {
            if can_contain(&parent_bags, parent_bag.to_string()) {
                bags_that_can_contain_shiny_gold += 1;
            }
        }
    }

    println!("Part 1 - Bags that can contain shiny gold: {}", bags_that_can_contain_shiny_gold);

    // Part 2 --------------------------------------------------------------------------------------

    let child_count = get_bag_child_count(&parent_bags, "shiny gold".to_string());

    println!("Part 2 - Shiny gold children count: {}", child_count);
}
