use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let path = Path::new("input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}:", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();
    let commands = Vec::<Command>::new();

    for line in lines {
        if let Ok(command) = line {
            let mut command = command.split(" ");
            let first_word = command.next().unwrap();
            if let Ok(val) = first_word.parse::<u16>() {

            }
        }
    }

    let wires = HashMap::<String, Wire>::new();
}

struct Wire {
    signal: u16,
}

impl Wire {
    fn and(&self, wire: &Wire) -> u16 {
        self.signal & wire.signal
    }

    fn or(&self, wire: &Wire) -> u16 {
        self.signal | wire.signal
    }

    fn lshift(&self, shift_amount: u16) -> u16 {
        self.signal << shift_amount
    }

    fn rshift(&self, shift_amount: u16) -> u16 {
        self.signal >> shift_amount
    }

    fn not(&self) -> u16 {
        !self.signal
    }
}

enum CommandType{
    And,
    Or,
    LShift,
    RShift,
    Not,
    Number(u16),
}

struct Command {
    command_type: CommandType,
    wire_1: Option<Wire>,
    wire_2: Option<Wire>,
    dest_wire: Wire
}
