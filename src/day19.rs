use crate::bits::read_data;
use regex::Regex;
use std::collections::HashMap;

pub fn day19a() -> String {
    solve_set("assets/day19.txt")
}

pub fn day19b() -> String {
    solve_set("assets/day19b.txt")
}

fn solve_set(filename: &str) -> String {
    let data = read_data(filename);
    let mut rule_set = RuleSet::new(&data[0..129]);
    let messages = &data[130..];

    println!("{:?}", rule_set);

    messages.iter()
        .filter(|m| {
            let ok = rule_set.matches(m.as_str(), 0);
            println!("Checking [{}].. {}", m, ok);
            ok
        })
        .count()
        .to_string()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum ValidationRule {
    Literal(String),
    Or(Vec<ValidationRule>),
    List(Vec<usize>),
}

#[derive(Clone, Debug)]
struct RuleSet {
    rules: Vec<ValidationRule>,
    cache: HashMap<(String, ValidationRule), bool>,
}

impl RuleSet {
    pub fn new(s: &[String]) -> Self {
        let literal_re = Regex::new("\"(.+)\"").unwrap();
        let mut rules = vec![ValidationRule::Literal("x".to_string()); s.len()];
        for rule_str in s {
            let (rule_index, rule) = match rule_str.find(':') {
                Some(i) => (rule_str[0..i].trim().parse::<usize>().unwrap(), &rule_str[i + 2..]),
                _ => { continue; }
            };
            if let Some(cap) = literal_re.captures(rule) {
                let literal = cap.get(1).unwrap().as_str().to_string();
                set_rule(rule_index, ValidationRule::Literal(literal), &mut rules);
                continue;
            }
            let mut sub_rules = rule.split('|')
                .filter_map(str_to_list)
                .collect::<Vec<ValidationRule>>();
            if sub_rules.len() == 1 {
                set_rule(rule_index, sub_rules.remove(0), &mut rules);
            } else {
                set_rule(rule_index, ValidationRule::Or(sub_rules), &mut rules);
            }
        }
        Self { rules, cache: HashMap::new() }
    }

    pub fn matches(&mut self, msg: &str, rule_no: usize) -> bool {
        let rule = match self.rules.get(rule_no) {
            Some(rule) => rule.clone(), // Must clone to avoid double borrow
            _ => return false,
        };
        self.validates(msg, &rule)
    }

    fn validates(&mut self, msg: &str, rule: &ValidationRule) -> bool {
        // println!("Does {} match {:?}...", msg, rule);
        let lookup = (msg.to_string(), rule.clone());
        if let Some(result) = self.cache.get(&lookup) {
            // println!("Does {} match {:?} ? {} (cached)", msg, rule, *result);
            return *result;
        }
        let res = match rule {
            ValidationRule::Literal(s) => s.as_str() == msg,
            ValidationRule::List(rules) => self.match_list(msg, &rules),
            ValidationRule::Or(rules) => {
                rules.iter().any(|r| self.validates(msg, r))
            }
        };
        // println!("Does {} match {:?} ? {}", msg, rule, res);
        self.cache.insert(lookup, res);
        res
    }

    fn match_list(&mut self, msg: &str, list: &[usize]) -> bool {
        // An empty list matches an empty message
        if list.is_empty() {
            // println!("List is empty. {} matches? {}", msg, msg.is_empty());
            return msg.is_empty();
        }
        // match the first rule, then match the rest of the message against the other rules
        let first = self.rules[list[0]].clone();
        if list.len() == 1 {
            return self.validates(msg, &first);
        }
        // Increase message length until we hit a match
        (1..msg.len()).any(|len| {
            let (sub_msg, rest) = msg.split_at(len);
            // println!("Does {} match {:?} AND {} match {:?}", sub_msg, first, rest, &list[1..]);
            self.validates(sub_msg, &first) && self.match_list(rest, &list[1..])
        })
    }
}

fn set_rule(i: usize, rule: ValidationRule, rules: &mut Vec<ValidationRule>) {
    match rules.get_mut(i) {
        Some(el) => *el = rule,
        None => {
            println!("Didn't expect to hit this. {}, {}", i, rules.len());
            let mut padding = vec![ValidationRule::Literal("x".to_string()); i - rules.len() + 1];
            rules.append(&mut padding);
            set_rule(i, rule, rules)
        }
    }
}

fn str_to_list(s: &str) -> Option<ValidationRule> {
    let rule_numbers = s.trim()
        .split(' ')
        .map(|rule_no| rule_no.parse::<usize>().ok())
        .collect::<Option<Vec<usize>>>()?;
    Some(ValidationRule::List(rule_numbers))
}