use std::io;

const XMAS: &str = "MAS";
const XMAS_REVERSED: &str = "SAM";

pub fn run(input: &mut dyn Iterator<Item = String>) -> io::Result<()> {
    let input_matrix: Vec<Vec<char>> = input.map(|str| str.chars().collect()).collect();

    println!("{}", calculate(&input_matrix));

    Ok(())
}

fn calculate(input_matrix: &Vec<Vec<char>>) -> u32 {
    let w = input_matrix[0].len();
    let h = input_matrix.len();
    let x_mas_shift = XMAS.len() as i32 - 1;

    let mut result = 0;
    for y in 0..h as i32 {
        for x in 0..w as i32 {
            if (check_occurrence(x, y, 1, 1, input_matrix, XMAS)
                || check_occurrence(x, y, 1, 1, input_matrix, XMAS_REVERSED))
                && (check_occurrence(x + x_mas_shift, y, -1, 1, input_matrix, XMAS)
                    || check_occurrence(x + x_mas_shift, y, -1, 1, input_matrix, XMAS_REVERSED))
            {
                result += 1;
            }
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
