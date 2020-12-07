use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::iter::FromIterator;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: u64,
    cid: Option<u32>,
}

lazy_static! {
    pub static ref FIELDS: HashSet<&'static str> =
        HashSet::from_iter(vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
}

impl Passport {
    fn from_map(mut map: HashMap<String, String>) -> Result<Self, String> {
        if FIELDS.is_subset(&map.keys().map(String::as_str).collect::<HashSet<&str>>()) {
            Ok(Passport {
                byr: map.remove("byr").unwrap().parse().unwrap(),
                iyr: map.remove("iyr").unwrap().parse().unwrap(),
                eyr: map.remove("eyr").unwrap().parse().unwrap(),
                hgt: map.remove("hgt").unwrap(),
                hcl: map.remove("hcl").unwrap(),
                ecl: map.remove("ecl").unwrap(),
                pid: map.remove("pid").unwrap().parse().unwrap(),
                cid: map.remove("cid").map(|s| s.parse().ok()).flatten(),
            })
        } else {
            println!(
                "Not all necessary fields found in map.\nRequired: {:?}\nFound: {:?}",
                FIELDS.iter().join(", "),
                map.keys().format(", ")
            );
            Err(format!(
                "Not all necessary fields found in map.\nRequired: {:?}\nFound: {:?}",
                FIELDS.iter().join(", "),
                map.keys().format(", ")
            ))
        }
    }
}

pub struct BatchFile {
    lines: VecDeque<String>,
}

impl FromIterator<String> for BatchFile {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        let mut bf = BatchFile::new();
        for line in iter {
            bf.put_line(line);
        }
        bf
    }
}

impl FromIterator<io::Result<String>> for BatchFile {
    fn from_iter<T: IntoIterator<Item = io::Result<String>>>(iter: T) -> Self {
        let mut bf = BatchFile::new();
        for line in iter {
            bf.put_line(line.unwrap());
        }
        bf
    }
}

impl BatchFile {
    pub fn new() -> BatchFile {
        BatchFile {
            lines: VecDeque::new(),
        }
    }
    pub fn put_line(&mut self, line: String) {
        self.lines.push_back(line);
    }
}

impl Iterator for BatchFile {
    type Item = Passport;

    fn next(&mut self) -> Option<Passport> {
        let mut map: HashMap<String, String> = HashMap::with_capacity(FIELDS.len());
        loop {
            let line = self.lines.pop_front().unwrap_or("".to_string());
            if line == "" {
                return Passport::from_map(map).ok();
            }
            let fields = line.split(" ");
            map.extend(fields.map(to_entry));
        }
    }
}

fn to_entry(e: &str) -> (String, String) {
    let s: Vec<&str> = e.split(":").collect();
    assert_eq!(s.len(), 2);
    (s.get(0).unwrap().to_string(), s.get(1).unwrap().to_string())
}
