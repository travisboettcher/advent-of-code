use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

type BiFunc = fn(usize, usize) -> usize;

pub fn day_7() {
    let file = File::open("input-day7.txt").expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .flatten()
        .collect();

    let re = Regex::new(r"(\d+): (.*)").unwrap();
    let mut sum = 0;
    for line in lines.iter() {
        let captures = re.captures(&line).unwrap();
        let test: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let operands: Vec<usize> = captures.get(2).unwrap().as_str().split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        if test_value(test, &operands, &[add, mult]) {
            sum += test;
        }
    }

    println!("Total calibration result (part 1): {sum}");

    let mut sum = 0;
    for line in lines.iter() {
        let captures = re.captures(&line).unwrap();
        let test: usize = captures.get(1).unwrap().as_str().parse().unwrap();
        let operands: Vec<usize> = captures.get(2).unwrap().as_str().split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        if test_value(test, &operands, &[add, mult, concat]) {
            sum += test;
        }
    }

    println!("Total calibration result (part 2): {sum}");
}

fn add(a: usize, b: usize) -> usize { a + b }
fn mult(a: usize, b: usize) -> usize { a * b }
fn concat(a: usize, b: usize) -> usize { format!("{}{}", a, b).parse().unwrap() }

fn test_value(test: usize, operands: &[usize], operators: &[BiFunc]) -> bool {
    let (head, tail) = operands.split_first().unwrap();
    test_value_recurse(test, *head, tail, operators)
}

fn test_value_recurse(test: usize, sum_so_far: usize, operands: &[usize], operators: &[BiFunc]) -> bool {
    if operands.is_empty() {
       return test == sum_so_far;
    };

    let (head, tail) = operands.split_first().unwrap();
    sum_so_far <= test &&
        operators.iter().any(|f| test_value_recurse(test, f(sum_so_far, *head), tail, operators))
}

#[cfg(test)]
mod tests {
    use crate::day_7::{test_value, add, mult, concat};

    #[test]
    fn test_testing_value() {
        // 292: 11 6 16 20
        let test = 292;
        let operands = [11, 6, 16, 20];

        assert!(test_value(test, &operands, &[add, mult]));
    }

    #[test]
    fn test_with_concat() {
        // 7290: 6 8 6 15
        let test = 7290;
        let operands = [6, 8, 6, 15];

        assert!(test_value(test, &operands, &[add, mult, concat]));
    }

}
