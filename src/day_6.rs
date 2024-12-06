use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use std::result::Result;

#[derive(Debug,Eq,PartialEq,Hash,Clone,Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug)]
struct GameState {
    visited_pos: HashSet<(usize, usize, Direction)>,
    x: usize,
    y: usize,
    direction: Direction,
    frame: u32
}

pub fn day_6() {
    let file = File::open("input-day6.txt").expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .flatten()
        .collect();

    let game_state = play(&lines).expect("Loop found");
    let unique_spots = unique_spots(game_state.visited_pos).len();
    println!("Number of unique spots visited: {unique_spots}");

    let obstruction_spots = add_obstacles(lines);
    println!("Number of spots for obstacles: {obstruction_spots}");
}

fn play(lines: &Vec<String>) -> Result<GameState, ()> {
    let mut starting_pos: Option<(usize, usize)> = None;
    for y in 0..lines.len() {
        if let Some(x) = lines.get(y).unwrap().chars().position(|c| c == '^') {
            starting_pos = Some((x,y));
            break;
        }
    }
    
    let starting_pos = starting_pos.unwrap(); 
    let mut game_state = GameState {
        visited_pos: HashSet::from([(starting_pos.0, starting_pos.1, Direction::UP)]),
        x: starting_pos.0,
        y: starting_pos.1,
        direction: Direction::UP,
        frame: 0
    };

    loop {

        if (game_state.direction == Direction::UP && game_state.y == 0) 
            || (game_state.direction == Direction::DOWN && game_state.y + 1 >= lines.len())
            || (game_state.direction == Direction::LEFT && game_state.x == 0)
            || (game_state.direction == Direction::RIGHT && game_state.x + 1 >= lines.get(0).unwrap().len()) {

                break Ok(game_state);
        }

        let next_pos: (usize, usize) = match game_state.direction {
            Direction::UP => (game_state.x, game_state.y - 1),
            Direction::DOWN => (game_state.x, game_state.y + 1),
            Direction::LEFT => (game_state.x - 1, game_state.y),
            Direction::RIGHT => (game_state.x + 1, game_state.y)
        };

        let c = lines.get(next_pos.1).unwrap().as_bytes()[next_pos.0] as char;
        if c == '#' {
            let next_dir = match game_state.direction {
                Direction::UP => Direction::RIGHT,
                Direction::DOWN => Direction::LEFT,
                Direction::LEFT => Direction::UP,
                Direction::RIGHT => Direction::DOWN
            };

            if !game_state.visited_pos.insert((game_state.x, game_state.y, next_dir)) {
                break Err(());
            }

            game_state = GameState {
                visited_pos: game_state.visited_pos,
                x: game_state.x,
                y: game_state.y,
                direction: next_dir,
                frame: game_state.frame
            };
        } else {
            game_state.visited_pos.insert((next_pos.0, next_pos.1, game_state.direction));
            game_state = GameState {
                visited_pos: game_state.visited_pos,
                x: next_pos.0,
                y: next_pos.1,
                direction: game_state.direction,
                frame: game_state.frame + 1
            }
        }
    }
}

fn add_obstacles(lines: Vec<String>) -> usize {
    let mut count = 0;

    let game_state = play(&lines).unwrap();
    let potential_obstacles = unique_spots(game_state.visited_pos);
    for (x, y) in potential_obstacles {
        let c = lines.get(y).unwrap().as_bytes()[x] as char;
        if c == '^' {
            continue;
        }
   

        let mut s = lines.get(y).unwrap().clone();
        s.replace_range(x..x+1, "#");
        let mod_lines = [lines[..y].to_vec(), vec![s], lines[y+1..].to_vec()].concat();
        if play(&mod_lines).is_err() {
            count += 1;
        }
    }

    count
}

fn unique_spots(visited_spots: HashSet<(usize, usize, Direction)>) -> HashSet<(usize, usize)> {
    visited_spots.iter()
        .map(|(x, y, _)| (*x, *y))
        .collect::<HashSet<(usize, usize)>>()
}

#[cfg(test)]
mod tests {
    use crate::day_6::{play, add_obstacles, unique_spots};

    #[test]
    fn test_play() {
        let example: Vec<String> = vec![
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string()
        ];
        let game_state = play(&example).expect("Loop found");
        let unique_spots = unique_spots(game_state.visited_pos).len();

        assert_eq!(41, unique_spots);
        assert_eq!(6, add_obstacles(example));
    }

}
