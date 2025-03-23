use std::collections::HashSet;
use std::{
    io::{self, BufRead, IsTerminal},
    process::exit,
};

struct Case<'c> {
    wall: Vec<&'c str>,
    cols: usize,
    rows: usize,
}

fn main() {
    let mut buf = String::with_capacity(size_of::<char>() * 100);
    read_stdin(&mut buf);
    for (idx, case) in get_cases(&buf).iter().enumerate() {
        run_case(idx, &case.wall, case.rows, case.cols);
    }
}

fn get_cases(buf: &str) -> Vec<Case> {
    let mut cases_vec = vec![];
    let (n, cases) = buf.split_once('\n').unwrap();
    let n = n.parse().unwrap();
    let cases = cases.split('\n').collect::<Vec<&str>>();
    let mut offset = 0;
    for c_idx in 0..n {
        let size = cases[c_idx + offset].split_once(' ').unwrap();
        let r = size.0.parse::<usize>().unwrap();
        let c = size.1.parse::<usize>().unwrap();
        let mut wall = cases[c_idx + offset + 1..c_idx + offset + 1 + r].to_vec();
        wall.reverse();
        offset += r;
        cases_vec.push(Case {
            wall,
            cols: c,
            rows: r,
        });
    }
    cases_vec
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
        'row_loop: for row in 1..rows {
            for col in 0..cols {
                // println!("row: {} -> {}", row, wall[row]); // debugging
                if row == 0 {
                    continue 'row_loop;
                }
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

        if stdin.is_terminal() || n == 0 {
            // no input file or EOF, break
            break;
        };
        buf.reserve(n); // more content, reserve
    }
}
