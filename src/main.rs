use std::io::prelude::*;
use std::{fs::File, io::BufReader};

mod day_01;
mod day_02;

fn main() {
    // day_01::run(&mut read_input("inputs/01.txt")).expect("Failed run of puzzle");
    day_02::run(&mut read_input("inputs/02.txt")).expect("Failed run of puzzle");
}

fn read_input(fname: &str) -> impl Iterator<Item = String> {
    let input_file =
        File::open(fname).expect(format!("ERROR: Unable to read file {fname}").as_str());
    let buf_input = BufReader::new(input_file);
    buf_input
        .lines()
        .map(|it| it.expect("IO Exception while reading file lines"))
}