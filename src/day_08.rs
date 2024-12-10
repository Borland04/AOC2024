use std::{
    collections::{HashMap, HashSet},
    io,
};

pub fn run(input: &mut dyn Iterator<Item = String>) -> io::Result<()> {
    let (antennas, w, h) = parse_input(input);

    let result = solve(&antennas, w, h);

    println!("{}", result);

    Ok(())
}

type Antenna = (char, i32, i32);

fn parse_input(input: &mut dyn Iterator<Item = String>) -> (Vec<Antenna>, u32, u32) {
    let map: Vec<Vec<char>> = input.map(|line| line.chars().collect()).collect();

    let w = map[0].len();
    let h = map.len();
    let mut antennas: Vec<Antenna> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if map[y][x] != '.' {
                antennas.push((map[y][x], x as i32, y as i32));
            }
        }
    }

    (antennas, w as u32, h as u32)
}

fn solve(antennas: &Vec<Antenna>, w: u32, h: u32) -> i32 {
    let antenna_groups = group_antennas(antennas);

    let non_distinct_anti_antennas = antenna_groups
        .iter()
        .flat_map(|grp| unfold_to_pairs(grp))
        .flat_map(|(a, b)| find_anti_antennas(a, b, w, h))
        .filter(|(_, x, y)| is_on_map(*x, *y, w, h))
        .map(|(_, x, y)| (x, y));
    let distinct: HashSet<(i32, i32)> = non_distinct_anti_antennas.collect();

    distinct.len() as i32
}

fn group_antennas(antennas: &Vec<Antenna>) -> Vec<Vec<Antenna>> {
    let mut groups_map: HashMap<char, Vec<Antenna>> = HashMap::new();

    for a in antennas {
        let key = a.0;
        if !groups_map.contains_key(&key) {
            groups_map.insert(key, Vec::new());
        }

        let group = groups_map.get_mut(&key).unwrap();
        group.push(*a);
    }

    groups_map.into_values().collect()
}

fn unfold_to_pairs<T>(arr: &Vec<T>) -> Vec<(T, T)>
where
    T: Copy,
{
    let mut result = Vec::new();
    for i in 0..arr.len() - 1 {
        let current = arr[i];
        let mut pairs: Vec<(T, T)> = arr[i + 1..]
            .iter()
            .map(|other| (current, other.clone()))
            .collect();
        result.append(&mut pairs);
    }

    result
}

fn find_anti_antennas(antenna_a: Antenna, antenna_b: Antenna, w: u32, h: u32) -> Vec<Antenna> {
    let xa = antenna_a.1;
    let ya = antenna_a.2;

    let xb = antenna_b.1;
    let yb = antenna_b.2;

    let dx = xb as i32 - xa as i32;
    let dy = yb as i32 - ya as i32;

    let gcd: i32 = gcd::euclid_u32(i32::abs(dx) as u32, i32::abs(dy) as u32) as i32;
    let step_x = dx / gcd;
    let step_y = dy / gcd;

    let mut result = Vec::new();
    {
        let mut current_antinode_forward = (xb, yb);
        while is_on_map(current_antinode_forward.0, current_antinode_forward.1, w, h) {
            let (xaa, yaa) = current_antinode_forward;
            result.push((antenna_a.0, xaa, yaa));

            current_antinode_forward = (xaa + step_x, yaa + step_y);
        }
    }

    {
        let mut current_antinode_backward = (xa, ya);
        while is_on_map(
            current_antinode_backward.0,
            current_antinode_backward.1,
            w,
            h,
        ) {
            let (xaa, yaa) = current_antinode_backward;
            result.push((antenna_a.0, xaa, yaa));

            current_antinode_backward = (xaa - step_x, yaa - step_y);
        }
    }

    result
}

fn is_on_map(x: i32, y: i32, w: u32, h: u32) -> bool {
    x >= 0 && (x as u32) < w && y >= 0 && (y as u32) < h
}
