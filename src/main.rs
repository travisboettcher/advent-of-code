use crate::day_1::day_1;
use crate::day_2::day_2;
use crate::day_3::day_3;
use crate::day_4::day_4;
use crate::day_5::day_5;
use crate::day_6::day_6;
use std::env;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "1" => day_1(),
        "2" => day_2(),
        "3" => day_3(),
        "4" => day_4(),
        "5" => day_5(),
        "6" => day_6(),
        _ => panic!("day not implemented yet")
    }
}

