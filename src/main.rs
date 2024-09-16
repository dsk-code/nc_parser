use nc_parser::structs::{
    line_parser::Line,
    state::State,
    
};

use std::{fs::File, io::{BufRead, BufReader,}};


fn main() {
    let file = File::open("test.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let mut state = State::default();
    buf_reader.lines().for_each(|line| {
        let line = Line::new(line.unwrap());
        let result = state.state_update(line).unwrap();
        if let Some(state) = result {
            println!("X{:.3}Y{:.3}", state.get_x(), state.get_y());
        }
        
    });    
}

