use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead},
};

fn str_to_num(s: &str) -> u128 {
    let l = s
        .par_char_indices()
        .filter(|(_num, ch)| char::is_numeric(*ch))
        .min_by_key(|(num, _ch)| *num)
        .unwrap()
        .1;
    let r = s
        .par_char_indices()
        .filter(|(_num, ch)| char::is_numeric(*ch))
        .max_by_key(|(num, _ch)| *num)
        .unwrap()
        .1;
    let digits = format!("{l}{r}");
    digits.parse::<u128>().unwrap()
}

fn str_to_num_2(s: &str) -> Option<u128> {
    let nums: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
    ]);
    let indices: Vec<(usize, &str)> = nums
        .iter()
        .map(|(&refstr, &refnum)| (s.find(refstr), refnum))
        .filter(|x| x.0.is_some())
        .map(|(x, y)| (x.unwrap(), y))
        .collect();

    let min = indices.iter().min()?.1;
    let indices: Vec<(usize, &str)> = nums
        .iter()
        .map(|(&refstr, &refnum)| (s.rfind(refstr), refnum))
        .filter(|x| x.0.is_some())
        .map(|(x, y)| (x.unwrap(), y))
        .collect();

    let max = indices.iter().max()?.1;
    Some(format!("{min}{max}").parse::<u128>().unwrap())
}
use rayon::{iter::ParallelIterator, str::ParallelString};

fn main() -> Result<(), Box<dyn Error>> {
    let num: u128 = io::BufReader::new(File::open("src/bin/1/input.txt")?)
        .lines()
        .map(std::result::Result::unwrap)
        .map(String::from)
        .map(str_to_num)
        .sum();
    println!("{num}");

    let num2: u128 = io::BufReader::new(File::open("src/bin/1/input.txt")?)
        .lines()
        .map(std::result::Result::unwrap)
        .map(String::from)
        .map(|s| (s.clone(),str_to_num_2(&s).unwrap()))
        .inspect(|f| println!("{f:?}"))
        .map(|(s,v)|v)
        .sum();
    println!("{num2}");
    Ok(())
}
