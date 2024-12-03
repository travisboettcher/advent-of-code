use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_2() {
    let file = File::open("input-day2.txt").expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .flatten()
        .collect();

    let mut cnt = 0;
    for line in lines {
        let report = parse_report(line);

        if is_safe(&report, true) { cnt += 1 }
    }        
    
    println!("Number of safe reports: {}", cnt);

}

fn parse_report(line: String) -> Vec<i32> {
    let mut s = line.split_whitespace();
    let mut report: Vec<i32> = vec![];
    while let Some(a) = s.next() {
        report.push(a.parse().unwrap());
    }
    report
}

fn is_safe(report: &Vec<i32>, try_subsets: bool) -> bool {
    let is_increasing = report.get(0).unwrap() - report.get(1).unwrap_or(&0) < 0;
    let mut safe = true;
    for (i, a) in report.iter().enumerate() {
        if let Some(b) = report.get(i + 1) {
            safe &= (a - b < 0) == is_increasing;

            let diff = (a - b).abs();
            safe &= diff >= 1 && diff < 4;
        }
    }

    if try_subsets {
        for i in 0..report.len() {
            let new_report = [&report[..i], &report[i + 1..]].concat();
            safe = is_safe(&new_report, false);

            if safe {
                return true;
            }
        }
    }

    safe
}
