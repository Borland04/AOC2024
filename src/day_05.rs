use std::{collections::hash_map, io};

pub fn run(input: &mut dyn Iterator<Item = String>) -> io::Result<()> {
    let (rules, updates) = parse_input(input);
    let left_to_rights_map = generate_left_to_rights_map(&rules);

    let result: u32 = updates
        .iter()
        .filter(|update| is_update_valid(&update, &left_to_rights_map))
        .map(|update| {
            let middle_idx = update.len() / 2;
            update[middle_idx]
        })
        .sum();

    println!("{}", result);

    Ok(())
}

fn parse_input(input: &mut dyn Iterator<Item = String>) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let rules: Vec<(u32, u32)> = input
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut rule = line.split('|');
            let l = rule
                .nth(0)
                .expect("Invalid format: expected 'X|Y'")
                .parse::<u32>()
                .expect("Invalid format: left part of rule must be a number");
            let r = rule
                .nth(0)
                .expect("Invalid format: expected 'X|Y'")
                .parse::<u32>()
                .expect("Invalid format: left part of rule must be a number");

            (l, r)
        })
        .collect();

    let updates: Vec<Vec<u32>> = input
        .map(|line| {
            line.split(',')
                .map(|num_str| {
                    num_str
                        .trim()
                        .parse::<u32>()
                        .expect("Invalid format: updates must contain numbers")
                })
                .collect()
        })
        .collect();

    (rules, updates)
}

fn generate_left_to_rights_map(rules: &Vec<(u32, u32)>) -> hash_map::HashMap<u32, Vec<u32>> {
    rules
        .iter()
        .fold(hash_map::HashMap::new(), |mut acc, (l, r)| {
            let lefts: &mut Vec<u32> = match acc.get_mut(l) {
                Some(arr) => arr,
                None => {
                    let arr = Vec::new();
                    acc.insert(*l, arr);
                    acc.get_mut(l).unwrap()
                }
            };

            lefts.push(*r);
            acc
        })
}

fn is_update_valid(
    update: &Vec<u32>,
    left_to_rights_map: &hash_map::HashMap<u32, Vec<u32>>,
) -> bool {
    for i in 1..update.len() {
        let item = update[i];
        let slice = &update[0..i];

        let is_valid = left_to_rights_map
            .get(&item)
            .map(|rights| rights.iter().all(|right| !slice.contains(right)))
            .unwrap_or(true);

        if !is_valid {
            return false;
        }
    }

    true
}
