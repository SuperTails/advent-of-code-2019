use std::path::Path;
use std::collections::HashSet;

type Body = String;

pub fn get_input(path: &Path) -> Vec<(Body, Body)> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let mut l = line.split(')');
            let first = l.next().unwrap().to_string();
            let second = l.next().unwrap().to_string();
            (first, second)
        })
        .collect()
}

pub fn get_objects(data: &[(Body, Body)]) -> HashSet<Body> {
    use std::iter::once;
    data.iter().flat_map(|(a, b)| once(a.clone()).chain(once(b.clone()))).collect()
}

pub fn count_orbits(objects: &HashSet<Body>, data: &[(Body, Body)]) -> usize {
    objects.iter().map(|o| count_orbit(o, data)).sum()
}

pub fn get_base<'a>(object: &str, data: &'a [(Body, Body)]) -> Option<&'a str> {
    data.iter().find(|(_, o)| o == object).map(|(a, _)| a.as_str())
}

pub fn get_all_bases<'a>(object: &'a str, data: &'a [(Body, Body)]) -> Vec<&'a str> {
    std::iter::successors(Some(object), |b| get_base(b, data)).skip(1).collect()
}

pub fn count_orbit(object: &str, data: &[(Body, Body)]) -> usize {
    if let Some(base) = get_base(object, data) {
        1 + count_orbit(base, data)
    } else {
        0
    }
}

pub fn solve(path: &Path) -> usize {
    let input = get_input(Path::new(path));
    println!("Input: {:?}", input);
    
    let objects = get_objects(&input);
    println!("Objects: {:?}", objects);

    let orbit_count = count_orbits(&objects, &input);
    println!("Orbit count: {:?}", orbit_count);

    orbit_count
}

fn main() {
    let input = get_input(Path::new("./data.txt"));
    let you = "YOU".to_string();
    let san = "SAN".to_string();

    let you_bases = get_all_bases(&you, &input);
    let san_bases = get_all_bases(&san, &input);

    let you_bases_hash = you_bases.iter().collect::<HashSet<_>>();

    let first_common = san_bases.iter().find(|b| you_bases_hash.get(b).is_some()).unwrap();
    println!("First common: {:?}", first_common);

    let c1 = you_bases.iter().take_while(|b| b != &first_common).count();
    let c2 = san_bases.iter().take_while(|b| b != &first_common).count();
    
    println!("C1: {}, C2: {}, total: {}", c1, c2, c1 + c2);
}

