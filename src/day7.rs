use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn day7a() -> String {
    let bags = read_data();
    let mybag = "shiny gold";
    bags.values()
        .fold(0usize, |tot, b| {
            println!("{:?}", b);
            if b.can_ultimately_hold(mybag, &bags) {
                tot + 1
            } else {
                tot
            }
        })
        .to_string()
}

pub fn day7b() -> String {
    let bags = read_data();
    let mybag = "shiny gold";
    // let mybag = "dark olive";
    count_bags(bags.get(mybag).unwrap(), &bags).to_string()
}

fn count_bags(bag: &Bag, set: &HashMap<String, Bag>) -> usize {
    if bag.contains.is_empty() {
        println!("{} is empty", bag.color);
        return 0;
    }
    println!(
        "Checking {} inner bags: {:?}",
        bag.contains.len(),
        &bag.contains
    );
    let n = bag.contains.iter().fold(0usize, |tot, (n, b)| {
        let inner_count = count_bags(set.get(b.as_str()).unwrap(), set);
        println!("Counting for {} '{}' inner bags = {}", n, b, inner_count);
        tot + n * (1 + inner_count)
    });
    println!("{} contains {}", bag.color, n);
    n
}

#[derive(Debug)]
struct Bag {
    color: String,
    contains: Vec<(usize, String)>,
}

impl Bag {
    pub fn new(color: &str) -> Self {
        Self {
            color: color.to_string(),
            contains: Vec::new(),
        }
    }

    pub fn can_contain(&mut self, n: usize, bag: &str) {
        self.contains.push((n, bag.to_string()))
    }

    pub fn can_ultimately_hold(&self, color: &str, set: &HashMap<String, Bag>) -> bool {
        if self.contains.is_empty() {
            return false;
        }
        self.contains.iter().any(|(_, b)| {
            b.as_str() == color || set.get(b.as_str()).unwrap().can_ultimately_hold(color, set)
        })
    }
}

fn read_data() -> HashMap<String, Bag> {
    let re = Regex::new(r"^(.*) bags contain (.*)\.$").unwrap();
    let mut set = HashMap::new();
    fs::read_to_string("assets/day7.txt")
        .expect("Could not read file")
        .split('\n')
        .filter_map(|s| to_bag(s, &re))
        .for_each(|b| {
            set.insert(b.color.clone(), b);
        });
    set
}

fn to_bag(s: &str, re: &Regex) -> Option<Bag> {
    let re2 = Regex::new(r"^\s*(\d*) (.*) bags?$").unwrap();
    let caps = re.captures(s)?;
    let color = caps.get(1).unwrap().as_str();
    let mut bag = Bag::new(color);
    let contains = caps.get(2).unwrap().as_str();
    if contains == "no other bags" {
        return Some(bag);
    }
    contains.split(",").for_each(|s| {
        let cap = re2.captures(s).unwrap();
        let n = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let color = cap.get(2).unwrap().as_str();
        bag.can_contain(n, color)
    });
    println!("{:?}", bag);
    Some(bag)
}
