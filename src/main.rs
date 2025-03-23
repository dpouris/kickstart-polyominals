use std::collections::HashSet;
use std::{
    io::{self, BufRead},
    process::exit,
};

struct Case<'c> {
    wall: Vec<&'c str>,
    cols: usize,
    rows: usize,
}

const CHUCK_SIZE: usize = size_of::<char>() * 100;

fn main() {
    let mut buf = String::with_capacity(CHUCK_SIZE);
    read_stdin(&mut buf);
    for (idx, case) in get_cases(&buf).iter().enumerate() {
        run_case(idx, &case.wall, case.rows, case.cols);
    }
}

fn read_stdin(buf: &mut String) {
    let stdin = io::stdin();
    loop {
        let Ok(n) = stdin
            .lock()
            .read_line(buf)
            .map_err(|err| println!("err: {err}"))
        else {
            exit(1)
        };

        if n != 0 {
            break; // EOF, break
        };
        buf.reserve(n); // more content, reserve
    }
}

fn get_cases(buf: &str) -> Vec<Case> {
    let mut cases = vec![];
    let (n, rest) = buf.split_once('\n').unwrap();
    let n = n.parse().unwrap();
    let input = rest.split('\n').collect::<Vec<&str>>();
    let mut offset = 0;
    for c_idx in 0..n {
        let dimensions = input[c_idx + offset].split_once(' ').unwrap();
        let rows = dimensions.0.parse::<usize>().unwrap();
        let cols = dimensions.1.parse::<usize>().unwrap();
        let mut wall = input[c_idx + offset + 1..c_idx + offset + 1 + rows].to_vec();
        wall.reverse();
        cases.push(Case { wall, cols, rows });

        offset += rows;
    }
    cases
}

fn run_case(idx: usize, wall: &[&str], rows: usize, cols: usize) {
    let order: Vec<char> = compute_build_order(wall, &vec![], rows, cols);
    let order = match order.len() {
        0 => "-1".to_string(),
        _ => order.iter().collect(),
    };
    println!("Case #{idx}: {order}", idx = idx + 1);
}

fn compute_build_order(wall: &[&str], order: &Vec<char>, rows: usize, cols: usize) -> Vec<char> {
    let letters = wall
        .iter()
        .flat_map(|letters| letters.chars())
        .collect::<HashSet<char>>()
        .into_iter()
        .filter(|el| !order.contains(el))
        .collect::<Vec<char>>();
    let mut _order = order.clone();
    if letters.is_empty() {
        return order.clone();
    }

    let mut pushed = false;
    'letter_loop: for letter in letters {
        for row in 1..rows {
            for col in 0..cols {
                // println!("row: {} -> {}", row, wall[row]); // debugging
                if wall[row].chars().nth(col).unwrap() == letter {
                    let chr = wall[row - 1].chars().nth(col).unwrap();
                    if _order.contains(&chr) || chr == letter {
                        continue;
                    } else {
                        continue 'letter_loop;
                    }
                }
            }
        }
        pushed = true;
        _order.push(letter);
        break;
    }

    if !pushed {
        return _order;
    }

    compute_build_order(wall, &_order, rows, cols)
}
