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
            let mut report_variations = generate_reports_with_dampener(report);
            report_variations.insert(0, report.to_vec()); // Also check original report

            report_variations
                .iter()
                .any(|report| is_report_safe(report))
        })
        .count();

    println!("{}", result);
    Ok(())
}

fn is_report_safe(report: &Vec<i32>) -> bool {
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
}

fn generate_reports_with_dampener(original: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for i in 0..original.len() {
        let mut tmp = original.clone();
        tmp.remove(i);

        result.push(tmp);
    }

    result
}
