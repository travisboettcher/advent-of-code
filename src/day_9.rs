use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt;

struct HardDrive {
    hd: Vec<Option<usize>>
}

pub fn day_9() {
    let file = File::open("input-day9.txt").expect("no such file");
    let buf = BufReader::new(file);

    let line: String = buf.lines()
        .flatten()
        .collect();
    
    let sum = defrag(line.clone());
    println!("The filesystem checksum (part 1): {sum}");

    let sum = defrag_2(line);
    println!("The filesystem checksum (part 2): {sum}");
}

fn defrag(compact_repr: String) -> usize {
    let mut hd = HardDrive::from_compact_representation(&compact_repr);
    let mut next_file = hd.next_file_space(hd.hd.len());
    let mut next_free = hd.next_free_space(0);

    while next_free.unwrap() < next_file.unwrap() {
        hd.hd.swap(next_file.unwrap(), next_free.unwrap());
        next_file = hd.next_file_space(next_file.unwrap());
        next_free = hd.next_free_space(next_free.unwrap());
    }
    
    hd.hd.iter()
        .enumerate()
        .take_while(|(_, c)| c.is_some())
        .map(|(i, c)| i * c.unwrap())
        .sum()
}

fn defrag_2(compact_repr: String) -> usize {
    let mut hd = HardDrive::from_compact_representation(&compact_repr);
    let mut next_file = hd.next_file_space(hd.hd.len());

    while next_file.is_some() {
        let file_len = hd.length_of_file(next_file.unwrap());
        let free_space = hd.find_free_space(file_len);

        if free_space.is_some() && free_space.unwrap() < next_file.unwrap() {
            for i in 0..file_len {
                hd.hd.swap(next_file.unwrap() - i, free_space.unwrap() + i);
            }
        }

        if next_file.unwrap() < file_len {
            next_file = None;
        } else {
            next_file = hd.next_file_space(next_file.unwrap() - file_len + 1);
        }
    }

    hd.hd.iter()
        .enumerate()
        .filter(|(_, c)| c.is_some())
        .map(|(i, c)| i * c.unwrap())
        .sum()
}

impl HardDrive {

    fn from_compact_representation(compact_repr: &str) -> HardDrive {
        let ints: Vec<_> = compact_repr.chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect();

        let size: usize = ints.iter().sum();

        let mut hd = Vec::with_capacity(size);
        let mut current_index = 0;
        let mut current_file_id = 0;
        let mut is_file = true;
        for i in ints {
            for _ in 0..i {
                if is_file {
                    hd.insert(current_index, Some(current_file_id));
                } else {
                    hd.insert(current_index, None);
                }
                current_index += 1;
            }

            if is_file { current_file_id += 1; }
            is_file = !is_file;
        }
        
        HardDrive { hd }
    }


    fn next_file_space(&self, start: usize) -> Option<usize> {
        self.hd.iter().enumerate()
            .rev()
            .skip(self.hd.len() - start)
            .find(|(_, f)| f.is_some())
            .map(|c| c.0)
    }

    fn next_free_space(&self, start: usize) -> Option<usize> {
        self.hd.iter().enumerate()
            .skip(start)
            .find(|(_, f)| f.is_none())
            .map(|c| c.0)
    }

    fn length_of_file(&self, start: usize) -> usize {
        let find = self.hd.get(start).unwrap();
        self.hd.iter()
            .rev()
            .skip(self.hd.len() - start - 1)
            .take_while(|x| x == &find)
            .count()
    }

    fn find_free_space(&self, len: usize) -> Option<usize> {
        let mut next_free = self.next_free_space(0);
        while next_free.is_some() && !self.hd.iter().skip(next_free.unwrap()).take(len).all(|c| c.is_none()) {
            next_free = self.next_free_space(next_free.unwrap() + 1);
        }

        if self.hd.iter().skip(next_free.unwrap()).take(len).len() < len {
            None
        } else {
            next_free
        }
    }
}

impl fmt::Display for HardDrive {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.hd.iter()
            .map(|i| {
                if let Some(i) = i {
                    return i.to_string();
                } else {
                    return ".".to_string();
                }
            })
        .collect();

        write!(f, "{s}")
    }

}

#[cfg(test)]
mod tests {
    use crate::day_9::*;

    #[test]
    fn test_produce_hard_drive() {
        let compact_repr = "2333133121414131402";

        let hd = HardDrive::from_compact_representation(&compact_repr);
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.888899", format!("{hd}")
        );
    }

    #[test]
    fn test_defrag() {
        let compact_repr = "2333133121414131402";

        assert_eq!(1928, defrag(compact_repr.to_string()));
    }

    #[test]
    fn test_defrag_2() {
        let compact_repr = "2333133121414131402";

        assert_eq!(2858, defrag_2(compact_repr.to_string()));
    }
}
