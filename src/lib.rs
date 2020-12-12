#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pass_converts() {
        assert_eq!(parse_pass(String::from("FBFBBFFRLR")), 357);
    }

    #[test]
    fn find_xor_between_min_less_than_max() {
        let max: u32 = 0b11111001;
        let min: u32 = 0b00000101;
        let range = min..(max + 1);

        assert_eq!(find_xor_between(min, max), range.fold(0, |a, b| a ^ b));
    }

    #[test]
    fn find_xor_between_backwards() {
        let max: u32 = 0b11111001;
        let min: u32 = 0b00000101;

        assert_eq!(find_xor_between(max, min), find_xor_between(min, max));
    }

    #[test]
    fn find_xor_between_empty_range() {
        let n: u32 = 0b11111001;

        assert_eq!(find_xor_between(n, n), n);
    }

    #[test]
    fn remove_haystack() {
        let max: u32 = 0b11111001;
        let min: u32 = 0b00000101;
        let needle: u32 = 5;
        let haystack = (min..max + 1).filter(|x| *x != needle);

        let haystack_range = find_xor_between(min, max) ^ needle;

        assert_eq!(haystack.fold(0, |a, e| a ^ e), haystack_range);
    }

    #[test]
    fn xor_associative() {
        let max: u32 = 0b11111001;
        let mid: u32 = 0b10001000;
        let min: u32 = 0b00000101;

        assert_eq!(
            find_xor_between(min, max),
            find_xor_between(min, mid - 1) ^ find_xor_between(mid, max)
        );
    }

    #[test]
    fn xor_associative_from_zero() {
        let max: u32 = 0b11111001;
        let min: u32 = 0b00000101;

        assert_eq!(
            find_xor_to(max),
            find_xor_to(min - 1) ^ find_xor_between(min, max)
        );
    }

    #[test]
    fn find_needle_finds_needle() {
        let needle: u32 = 622;
        let haystack: Vec<u32> = (5..743).filter(|v| *v != needle).collect();
        assert_eq!(find_needle(&haystack), needle);
    }
}

fn find_xor_to(n: u32) -> u32 {
    match n % 4 {
        0 => n,
        1 => 1,
        2 => n + 1,
        3 => 0,
        _ => panic!("Can't get here."),
    }
}

fn find_xor_between(min: u32, max: u32) -> u32 {
    if min > max {
        return find_xor_between(max, min);
    }
    find_xor_to(min - 1) ^ find_xor_to(max)
}

pub fn find_needle(haystack: &Vec<u32>) -> u32 {
    let max = haystack.iter().max().unwrap();
    let min = haystack.iter().min().unwrap();
    let haystack_xor = haystack.iter().fold(0, |a, b| a ^ b);
    find_xor_between(*min, *max) ^ haystack_xor
}

pub fn parse_pass(pass: String) -> u32 {
    let mut seat = 0;
    for c in pass.chars() {
        seat = match c {
            'B' | 'R' => (seat << 1) + 1,
            _ => seat << 1,
        };
    }
    seat
}
