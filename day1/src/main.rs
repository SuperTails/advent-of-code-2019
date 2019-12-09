fn get_fuel(mass: isize) -> isize {
    (mass / 3) - 2
}

fn get_recursive_fuel(mass: isize) -> isize {
    let mut total = mass;
    let mut next_fuel = get_fuel(total);
    while next_fuel > 0 {
        total += next_fuel;
        next_fuel = get_fuel(next_fuel);
    }

    total - mass
}

fn main() {
    assert_eq!(get_recursive_fuel(1969), 966);
    let result = std::fs::read_to_string("data.text")
        .unwrap()
        .lines()
        .map(|s| s.parse::<isize>().unwrap())
        .map(get_recursive_fuel)
        .sum::<isize>();

    println!("Result: {:?}", result);
}
