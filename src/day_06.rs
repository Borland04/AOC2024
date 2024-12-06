use std::{collections::hash_set, io};

pub fn run(input: &mut dyn Iterator<Item = String>) -> io::Result<()> {
    let map = parse_input(input);

    let result = traverse_guard(&map).len();

    println!("{}", result);

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum MapItem {
    Empty,
    Obstacle,
    Guard(Direction),
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn parse_input(input: &mut dyn Iterator<Item = String>) -> Vec<Vec<MapItem>> {
    input
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => MapItem::Obstacle,
                    '>' => MapItem::Guard(Direction::Right),
                    '<' => MapItem::Guard(Direction::Left),
                    '^' => MapItem::Guard(Direction::Up),
                    'V' => MapItem::Guard(Direction::Down),
                    _ => MapItem::Empty,
                })
                .collect()
        })
        .collect()
}

fn traverse_guard(map: &Vec<Vec<MapItem>>) -> hash_set::HashSet<(i32, i32)> {
    let mut maybe_guard = None;
    for y in 0..map.len() {
        if let Some(_) = maybe_guard {
            break;
        }

        for x in 0..map[y].len() {
            if let MapItem::Guard(dir) = map[y][x] {
                maybe_guard = Some((dir, (x as i32, y as i32)));
                break;
            }
        }
    }

    let mut guard = maybe_guard.expect("Cannot find guard on a map");
    let mut result: hash_set::HashSet<(i32, i32)> = hash_set::HashSet::new();

    while is_on_map(guard.1 .0, guard.1 .1, map) {
        result.insert((guard.1 .0, guard.1 .1));
        guard = step(guard, map);
    }

    result
}

fn step(guard: (Direction, (i32, i32)), map: &Vec<Vec<MapItem>>) -> (Direction, (i32, i32)) {
    println!("{:?}", guard);
    let (dir, (x, y)) = guard;
    let dx = match dir {
        Direction::Left => -1,
        Direction::Right => 1,
        _ => 0,
    };

    let dy = match dir {
        Direction::Up => -1,
        Direction::Down => 1,
        _ => 0,
    };

    let next_x = x + dx;
    let next_y = y + dy;

    if is_on_map(next_x, next_y, map) {
        match map[next_y as usize][next_x as usize] {
            MapItem::Obstacle => step((rotate_guard_right(dir), (x, y)), map),
            _ => (dir, (next_x, next_y)),
        }
    } else {
        (dir, (next_x, next_y))
    }
}

fn is_on_map<T>(x: i32, y: i32, map: &Vec<Vec<T>>) -> bool {
    y >= 0 && y < map.len() as i32 && x >= 0 && x < map[y as usize].len() as i32
}

fn rotate_guard_right(dir: Direction) -> Direction {
    match dir {
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
    }
}
