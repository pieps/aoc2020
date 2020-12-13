use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_group_sums_unique_chars() {
        let group: Vec<String> = vec!["abc", "a", "a", "z"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(process_group(&group), 4);
    }
}

pub fn partition<I>(mut lines: I) -> Vec<Vec<String>>
where
    I: Iterator<Item = String>,
{
    let mut v: Vec<Vec<String>> = Vec::new();
    let mut group: Vec<String> = Vec::new();
    while let Some(line) = lines.next() {
        if line == "" {
            v.push(group);
            group = Vec::new();
        } else {
            group.push(line);
        }
    }

    v.push(group);
    v
}

pub fn process_group(lines: &Vec<String>) -> usize {
    let set: HashSet<char> = lines.iter().flat_map(|l| l.chars()).collect();
    set.len()
}
