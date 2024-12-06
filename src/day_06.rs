use std::{
    collections::{hash_set, HashSet},
    io,
};

pub fn run(input: &mut dyn Iterator<Item = String>) -> io::Result<()> {
    let map = parse_input(input);

    let result = solve(&map);

    println!("{}", result);

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum MapItem {
    Empty,
    Obstacle,
    Guard(Direction),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

fn solve(map: &Vec<Vec<MapItem>>) -> i32 {
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

    let mut result: HashSet<(i32, i32)> = HashSet::new();

    let initial_guard = maybe_guard.expect("Cannot find guard on a map");

    let mut obstacle = step(initial_guard, map);
    while is_on_map(obstacle.1 .0, obstacle.1 .1, map) {
        let (_, (obstacle_x, obstacle_y)) = obstacle;
        let mut tmp_map: Vec<Vec<MapItem>> = map.iter().map(|row| row.clone()).collect();
        tmp_map[obstacle_y as usize][obstacle_x as usize] = MapItem::Obstacle;

        if is_looping(initial_guard, &tmp_map) {
            result.insert(obstacle.1);
        }

        obstacle = step(obstacle, map);
    }

    result.len() as i32
}

fn is_looping(guard: (Direction, (i32, i32)), map: &Vec<Vec<MapItem>>) -> bool {
    let mut history: hash_set::HashSet<(Direction, (i32, i32))> = HashSet::new();

    let mut guard_current = guard;

    loop {
        if !is_on_map(guard_current.1 .0, guard_current.1 .1, map) {
            return false;
        }

        if history.contains(&guard_current) {
            return true;
        }

        history.insert(guard_current);
        guard_current = step(guard_current, map);
    }
}

fn step(guard: (Direction, (i32, i32)), map: &Vec<Vec<MapItem>>) -> (Direction, (i32, i32)) {
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
