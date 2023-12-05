use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug)]
struct Number {
    n: u64,
    line: usize,
    span: (usize, usize),
}
#[derive(Clone, Copy, Debug)]
struct Symbol {
    line: usize,
    location: usize,
    c: char,
}

fn main() {
    let mut parts_number_result = 0u64;
    let mut gear_ratio_result = 0u64;
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    let input_path = env::var("aoc_2023_03_path").unwrap() + "/input.txt";
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);
    for (i, line) in reader.lines().enumerate() {
        let mut number_string = String::new();
        let line = line.unwrap();
        for (j, c) in line.char_indices() {
            if c.is_ascii_digit() {
                number_string.push(c);
            } else {
                if c == '.' {
                    if !number_string.is_empty() {
                        numbers.push(Number {
                            n: u64::from_str_radix(&number_string, 10).unwrap(),
                            line: i,
                            span: (j - number_string.len(), j - 1),
                        });
                        number_string.clear();
                    }
                } else {
                    if !number_string.is_empty() {
                        numbers.push(Number {
                            n: u64::from_str_radix(&number_string, 10).unwrap(),
                            line: i,
                            span: (j - number_string.len(), j - 1),
                        });
                        number_string.clear();
                    }
                    symbols.push(Symbol {
                        line: i,
                        location: j,
                        c,
                    });
                }
            }
            if j == line.chars().count() - 1 && !number_string.is_empty() {
                numbers.push(Number {
                    n: u64::from_str_radix(&number_string, 10).unwrap(),
                    line: i,
                    span: (j - number_string.len(), j - 1),
                });
                number_string.clear();
            }
        }
    }

    for numb in &numbers {
        for symb in &symbols {
            let start = usize::checked_sub(numb.line, 1).or(Some(0)).unwrap();
            if symb.line >= start && symb.line <= (numb.line + 1) {
                let num_start = usize::checked_sub(numb.span.0, 1).or(Some(0)).unwrap();
                if symb.location >= num_start && symb.location <= (numb.span.1 + 1) {
                    parts_number_result += numb.n;
                    break;
                }
            }
        }
    }

    for symb in symbols {
        if symb.c == '*' {
            let mut num_vec: Vec<u64> = vec![];
            for numb in &numbers {
                let start = usize::checked_sub(numb.line, 1).or(Some(0)).unwrap();
                if symb.line >= start && symb.line <= (numb.line + 1) {
                    let num_start = usize::checked_sub(numb.span.0, 1).or(Some(0)).unwrap();
                    if symb.location >= num_start && symb.location <= (numb.span.1 + 1) {
                        num_vec.push(numb.n);
                    }
                }
            }
            if num_vec.len() == 2 {
                gear_ratio_result += num_vec[0] * num_vec[1];
            }
        }
    }
    println!("parts sum result:{}", parts_number_result);
    println!("gears sum result:{}", gear_ratio_result);
}
