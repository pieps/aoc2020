use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Entry(Rule, Password);
struct Rule {
    start: usize,
    end: usize,
    letter: char,
}
type Password = String;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("data.txt")?;
    let reader = BufReader::new(f);
    let v: Vec<Entry> = reader.lines().flatten().map(process_line).collect();

    let valid: u32 = v.iter().map(Entry::check).sum();

    println!("{}", valid);

    Ok(())
}

fn process_line(line: String) -> Entry {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    }
    //let RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let captures: Captures = RE.captures(&line).unwrap();

    let rule = Rule {
        start: captures.get(1).unwrap().as_str().parse().unwrap(),
        end: captures.get(2).unwrap().as_str().parse().unwrap(),
        letter: captures.get(3).unwrap().as_str().parse().unwrap(),
    };

    let password = captures.get(4).unwrap().as_str();

    Entry(rule, String::from(password))
}

impl Entry {
    fn check(&self) -> u32 {
        let count = self.1.chars().filter(|c| c == &self.0.letter).count();
        match count {
            x if x >= self.0.start && x <= self.0.end => 1,
            _ => 0,
        }
    }
}
