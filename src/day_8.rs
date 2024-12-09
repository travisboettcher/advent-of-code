use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::ops::Range;
use multimap::MultiMap;

pub fn day_8() {
    let file = File::open("input-day8.txt").expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .flatten()
        .collect();

    let count = find_antinodes(lines);
    println!("Number of antinodes: {count}");
}

fn find_antinodes(lines: Vec<String>) -> usize {
    let max_x: isize = lines.get(0).unwrap().len() as isize;
    let max_y: isize = lines.len() as isize;

    let mut antennae_locations: MultiMap<char, (usize, usize)> = MultiMap::new();
    for (j, line) in lines.iter().enumerate() {
        line.char_indices()
            .filter(|(_, c)| c != &'.')
            .for_each(|(i, c)| {
                antennae_locations.insert(c, (i, j));
            });
    }

    let mut antinodes = HashSet::new();
    for (_, locs) in antennae_locations.iter_all() {
        for (i, loc_a) in locs.iter().enumerate() {
            for loc_b in locs[i+1..].iter() {
                let points = produce_antinodes(loc_a, loc_b, (0..max_x, 0..max_y));
                for point in points {
                    antinodes.insert(point);
                }
            }
        }
    }

    antinodes.len()
}

type Point = (usize, usize);
fn produce_antinodes(a: &Point, b: &Point, bounds: (Range<isize>, Range<isize>)) -> Vec<(isize, isize)> {
    let mut antinodes = vec![];
    let d_x = b.0 as isize - a.0 as isize;
    let d_y = b.1 as isize - a.1 as isize;
    
    antinodes.push((a.0 as isize, a.1 as isize));
    antinodes.push((b.0 as isize, b.1 as isize));

    let mut antinode_a = (a.0 as isize - d_x, a.1 as isize - d_y);
    while bounds.0.contains(&antinode_a.0) && bounds.1.contains(&antinode_a.1) {
        antinodes.push(antinode_a);
        antinode_a = (antinode_a.0 - d_x, antinode_a.1 - d_y);
    }

    let mut antinode_b = (b.0 as isize + d_x, b.1 as isize + d_y);
    while bounds.0.contains(&antinode_b.0) && bounds.1.contains(&antinode_b.1) {
        antinodes.push(antinode_b);
        antinode_b = (antinode_b.0 + d_x, antinode_b.1 + d_y);
    }

    antinodes
}

#[cfg(test)]
mod tests {
    use crate::day_8::find_antinodes;

    #[test]
    fn test_example() {
        let lines: Vec<String> = vec![
            "............".to_string(),
            "........0...".to_string(),
            ".....0......".to_string(),
            ".......0....".to_string(),
            "....0.......".to_string(),
            "......A.....".to_string(),
            "............".to_string(),
            "............".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "............".to_string(),
            "............".to_string()
        ];

        assert_eq!(34, find_antinodes(lines));
    }

}
