use regex::Regex;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn day_3() {
    let file = File::open("input-day3.txt").expect("no such file");
    let mut buf = BufReader::new(file);

    let re_do = Regex::new(r"(?ms)\A(.*?)don't\(\)|do\(\)(.*?)don't\(\)|do\(\)(.*)\z").unwrap();
    let re = Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();

    let mut input = String::new();
    buf.read_to_string(&mut input).expect("cannot read file");

    let mut final_sum: u32 = 0;
    let matches: Vec<&str> = re_do.find_iter(&input).map(|m| m.as_str()).collect();

    for m in matches {
        let sum: u32 = re.captures_iter(&m).map(|caps| {
            let x = caps.name("x").unwrap().as_str();
            let y = caps.name("y").unwrap().as_str();
            x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap()
        }).sum();
        final_sum += sum;
    }

    println!("Answer with conditionals: {final_sum}");

}

