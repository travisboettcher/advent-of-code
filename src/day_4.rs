use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day_4() {
    let file = File::open("input-day4.txt").expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .flatten()
        .collect();

    let count = find_xmas(&lines);
    println!("[Part 1] Total count: {count}");

    let mut count = 0;
    for i in 0..lines.iter().count()-2 {
        let ls: [&String;3] = [
            lines.get(i).unwrap(), lines.get(i+1).unwrap(), lines.get(i+2).unwrap()
        ];
        let n = find_x_mas(ls);
        count += n;
    }

    println!("[Part 2] Total count: {count}");
}

fn find_xmas(lines: &Vec<String>) -> usize {
    let mut count = 0;
    for line in lines.iter() {
        let n = find_xmas_in_line(&line);
        count += n;
    }

    for i in 0..lines.iter().count()-3 {
        let ls: [&String;4] = [
            lines.get(i).unwrap(), lines.get(i+1).unwrap(), lines.get(i+2).unwrap(), lines.get(i+3).unwrap()
        ];
        let n = find_xmas_across_lines(ls);
        count += n;
    }
    count
}

fn find_xmas_in_line(line: &str) -> usize {
    let re = Regex::new(r"XMAS").unwrap();
    let forward_count = re.find_iter(line).count();

    let re = Regex::new(r"SAMX").unwrap();
    let backward_count = re.find_iter(line).count();
    forward_count + backward_count
}

fn find_xmas_across_lines(lines: [&String;4]) -> usize {
    let mut count = 0;
    for (idx, c) in lines[0].as_str().chars().enumerate() {
        let mut s_vert = String::new();
        s_vert.push(lines[1].as_bytes()[idx] as char);
        s_vert.push(lines[2].as_bytes()[idx] as char);
        s_vert.push(lines[3].as_bytes()[idx] as char);

        let mut s_diag_r = String::new();
        let mut s_diag_l = String::new();
        if idx <= lines[0].len() - 4 {
            s_diag_r.push(lines[1].as_bytes()[idx+1] as char);
            s_diag_r.push(lines[2].as_bytes()[idx+2] as char);
            s_diag_r.push(lines[3].as_bytes()[idx+3] as char);
        }

        if idx >= 3 {
            s_diag_l.push(lines[1].as_bytes()[idx-1] as char);
            s_diag_l.push(lines[2].as_bytes()[idx-2] as char);
            s_diag_l.push(lines[3].as_bytes()[idx-3] as char);
        }

        if c == 'X' {
            if s_vert == "MAS" { count += 1}
            if s_diag_r == "MAS" { count += 1} 
            if s_diag_l == "MAS" { count += 1}
        } else if c == 'S' {
            if s_vert == "AMX" { count += 1}
            if s_diag_r == "AMX" { count += 1}
            if s_diag_l == "AMX" { count += 1}
        }
    }
    count
}

fn find_x_mas(lines: [&String;3]) -> usize {
    let mut count = 0;
    for (idx, c) in lines[0].as_str().chars().enumerate() {
        let mut s_diag_r = String::new();
        let mut s_diag_l = String::new();
        if idx <= lines[0].len() - 3 {
            s_diag_r.push(c);
            s_diag_r.push(lines[1].as_bytes()[idx+1] as char);
            s_diag_r.push(lines[2].as_bytes()[idx+2] as char);
            
            s_diag_l.push(lines[0].as_bytes()[idx+2] as char);
            s_diag_l.push(lines[1].as_bytes()[idx+1] as char);
            s_diag_l.push(lines[2].as_bytes()[idx] as char); 
        }

        if (s_diag_r == "MAS" || s_diag_r == "SAM")
            && (s_diag_l == "MAS" || s_diag_l == "SAM") {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::day_4::{find_xmas, find_xmas_in_line, find_xmas_across_lines, find_x_mas};

    #[test]
    fn test_find_xmas_in_line() {
        let line: &str = "XMASASDFSAMXASDFSAMXMASAMX";

        assert_eq!(5, find_xmas_in_line(line));
    }

    #[test]
    fn test_find_xmas_across_lines() {
        let lines: [&String;4] = [
            &"XSSXSX..".to_string(),
            &"MA.AM...".to_string(),
            &"AMMAMA..".to_string(),
            &"SXS..XS.".to_string()
        ];

        assert_eq!(6, find_xmas_across_lines(lines));
    }

    #[test]
    fn test_find_xmas() {
        let example = r#"
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX"#;

        assert_eq!(18, find_xmas(&example.lines().map(|s| s.to_string()).collect()));
    }

    #[test]
    fn test_find_x_mas() {
        let lines: [&String;3] = [
            &"..A..MSMS.".to_string(),
            &".M.S.MAA..".to_string(),
            &"..A.ASMSM.".to_string()
        ];

        assert_eq!(2, find_x_mas(lines));
    }
}
