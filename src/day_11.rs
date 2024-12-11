use std::{collections::HashMap, io, str::FromStr};

const STEPS: usize = 75;

type Cache = HashMap<(String, usize), u64>;

pub fn run(input: &mut dyn Iterator<Item = String>) -> io::Result<()> {
    let stones = parse_input(input);
    let mut cache: Cache = HashMap::new();

    let result = solve(&stones, &mut cache);

    println!("{}", result);

    Ok(())
}

fn parse_input(input: &mut dyn Iterator<Item = String>) -> Vec<String> {
    input
        .flat_map(|line| {
            line.split(' ')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>()
        })
        .collect()
}

fn solve(stones: &Vec<String>, cache: &mut Cache) -> u64 {
    stones
        .iter()
        .map(|stone| step(stone, STEPS, cache) as u64)
        .sum()
}

fn step(stone: &String, steps_remaining: usize, cache: &mut Cache) -> u64 {
    if steps_remaining <= 0 {
        return 1;
    }

    let maybe_cached = cache.get(&(stone.clone(), steps_remaining));
    if maybe_cached.is_some() {
        return *maybe_cached.unwrap();
    }

    let result = blink(stone)
        .iter()
        .map(|sub_stone| step(sub_stone, steps_remaining - 1, cache))
        .sum();

    cache.insert((stone.clone(), steps_remaining), result);

    result
}

fn blink(stone: &String) -> Vec<String> {
    if stone == "0" {
        vec![String::from_str("1").unwrap()]
    } else if stone == "1" {
        vec![String::from_str("2024").unwrap()]
    } else if stone.len() % 2 == 0 {
        let half_len = stone.len() / 2;
        let (left_str, right_str) = stone.split_at(half_len);

        vec![
            u64::to_string(&left_str.parse::<u64>().unwrap()),
            u64::to_string(&right_str.parse::<u64>().unwrap()),
        ]
    } else {
        let num = stone.parse::<u64>().unwrap();
        vec![u64::to_string(&(num * 2024))]
    }
}
