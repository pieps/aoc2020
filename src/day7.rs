use crate::Day;

use std::collections::HashSet;
use std::error::Error;
use std::{cell::Cell, collections::HashMap};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const SAMPLE_2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn day7_1_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE_1);
        let day7 = Day7::new(lines);
        assert_eq!(4, day7.solve1().unwrap());
    }

    #[test]
    fn day7_2_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE_2);
        let day7 = Day7::new(lines);
        assert_eq!(126, day7.solve2().unwrap());
    }
}

pub struct Day7 {
    rules: Rules,
}

impl Day7 {
    pub fn new(lines: Vec<&str>) -> Box<dyn Day> {
        Box::new(Day7 {
            rules: Rules::new(
                lines
                    .iter()
                    .map(|l| parse_rule(*l))
                    .map(|r| (r.id.clone(), r))
                    .collect(),
            ),
        })
    }
}

impl Day for Day7 {
    fn solve1(&self) -> Result<i64, Box<dyn Error>> {
        Ok(self.rules.find_node("shiny gold").len() as i64)
    }

    fn solve2(&self) -> Result<i64, Box<dyn Error>> {
        Ok((self.rules.count_bags("shiny gold") - 1) as i64)
    }
}

lazy_static! {
    static ref TOP_LEVEL_RULE: Regex = Regex::new(r"^(.+) bags contain (.+).$").unwrap();
    static ref SUB_RULE: Regex = Regex::new(r"^(\d+) (.+) bags?").unwrap();
}

#[derive(Debug)]
struct Rule {
    id: String,
    children: HashMap<String, i64>,
    total_count: Cell<Option<i64>>,
}

fn parse_rule(line: &str) -> Rule {
    let caps: Captures = TOP_LEVEL_RULE.captures(line).unwrap();
    let id = caps.get(1).unwrap().as_str().to_owned();
    let children: HashMap<String, i64> = caps
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
struct Rules {
    rules: HashMap<String, Rule>,
}

impl Rules {
    fn new(rules: HashMap<String, Rule>) -> Self {
        Rules { rules }
    }

    fn count_bags(&self, bag: &str) -> i64 {
        let rule = self.rules.get(bag).unwrap();
        if rule.total_count.get().is_some() {
            return rule.total_count.get().unwrap();
        }
        let total_count = rule
            .children
            .iter()
            .map(|(id, count)| count * self.count_bags(id))
            .sum::<i64>()
            + 1;

        self.rules
            .get(bag)
            .unwrap()
            .total_count
            .set(Some(total_count));
        total_count
    }

    fn find_node(&self, needle: &str) -> HashSet<&str> {
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
