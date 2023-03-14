use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut floor = 0;
    let mut char_count = 1;

    for c in contents.chars() {
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }
        if floor == -1 {
            println!("{char_count}");
        }
        char_count += 1;
    }

    println!("{floor}");
}
