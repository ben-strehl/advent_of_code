use std::cmp::min;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}:", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();

    let mut total_paper = 0;
    let mut total_ribbon = 0;
    let mut l:i32;
    let mut w:i32;
    let mut h:i32;

    for line in lines {
        if let Ok(dims) = line {
            let mut dims = dims.split("x");
            l = dims.next().unwrap().parse().unwrap();
            w = dims.next().unwrap().parse().unwrap();
            h = dims.next().unwrap().parse().unwrap();
            total_paper += 2*l*w + 2*w*h + 2*l*h + least_area(l, w, h);
            total_ribbon += l*w*h + least_perimeter(l, w, h);
        }
    }

    println!("Total paper:{total_paper} sq ft\n Total ribbon:{total_ribbon} ft");
}

fn least_area(a: i32, b: i32, c: i32) -> i32 {
    min(a*b,min(b*c,a*c))
}

fn least_perimeter(a: i32, b: i32, c: i32) -> i32 {
    min(2*a+2*b,min(2*b+2*c,2*a+2*c))
}
