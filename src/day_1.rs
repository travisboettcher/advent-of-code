use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day_1() {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .flatten()
        .collect();

    let mut list_1 = vec![];
    let mut list_2 = vec![];
    for line in lines {
        let (a, b) = split(line);

        list_1.push(a);
        list_2.push(b);
    }        

    list_1.sort();
    list_2.sort();

    let sorted_list = list_1.iter().zip(list_2.iter());
    let mut sum = 0;
    for (a, b) in sorted_list {
        let a: i32 = a.parse().unwrap();
        let b: i32 = b.parse().unwrap();
        let c = (a - b).abs();
        sum += c;
    }

    println!("The total distance: {}", sum);

    let mut similarity = 0;
    for a in list_1 {
        let n = list_2.iter().filter(|x| x == &&a).collect::<Vec<&String>>().len();
        let a: usize = a.parse().unwrap();
        let c = a * n;
        similarity += c;
    }

    println!("The similarity score: {}", similarity);
}

fn split(line: String) -> (String, String) {
    let mut s = line.split_whitespace();
    let a = s.next().unwrap();
    let b = s.next().unwrap();
    (a.to_owned(), b.to_owned())
}
