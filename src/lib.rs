use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
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

pub struct Passport {}

lazy_static! {
    pub static ref FIELDS: HashSet<&'static str> =
        HashSet::from_iter(vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    pub static ref YEAR: Regex = Regex::new(r"^(\d{4})$").unwrap();
    pub static ref HGT_CM: Regex = Regex::new(r"^(\d{3})cm$").unwrap();
    pub static ref HGT_IN: Regex = Regex::new(r"^(\d{2})in$").unwrap();
    pub static ref HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    pub static ref ECL_SET: HashSet<&'static str> =
        HashSet::from_iter(vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);
    pub static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
}

impl Passport {
    fn from_map(map: HashMap<String, String>) -> Option<()> {
        if FIELDS.is_subset(&map.keys().map(String::as_str).collect::<HashSet<&str>>()) {
            if Passport::validate_byr(map.get("byr").map(String::as_str).unwrap()) {
                print!("byr ");
            } else {
                println!("");
                return None;
            }
            if Passport::validate_iyr(map.get("iyr").map(String::as_str).unwrap()) {
                print!("iyr ");
            } else {
                println!("");
                return None;
            }
            if Passport::validate_eyr(map.get("eyr").map(String::as_str).unwrap()) {
                print!("eyr ");
            } else {
                println!("");
                return None;
            }
            if Passport::validate_hgt(map.get("hgt").map(String::as_str).unwrap()) {
                print!("hgt ");
            } else {
                println!("");
                return None;
            }
            if Passport::validate_hcl(map.get("hcl").map(String::as_str).unwrap()) {
                print!("hcl ");
            } else {
                println!("");
                return None;
            }
            if Passport::validate_ecl(map.get("ecl").map(String::as_str).unwrap()) {
                print!("ecl ");
            } else {
                println!("");
                return None;
            }
            if Passport::validate_pid(map.get("pid").map(String::as_str).unwrap()) {
                print!("pid\n");
            } else {
                println!("");
                return None;
            }
            Some(())
        } else {
            println!("Found: {:?}", map.keys().format(", "));
            None
        }
    }

    fn extract_u32(s: &str, re: &Regex) -> Option<u32> {
        re.captures(s)
            .and_then(|c| c.get(1))
            .and_then(|y| y.as_str().parse::<u32>().ok())
    }

    fn year_between(year: &str, start: u32, end: u32) -> bool {
        Passport::extract_u32(year, &YEAR)
            .filter(|y| (*y >= start && *y <= end))
            .is_some()
    }

    fn validate_byr(byr: &str) -> bool {
        Passport::year_between(byr, 1920, 2002)
    }

    fn validate_iyr(iyr: &str) -> bool {
        Passport::year_between(iyr, 2010, 2020)
    }

    fn validate_eyr(eyr: &str) -> bool {
        Passport::year_between(eyr, 2020, 2030)
    }

    fn hgt_between(hgt: &str, re: &Regex, start: u32, end: u32) -> bool {
        Passport::extract_u32(hgt, re)
            .filter(|h| *h >= start && *h <= end)
            .is_some()
    }

    fn validate_hgt(hgt: &str) -> bool {
        Passport::hgt_between(hgt, &HGT_CM, 150, 193) || Passport::hgt_between(hgt, &HGT_IN, 59, 76)
    }

    fn validate_hcl(hcl: &str) -> bool {
        HCL.is_match(hcl)
    }

    fn validate_ecl(ecl: &str) -> bool {
        ECL_SET.contains(ecl)
    }

    fn validate_pid(pid: &str) -> bool {
        PID.is_match(pid)
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
        iter.into_iter().map(Result::unwrap).collect()
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
    type Item = ();

    fn next(&mut self) -> Option<()> {
        let mut map: HashMap<String, String> = HashMap::with_capacity(FIELDS.len());
        loop {
            let line = self.lines.pop_front();
            if line.is_none() {
                return Passport::from_map(map);
            }

            let line = line.unwrap();
            if line == "".to_string() {
                let p = Passport::from_map(map);
                if p.is_some() {
                    return p;
                }
                map = HashMap::with_capacity(FIELDS.len());
            } else {
                let fields = line.split(" ");
                map.extend(fields.map(to_entry));
            }
        }
    }
}

fn to_entry(e: &str) -> (String, String) {
    let s: Vec<&str> = e.split(":").collect();
    assert_eq!(s.len(), 2);
    (s.get(0).unwrap().to_string(), s.get(1).unwrap().to_string())
}
