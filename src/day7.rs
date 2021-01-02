use std::collections::HashSet;
use std::{cell::Cell, collections::HashMap};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

lazy_static! {
    pub static ref TOP_LEVEL_RULE: Regex = Regex::new(r"^(.+) bags contain (.+).$").unwrap();
    pub static ref SUB_RULE: Regex = Regex::new(r"^(\d+) (.+) bags?").unwrap();
}

#[derive(Debug)]
pub struct Rule {
    pub id: String,
    children: HashMap<String, u32>,
    total_count: Cell<Option<u32>>,
}

pub fn parse_rule(line: String) -> Rule {
    let caps: Captures = TOP_LEVEL_RULE.captures(&line).unwrap();
    let id = caps.get(1).unwrap().as_str().to_owned();
    let children: HashMap<String, u32> = caps
        .get(2)
        .unwrap()
        .as_str()
        .split(", ")
        .flat_map(|s| SUB_RULE.captures(s))
        .map(|c| {
            (
                c.get(2).unwrap().as_str().to_owned(),
                c.get(1).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();
    Rule {
        id,
        children,
        total_count: Cell::new(Option::None),
    }
}

#[derive(Debug)]
pub struct Rules {
    rules: HashMap<String, Rule>,
}

impl Rules {
    pub fn new(rules: HashMap<String, Rule>) -> Self {
        Rules { rules }
    }

    pub fn count_bags(&self, bag: &str) -> u32 {
        let rule = self.rules.get(bag).unwrap();
        if rule.total_count.get().is_some() {
            return rule.total_count.get().unwrap();
        }
        let total_count = rule
            .children
            .iter()
            .map(|(id, count)| count * self.count_bags(id))
            .sum::<u32>()
            + 1;

        self.rules
            .get(bag)
            .unwrap()
            .total_count
            .set(Some(total_count));
        total_count
    }

    pub fn find_node(&self, needle: &str) -> HashSet<&str> {
        self.rules
            .keys()
            .filter(|k| self.find_node_internal(needle, self.rules.get(*k).unwrap()))
            .map(String::as_ref)
            .collect()
    }

    fn find_node_internal<'a>(&'a self, needle: &str, rule: &'a Rule) -> bool {
        if rule.children.keys().any(|k| k == needle) {
            return true;
        }

        rule.children
            .keys()
            .any(|k| self.find_node_internal(needle, self.rules.get(k).unwrap()))
    }
}
