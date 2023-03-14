use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;
    let mut unique = Vec::<(i32, i32)>::new();
    let mut santa_moves = true;
    unique.push((0, 0));

    for c in contents.chars() {

        if santa_moves {
            match c {
                '^' => santa_y += 1,
                'v' => santa_y -= 1,
                '>' => santa_x += 1,
                '<' => santa_x -= 1,
                _ => ()
            }
            if !unique.contains(&(santa_x, santa_y)) {
                unique.push((santa_x, santa_y));
            }
        } else {
            match c {
                '^' => robo_y += 1,
                'v' => robo_y -= 1,
                '>' => robo_x += 1,
                '<' => robo_x -= 1,
                _ => ()
            }
            if !unique.contains(&(robo_x, robo_y)) {
                unique.push((robo_x, robo_y));
            }
        }

        santa_moves = !santa_moves;
    }

    println!("{}", unique.len());
}
