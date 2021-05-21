use crate::Day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

    const SAMPLE_2: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn day4_1_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE_1);
        let day4 = Day4::new(lines);
        assert_eq!(2, day4.solve1().unwrap());
    }

    #[test]
    fn day4_2_sample() {
        let lines: Vec<&str> = crate::split_input(SAMPLE_2);
        let day4 = Day4::new(lines);
        assert_eq!(4, day4.solve2().unwrap());
    }

    #[test]
    fn day4_1() {
        let file = std::fs::read_to_string("data/day4.txt").unwrap();
        let lines: Vec<&str> = crate::split_input(&file);
        let day4 = Day4::new(lines);
        assert_eq!(254, day4.solve1().unwrap());
    }

    #[test]
    fn day4_2() {
        let file = std::fs::read_to_string("data/day4.txt").unwrap();
        let lines: Vec<&str> = crate::split_input(&file);
        let day4 = Day4::new(lines);
        assert_eq!(184, day4.solve2().unwrap());
    }
}

pub struct Day4 {
    file: BatchFile,
}

impl Day4 {
    pub fn new(lines: Vec<&str>) -> Box<Self> {
        Box::new(Day4 {
            file: lines.iter().map(|l| l.to_owned()).collect(),
        })
    }
}

impl Day for Day4 {
    fn solve1(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(self.file.iter(validate_1).collect::<Vec<()>>().len() as u64)
    }

    fn solve2(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(self.file.iter(validate_2).collect::<Vec<()>>().len() as u64)
    }
}

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

fn validate_1(map: HashMap<String, String>) -> Option<()> {
    if FIELDS.is_subset(&map.keys().map(String::as_str).collect::<HashSet<&str>>()) {
        Some(())
    } else {
        println!(
            "Not all necessary fields found in map.\nRequired: {:?}\nFound: {:?}",
            FIELDS.iter().join(", "),
            map.keys().join(", ")
        );
        None
    }
}
fn validate_2(map: HashMap<String, String>) -> Option<()> {
    if FIELDS.is_subset(&map.keys().map(String::as_str).collect::<HashSet<&str>>())
        && validate_byr(map.get("byr").map(String::as_str).unwrap())
        && validate_iyr(map.get("iyr").map(String::as_str).unwrap())
        && validate_eyr(map.get("eyr").map(String::as_str).unwrap())
        && validate_hgt(map.get("hgt").map(String::as_str).unwrap())
        && validate_hcl(map.get("hcl").map(String::as_str).unwrap())
        && validate_ecl(map.get("ecl").map(String::as_str).unwrap())
        && validate_pid(map.get("pid").map(String::as_str).unwrap())
    {
        Some(())
    } else {
        None
    }
}

fn extract_u32(s: &str, re: &Regex) -> Option<u32> {
    re.captures(s)
        .and_then(|c| c.get(1))
        .and_then(|y| y.as_str().parse::<u32>().ok())
}

fn year_between(year: &str, start: u32, end: u32) -> bool {
    extract_u32(year, &YEAR)
        .filter(|y| (*y >= start && *y <= end))
        .is_some()
}

fn validate_byr(byr: &str) -> bool {
    year_between(byr, 1920, 2002)
}

fn validate_iyr(iyr: &str) -> bool {
    year_between(iyr, 2010, 2020)
}

fn validate_eyr(eyr: &str) -> bool {
    year_between(eyr, 2020, 2030)
}

fn hgt_between(hgt: &str, re: &Regex, start: u32, end: u32) -> bool {
    extract_u32(hgt, re)
        .filter(|h| *h >= start && *h <= end)
        .is_some()
}

fn validate_hgt(hgt: &str) -> bool {
    hgt_between(hgt, &HGT_CM, 150, 193) || hgt_between(hgt, &HGT_IN, 59, 76)
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

pub struct BatchFile {
    lines: VecDeque<String>,
}

impl<'a> FromIterator<&'a str> for BatchFile {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> BatchFile {
        let mut bf = BatchFile::new();
        for line in iter {
            bf.put_line(line);
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
    pub fn put_line(&mut self, line: &str) {
        println!("line: {}", line);
        self.lines.push_back(line.to_string());
    }

    pub fn iter<F>(&self, validator: F) -> BatchFileIter<F>
    where
        F: Fn(HashMap<String, String>) -> Option<()>,
    {
        BatchFileIter::new(self, validator)
    }
}

pub struct BatchFileIter<'a, F: Fn(HashMap<String, String>) -> Option<()>> {
    lines: Vec<&'a String>,
    validator: F,
}

impl<'a, F> BatchFileIter<'a, F>
where
    F: Fn(HashMap<String, String>) -> Option<()>,
{
    fn new(bf: &'a BatchFile, validator: F) -> BatchFileIter<'a, F> {
        BatchFileIter {
            lines: bf.lines.iter().rev().collect(),
            validator,
        }
    }
}

impl<'a, F> Iterator for BatchFileIter<'a, F>
where
    F: Fn(HashMap<String, String>) -> Option<()>,
{
    type Item = ();

    fn next(&mut self) -> Option<()> {
        let mut map: HashMap<String, String> = HashMap::with_capacity(FIELDS.len());
        loop {
            let line = self.lines.pop();
            if line.is_none() {
                return (self.validator)(map);
            }

            let line = line.unwrap();
            if line == "" {
                let p = (self.validator)(map);
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
