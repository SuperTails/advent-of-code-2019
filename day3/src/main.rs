use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up
}

impl Direction {
    pub fn parse(c: char) -> Direction {
        match c {
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'U' => Direction::Up,
            _ => panic!("Invalid {:?}", c)
        }
    }
}

fn parse_one(d: &str) -> (Direction, usize) {
    (Direction::parse(d.chars().next().unwrap()), d[1..].parse::<usize>().unwrap())
}

fn parse_line(l: &str) -> Vec<(Direction, usize)> {
    l.split(',').map(parse_one).collect()
}

const ROW_CENTER: usize = 5000;
const GRID_SIZE: usize = 10000;

// Output is 1000x1000
fn fill_grid(dirs: &[(Direction, usize)]) -> Vec<bool> {
    let mut row = ROW_CENTER;
    let mut col = 4000;
    let mut result = Vec::new();
    result.resize(GRID_SIZE * GRID_SIZE, false);

    for (dir, amount) in dirs {
        println!("Row: {}, dir: {:?}, amount: {}", row, dir, amount);
        for _ in 0..*amount {
            match dir {
                Direction::Up => row -= 1,
                Direction::Down => row += 1,
                Direction::Left => col -= 1,
                Direction::Right => col += 1,
            }

            result[row * GRID_SIZE + col] = true;
        }
    }

    result
}

fn make_positions(dirs: &[(Direction, usize)]) -> Vec<((isize, isize), usize)> {
    let mut row = 0;
    let mut col = 0;
    let mut count = 1;
    let mut result = Vec::new();

    for (dir, amount) in dirs {
        println!("Row: {}, dir: {:?}, amount: {}", row, dir, amount);
        for _ in 0..*amount {
            match dir {
                Direction::Up => row -= 1,
                Direction::Down => row += 1,
                Direction::Left => col -= 1,
                Direction::Right => col += 1,
            }

            result.push(((row, col), count));
            
            count += 1;
        }
    }

    result
}

fn find_ints(dirs_l: &[(Direction, usize)], dirs_r: &[(Direction, usize)]) -> Vec<((isize, isize), usize)> {
    let pos_l = make_positions(dirs_l);
    let pos_r = make_positions(dirs_r);
    println!("Len: {}, {}", pos_l.len(), pos_r.len());

    let found = pos_l.into_iter().collect::<HashMap<(isize, isize), usize>>();

    pos_r
        .into_iter()
        .filter_map(|(pos, delay)| {
            let delay2 = found.get(&pos)?;
            Some((pos, delay + delay2))
        })
        .collect::<Vec<_>>()
}

fn common(g1: &[bool], g2: &[bool]) -> Vec<(usize, usize)> {
    g1.iter().enumerate().zip(g2).filter_map(|((idx, e1), e2)| {
        if *e1 && *e2 {
            Some(idx)
        } else {
            None
        }
    }).map(|idx| (idx / GRID_SIZE, idx % GRID_SIZE)).collect()
}

fn abs_diff(lhs: usize, rhs: usize) -> usize {
    if lhs < rhs {
        rhs - lhs
    } else {
        lhs - rhs
    }
}

fn center_dist(r: usize, c: usize) -> usize {
    abs_diff(r, ROW_CENTER) + abs_diff(c, 4000)
}

fn min_dist(inters: &[(isize, isize)]) -> ((isize, isize), isize) {
    inters.iter().map(|(r, c)| ((*r, *c), r.abs() + c.abs())).min_by_key(|(_, d)| *d).unwrap()
}

fn main() {
    let inputs = std::fs::read_to_string("./data.txt")
        .unwrap()
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();

    println!("Input: {:?}", inputs);

    let ints = find_ints(&inputs[0], &inputs[1]);

    println!("Ints: {:?}", ints);

    let min = ints.iter().min_by_key(|(pos, delay)| *delay).unwrap();

    println!("Min: {:?}", min);
}
