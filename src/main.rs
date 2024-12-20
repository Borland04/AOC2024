use std::io::prelude::*;
use std::{fs::File, io::BufReader};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_08;
mod day_10;
mod day_11;

fn main() {
    //day_01::run(&mut read_input("inputs/01.txt")).expect("Failed run of puzzle");
    //day_02::run(&mut read_input("inputs/02.txt")).expect("Failed run of puzzle");
    //day_03::run(&mut read_input("inputs/03.txt")).expect("Failed run of puzzle");
    //day_04::run(&mut read_input("inputs/04.txt")).expect("Failed run of puzzle");
    //day_05::run(&mut read_input("inputs/05.txt")).expect("Failed run of puzzle");
    //day_06::run(&mut read_input("inputs/06.txt")).expect("Failed run of puzzle");
    //day_08::run(&mut read_input("inputs/08.txt")).expect("Failed run of puzzle");
    //day_10::run(&mut read_input("inputs/10.txt")).expect("Failed run of puzzle");
    day_11::run(&mut read_input("inputs/11.txt")).expect("Failed run of puzzle");
}

fn read_input(fname: &str) -> impl Iterator<Item = String> {
    let input_file =
        File::open(fname).expect(format!("ERROR: Unable to read file {fname}").as_str());
    let buf_input = BufReader::new(input_file);
    buf_input
        .lines()
        .map(|it| it.expect("IO Exception while reading file lines"))
}
