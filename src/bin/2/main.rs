use std::error::Error;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

//if any round has at least one over max, for any round in game/line, discard
//else sum game number

fn valid_game(s: &str) -> Option<u64> {
    let i = s.find(':')?;
    let bytes = &s.as_bytes();
    let games = std::str::from_utf8(&bytes[i + 2..]).expect("not utf-8");
    let rounds: Vec<&str> = games.split(';').map(|s| s.trim()).collect();
    for r in rounds {
        for color in r.split(',').map(|s| s.trim()) {
            if color
                .find("red")
                .is_some_and(|_| color.split(' ').next().unwrap().parse::<u64>().unwrap() > MAX_R)
                || color.find("green").is_some_and(|_| {
                    color.split(' ').next().unwrap().parse::<u64>().unwrap() > MAX_G
                })
                || color.find("blue").is_some_and(|_| {
                    color.split(' ').next().unwrap().parse::<u64>().unwrap() > MAX_B
                })
            {
                return None;
            };
        }
    }
    let nums = &bytes[5..i];
    let num = std::str::from_utf8(nums).expect("not utf-8");
    num.parse().ok()
}

const MAX_R: u64 = 12;
const MAX_G: u64 = 13;
const MAX_B: u64 = 14;

fn main() -> Result<(), Box<dyn Error>> {
    let lines = aoc::input::lines(2)?;
    let num: u64 = lines
        .par_iter()
        .map(|s| valid_game(s))
        .filter_map(|num| num)
        .sum();
    println!("{num}");
    Ok(())
}
