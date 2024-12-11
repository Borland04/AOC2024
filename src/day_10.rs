use std::{collections::HashSet, io};

pub fn run(input: &mut dyn Iterator<Item = String>) -> io::Result<()> {
    let map = parse_input(input);

    let result = solve(&map);

    println!("{}", result);

    Ok(())
}

fn parse_input(input: &mut dyn Iterator<Item = String>) -> Vec<Vec<u8>> {
    input
        .map(|line| {
            line.chars()
                .map(|digit| String::from(digit).parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

fn solve(map: &Vec<Vec<u8>>) -> i32 {
    let mut result = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 0 {
                let trails = find_trails(x as i32, y as i32, map, 0);
                result += trails.iter().map(|tr| *tr.last().unwrap()).len();
            }
        }
    }

    result as i32
}

fn find_trails(
    current_x: i32,
    current_y: i32,
    map: &Vec<Vec<u8>>,
    current_height: u8,
) -> Vec<Vec<(i32, i32)>> {
    if current_height == 9 {
        vec![vec![(current_x, current_y)]]
    } else {
        vec![
            (current_x + 1, current_y),
            (current_x - 1, current_y),
            (current_x, current_y + 1),
            (current_x, current_y - 1),
        ]
        .iter()
        .filter(|(next_x, next_y)| is_on_map(*next_x, *next_y, map))
        .map(|(next_x, next_y)| (map[*next_y as usize][*next_x as usize], *next_x, *next_y))
        .filter(|(next_height, _, _)| *next_height == current_height + 1)
        .flat_map(|(next_height, next_x, next_y)| find_trails(next_x, next_y, map, next_height))
        .map(|trail| {
            let mut clone = trail.clone();
            clone.insert(0, (current_x, current_y));
            clone
        })
        .collect()
    }
}

fn is_on_map(x: i32, y: i32, map: &Vec<Vec<u8>>) -> bool {
    y >= 0 && (y as usize) < map.len() && x >= 0 && (x as usize) < map[0].len()
}
