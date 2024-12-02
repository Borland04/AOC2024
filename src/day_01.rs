use std::io::{prelude::*, Result};
use std::{fs::File, io::BufReader};

pub fn run(input_filename: &str) -> Result<()> {
    let input_file = File::open(input_filename)?;
    let buf_input = BufReader::new(input_file);
    let input = buf_input.lines();

    let (mut left_list, mut right_list) = input
        .map(|l| {
            l.expect("IO error")
                .split(" ")
                .filter(|sub_l| !sub_l.is_empty())
                .map(|sub_l| {
                    sub_l
                        .parse::<i32>()
                        .expect(format!("Value '{sub_l}' should be a number on line ").as_str())
                })
                .collect::<Vec<i32>>()
        })
        .fold(
            (Vec::<i32>::new(), Vec::<i32>::new()),
            |(mut l, mut r), is| {
                l.push(is[0]);
                r.push(is[1]);
                (l, r)
            },
        );

    left_list.sort();
    right_list.sort();

    let result = left_list
        .iter()
        .zip(right_list)
        .map(|(l, r)| (l - r).abs())
        .reduce(|acc, i| acc + i)
        .unwrap_or(0);

    println!("{}", result);

    Ok(())
}
