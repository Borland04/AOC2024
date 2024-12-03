use regex::Regex;
use std::io;

pub fn run(input: &mut dyn Iterator<Item = String>) -> io::Result<()> {
    let mul_regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("Invalid regex");

    let result: u64 = input
        .map(|input_str| {
            let mut res: u64 = 0;
            for capture in mul_regex.captures_iter(input_str.as_ref()) {
                println!("{}", &capture[0]);
                let l = capture[1].parse::<i32>().unwrap();
                let r = capture[2].parse::<i32>().unwrap();

                res += l as u64 * r as u64;
            }

            res
        })
        .sum::<u64>();

    println!("{}", result);

    Ok(())
}
