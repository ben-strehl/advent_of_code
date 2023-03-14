use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::RegexSet;
use fancy_regex::Regex;

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}:", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();

    let mut nice_count = 0;
    let mut total_count = 0;
    let nice1= Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let nice2 = Regex::new(r"(\w)\1").unwrap();

    let naughty_set = RegexSet::new(&[
        r"ab",
        r"cd",
        r"pq",
        r"xy",
    ]).unwrap();

    let mut nice_count_new = 0;
    let new_nice1 = Regex::new(r"(\w\w).*(\1)").unwrap();
    let new_nice2 = Regex::new(r"(\w)\w(\1)").unwrap();

    let new_naughty = Regex::new(r"(\w)\1\1").unwrap();

    for line in lines {
        let word = &line.unwrap()[..];
        if nice1.is_match(word).unwrap() && nice2.is_match(word).unwrap() && !naughty_set.is_match(word) {
            nice_count += 1;
        }

        if new_nice1.is_match(word).unwrap() && new_nice2.is_match(word).unwrap() && !new_naughty.is_match(word).unwrap() {
            nice_count_new += 1;
        }
       total_count += 1;
    }

    println!("Nice words:{nice_count}\nTotal words:{total_count}");

    println!("New nice count:{nice_count_new}");


}
