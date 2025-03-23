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
    for (idx, case) in get_cases(&buf).into_iter().enumerate() {
        run_case(idx, case);
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

        if n == 0 {
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

fn run_case(idx: usize, case: Case) {
    let unique_letters = case
        .wall
        .iter()
        .flat_map(|letters| letters.chars())
        .collect::<HashSet<char>>();
    let order: Vec<char> = compute_build_order(&case, &unique_letters, &vec![]);
    let order = match order.len() {
        n if n == unique_letters.len() => order.iter().collect(),
        _ => "-1".to_string(),
    };
    println!("Case #{idx}: {order}", idx = idx + 1);
}

fn compute_build_order(
    case: &Case,
    unique_letters: &HashSet<char>,
    order: &Vec<char>,
) -> Vec<char> {
    let letters = unique_letters
        .iter()
        .filter(|el| !order.contains(el))
        .collect::<Vec<&char>>();
    let mut _order = order.clone();
    if letters.is_empty() {
        return order.clone();
    }

    let mut pushed = false;
    'letter_loop: for letter in letters {
        for row in 1..case.rows {
            for col in 0..case.cols {
                // println!("row: {} -> {}", row, case.wall[row]); // debugging
                if case.wall[row].chars().nth(col).unwrap() == *letter {
                    let chr = case.wall[row - 1].chars().nth(col).unwrap();
                    if _order.contains(&chr) || chr == *letter {
                        continue;
                    } else {
                        continue 'letter_loop;
                    }
                }
            }
        }
        pushed = true;
        _order.push(*letter);
        break;
    }

    if !pushed {
        return _order;
    }

    compute_build_order(case, unique_letters, &_order)
}
