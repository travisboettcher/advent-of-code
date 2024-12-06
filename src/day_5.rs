use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Fact {
    x: u32,
    y: u32
}

pub fn day_5() {
    let file = File::open("input-day5.txt").expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .flatten()
        .collect();

    let facts: Vec<Fact> = lines.iter()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (x, y) = l.split_once('|').unwrap();
            Fact{x: x.parse().unwrap(), y: y.parse().unwrap()}
        })
        .collect();

    let updates: Vec<Vec<u32>> = lines.iter()
        .skip_while(|l| l.contains('|') || l.is_empty())
        .map(|l| l.split(',').map(|p| p.parse::<u32>().unwrap()).collect())
        .collect();

    let sum = sum_in_order_updates(&updates, &facts);

    println!("The sum of the middle page for the correct ordered pages is: {sum}");

    let sum_of_reordered_updates: u32 = updates.iter()
        .filter(|u| !in_order(&u, &facts))
        .map(|u| re_order(&u, &facts))
        .map(|u| u[(u.len() + 1)/2 - 1])
        .sum();

    println!("The sum of the middle page for the re-ordered pages is: {sum_of_reordered_updates}");
}

fn sum_in_order_updates(updates: &Vec<Vec<u32>>, facts: &Vec<Fact>) -> u32 {
    let mut sum = 0;
    for update in updates.iter() {
        if in_order(&update, &facts) {
            sum += update[(update.len() + 1)/2 - 1];
        }
    }
    sum
}

fn in_order(update: &Vec<u32>, facts: &Vec<Fact>) -> bool {
    let mut in_order = true;
    for fact in facts.iter() {
        let x_pos = update.iter().position(|&p| p == fact.x);
        let y_pos = update.iter().position(|&p| p == fact.y);

        in_order &= match (x_pos, y_pos) {
            (Some(x), Some(y)) => x < y,
            _ => true
        };
    }
    in_order
}

fn re_order(update: &Vec<u32>, facts: &Vec<Fact>) -> Vec<u32> {
    let mut new_update: Vec<u32> = vec![update[0]];
    'update: for page in update.iter().skip(1) {
        for i in 0..new_update.len() {
            let head: Vec<u32> = new_update[..i].to_vec();
            let p: Vec<u32> = vec![*page];
            let tail: Vec<u32> = new_update[i..].to_vec();
            let temp_update: Vec<u32> = [head, p, tail].concat();
           
            if in_order(&temp_update, &facts) {
                new_update = temp_update;
                continue 'update;
            }
        }

        let mut temp_update: Vec<u32> = new_update[..].to_vec();
        temp_update.push(*page);
        if in_order(&temp_update, &facts) {
            new_update = temp_update;
        } else {
            panic!("Unable to find ordered list for {update:?}");
        }
    }
    new_update
}

#[cfg(test)]
mod tests {
    use crate::day_5::{Fact, in_order, sum_in_order_updates, re_order};

    #[test]
    fn test_in_order() {
        let updates = vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47]
        ];
        let facts = vec![
            Fact{x: 47, y: 53 },
            Fact{x: 97, y: 13 },
            Fact{x: 97, y: 61 },
            Fact{x: 97, y: 47 },
            Fact{x: 75, y: 29 },
            Fact{x: 61, y: 13 },
            Fact{x: 75, y: 53 },
            Fact{x: 29, y: 13 },
            Fact{x: 97, y: 29 },
            Fact{x: 53, y: 29 },
            Fact{x: 61, y: 53 },
            Fact{x: 97, y: 53 },
            Fact{x: 61, y: 29 },
            Fact{x: 47, y: 13 },
            Fact{x: 75, y: 47 },
            Fact{x: 97, y: 75 },
            Fact{x: 47, y: 61 },
            Fact{x: 75, y: 61 },
            Fact{x: 47, y: 29 },
            Fact{x: 75, y: 13 },
            Fact{x: 53, y: 13 }
        ];

        assert_eq!(
            vec![true, true, true, false, false, false], 
            updates.iter().map(|update| in_order(&update, &facts)).collect::<Vec<bool>>()
        );

        assert_eq!(143, sum_in_order_updates(&updates, &facts));
    }

    #[test]
    fn test_re_order() {
        let update = vec![75,97,47,61,53];
        let facts = vec![
            Fact{x: 47, y: 53 },
            Fact{x: 97, y: 13 },
            Fact{x: 97, y: 61 },
            Fact{x: 97, y: 47 },
            Fact{x: 75, y: 29 },
            Fact{x: 61, y: 13 },
            Fact{x: 75, y: 53 },
            Fact{x: 29, y: 13 },
            Fact{x: 97, y: 29 },
            Fact{x: 53, y: 29 },
            Fact{x: 61, y: 53 },
            Fact{x: 97, y: 53 },
            Fact{x: 61, y: 29 },
            Fact{x: 47, y: 13 },
            Fact{x: 75, y: 47 },
            Fact{x: 97, y: 75 },
            Fact{x: 47, y: 61 },
            Fact{x: 75, y: 61 },
            Fact{x: 47, y: 29 },
            Fact{x: 75, y: 13 },
            Fact{x: 53, y: 13 }
        ];

        assert_eq!(
            vec![97,75,47,61,53],
            re_order(&update, &facts)
        );

    }

}
