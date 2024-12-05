use std::io;

const XMAS: &str = "XMAS";

pub fn run(input: &mut dyn Iterator<Item = String>) -> io::Result<()> {
    let input_matrix: Vec<Vec<char>> = input.map(|str| str.chars().collect()).collect();

    println!("{}", calculate(&input_matrix));

    Ok(())
}

fn calculate(input_matrix: &Vec<Vec<char>>) -> u32 {
    let w = input_matrix[0].len();
    let h = input_matrix.len();

    let directions: Vec<(i32, i32)> = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let mut result = 0;
    for y in 0..h {
        for x in 0..w {
            result += directions
                .iter()
                .filter(|(dx, dy)| {
                    check_occurrence(x as i32, y as i32, *dx, *dy, input_matrix, XMAS)
                })
                .count();
        }
    }

    result as u32
}

fn check_occurrence(
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    input_matrix: &Vec<Vec<char>>,
    word_to_check: &str,
) -> bool {
    if word_to_check.is_empty() {
        return true;
    }

    if y < 0 || y >= input_matrix.len() as i32 {
        return false;
    }
    if x < 0 || x >= input_matrix[y as usize].len() as i32 {
        return false;
    }

    if input_matrix[y as usize][x as usize] != word_to_check.chars().nth(0).unwrap() {
        return false;
    }

    check_occurrence(x + dx, y + dy, dx, dy, input_matrix, &word_to_check[1..])
}
