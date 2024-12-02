use std::io::Result;

pub fn run(input: &mut dyn Iterator<Item = String>) -> Result<()> {
    let (mut left_list, mut right_list) = input
        .map(|l| {
            l.split(" ")
                .filter(|sub_l| !sub_l.is_empty())
                .map(|sub_l| {
                    sub_l
                        .parse::<i32>()
                        .expect(format!("Value '{sub_l}' should be a number on line ").as_str())
                })
                .collect::<Vec<i32>>()
        })
        .fold(
            (Vec::<i32>::new(), Vec::<i32>::new()),
            |(mut l, mut r), is| {
                l.push(is[0]);
                r.push(is[1]);
                (l, r)
            },
        );

    let result = left_list
        .iter()
        .map(|i| i * (calculate_occurrences(i, &right_list) as i32))
        .reduce(|acc, i| acc + i)
        .unwrap_or(0);

    println!("{}", result);

    Ok(())
}

fn calculate_occurrences<T: PartialEq>(item: &T, list: &Vec<T>) -> usize {
    list.iter().filter(|i| item.eq(i)).count()
}
