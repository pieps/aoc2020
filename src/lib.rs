#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pass_converts() {
        assert_eq!(parse_pass(String::from("FBFBBFFRLR")), 357);
    }
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
