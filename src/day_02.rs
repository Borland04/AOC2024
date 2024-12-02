use std::io::Result;

const SAFETY_THRESHOLD: i32 = 3;

pub fn run(input: &mut dyn Iterator<Item = String>) -> Result<()> {
    let records: Vec<Vec<i32>> = input
        .map(|line| {
            line.split(" ")
                .map(|num| {
                    num.parse::<i32>()
                        .expect(format!("Wrong number format: {num}").as_str())
                })
                .collect()
        })
        .collect();

    let result = records
        .iter()
        .filter(|report| {
            if report.len() <= 1 {
                true
            } else {
                let is_increasing = report[0] < report[1];
                for i in 0..(report.len() - 1) {
                    let diff = report[i] - report[i + 1];

                    if diff == 0 || diff.abs() > SAFETY_THRESHOLD {
                        return false;
                    }

                    if (is_increasing && diff > 0) || (!is_increasing && diff < 0) {
                        return false;
                    }
                }

                true
            }
        })
        .count();

    println!("{}", result);
    Ok(())
}
