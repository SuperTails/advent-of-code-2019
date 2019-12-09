use std::ops::RangeInclusive;

const PASSWORD_RANGE: RangeInclusive<u32> = 206938..=679128;

fn is_valid(pass: u32) -> bool {
    let pass_str = format!("{}", pass);
    let pass_bytes = pass_str.bytes().collect::<Vec<_>>();

    let has_double = 
        pass_bytes.windows(4).filter(|w| w[1] == w[2] && w[0] != w[1] && w[3] != w[1]).count() != 0 ||
        (pass_bytes[0] == pass_bytes[1] && pass_bytes[0] != pass_bytes[2]) || 
        (pass_bytes[pass_bytes.len() - 1] == pass_bytes[pass_bytes.len() - 2] && pass_bytes[pass_bytes.len() - 1] != pass_bytes[pass_bytes.len() - 3]);
    let is_incr = pass_bytes.iter().try_fold(0, |min, new| {
        if min <= *new {
            Ok(*new)
        } else {
            Err(*new)
        }
    }).is_ok();

    has_double && is_incr
}

fn main() {
    assert!(is_valid(112233));
    assert!(!is_valid(123444));
    assert!(is_valid(111122));
    let result = PASSWORD_RANGE.filter(|p| is_valid(*p)).count();
    println!("Result: {:?}", result);
    println!("Hello, world!");
}
