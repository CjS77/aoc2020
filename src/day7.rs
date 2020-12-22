use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn day7a() -> String {
    let bags = read_data();
    let mybag = "shiny gold";
    bags.values()
        .filter(|b| b.can_ultimately_hold(mybag, &bags))
        .count()
        .to_string()
}

pub fn day7b() -> String {
    let bags = read_data();
    let mybag = "shiny gold";
    count_bags(bags.get(mybag).unwrap(), &bags).to_string()
}

fn count_bags(bag: &Bag, set: &HashMap<String, Bag>) -> usize {
    if bag.contains.is_empty() {
        println!("{} is empty", bag.color);
        return 0;
    }
    println!(
        "[{}] has {} inner bags: {:?}",
       bag.color, bag.contains.len(),
        &bag.contains
    );
    let n = bag.contains.iter().fold(0usize, |tot, (n, b)| {
        let inner_count = count_bags(set.get(b.as_str()).unwrap(), set);

        tot + n * (1 + inner_count)
    });
    println!("{} contains {}\n", bag.color, n);
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
            b.as_str() == color ||
                    set.get(b.as_str())
                        .unwrap()
                        .can_ultimately_hold(color, set)
        })
    }
}

fn read_data() -> HashMap<String, Bag> {
    let factory = BagFactory::default();
    let mut set = HashMap::new();
    fs::read_to_string("assets/day7.txt")
        .expect("Could not read file")
        .split('\n')
        .filter_map(|s| factory.from_str(s))
        .for_each(|b| {
            set.insert(b.color.clone(), b);
        });
    set
}

struct BagFactory {
    re: Regex,
    re2: Regex,
}

impl Default for BagFactory {
    fn default() -> Self {
        let re = Regex::new(r"^(.*) bags contain (.*)\.$").unwrap();
        let re2 = Regex::new(r"^\s*(\d*) (.*) bags?$").unwrap();
        Self { re, re2 }
    }
}

impl BagFactory {
    pub fn from_str<T: AsRef<str>>(&self, s: T) -> Option<Bag> {
        let caps = self.re.captures(s.as_ref())?;
        let color = caps.get(1)?.as_str();
        let mut bag = Bag::new(color);
        let contains = caps.get(2)?.as_str();
        if contains == "no other bags" {
            return Some(bag);
        }
        for s in contains.split(',') {
        let cap = self.re2.captures(s)?;
        let n = cap.get(1)?.as_str().parse::<usize>().unwrap();
        let color = cap.get(2)?.as_str();
        bag.can_contain(n, color)
    }
        Some(bag)
    }
}
