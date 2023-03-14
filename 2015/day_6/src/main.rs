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

    let mut light_array: [[u32; 1000]; 1000] = [[0; 1000]; 1000];

    for line in lines {
        if let Ok(command) = line {
            let mut command = command.split(" ");
            if command.next().unwrap().contains("toggle") {
                let mut first_pair = command.next().unwrap().split(",");
                let x1: i32 = first_pair.next().unwrap().parse().unwrap();
                let y1: i32 = first_pair.next().unwrap().parse().unwrap();
                command.next();
                let mut second_pair = command.next().unwrap().split(",");
                let x2: i32 = second_pair.next().unwrap().parse().unwrap();
                let y2: i32 = second_pair.next().unwrap().parse().unwrap();
                change_lights(x1, y1, x2, y2, LightAction::Toggle, &mut light_array);
            } else {
                if command.next().unwrap().contains("on") {
                    let mut first_pair = command.next().unwrap().split(",");
                    let x1: i32 = first_pair.next().unwrap().parse().unwrap();
                    let y1: i32 = first_pair.next().unwrap().parse().unwrap();
                    command.next();
                    let mut second_pair = command.next().unwrap().split(",");
                    let x2: i32 = second_pair.next().unwrap().parse().unwrap();
                    let y2: i32 = second_pair.next().unwrap().parse().unwrap();
                    change_lights(x1, y1, x2, y2, LightAction::TurnOn, &mut light_array);
                } else {
                    let mut first_pair = command.next().unwrap().split(",");
                    let x1: i32 = first_pair.next().unwrap().parse().unwrap();
                    let y1: i32 = first_pair.next().unwrap().parse().unwrap();
                    command.next();
                    let mut second_pair = command.next().unwrap().split(",");
                    let x2: i32 = second_pair.next().unwrap().parse().unwrap();
                    let y2: i32 = second_pair.next().unwrap().parse().unwrap();
                    change_lights(x1, y1, x2, y2, LightAction::TurnOff, &mut light_array);
                }
            }
        }
    }

    let mut total_brightness = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            total_brightness += light_array[x][y];
        }
    }

    println!("Brightness: {total_brightness}");
}

enum LightAction{
    TurnOn,
    TurnOff,
    Toggle,
}

fn change_lights(x1: i32, y1: i32, x2: i32, y2: i32, action: LightAction, light_arr: &mut [[u32; 1000]; 1000]) {
    match action {
        LightAction::TurnOn => {
            for x in x1..=x2 {
                for y in y1..=y2 {
                    light_arr[x as usize][y as usize] += 1;
                }
            }
        },
        LightAction::TurnOff => {
            for x in x1..=x2 {
                for y in y1..=y2 {
                    if light_arr[x as usize][y as usize] != 0 {
                        light_arr[x as usize][y as usize] -= 1;
                    }
                }
            }
        },
        LightAction::Toggle => {
            for x in x1..=x2 {
                for y in y1..=y2 {
                    light_arr[x as usize][y as usize] += 2;
                }
            }
        },
    }
}
