use regex::Regex;
use std::{io, ops::Range};

pub fn run(input: &mut dyn Iterator<Item = String>) -> io::Result<()> {
    let mul_regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("Invalid regex");
    let do_regex = Regex::new(r"do()").expect("Invalid regex");
    let dont_regex = Regex::new(r"don't()").expect("Invalid regex");

    let (_, result) = input.fold((true, 0), |(current_enabled, acc), input_str| {
        let mut dos: Vec<StackItem> = do_regex
            .captures_iter(input_str.as_ref())
            .map(|capt| StackItem::Do(capt.get(0).unwrap().start()))
            .collect();
        let mut donts: Vec<StackItem> = dont_regex
            .captures_iter(input_str.as_ref())
            .map(|capt| StackItem::Dont(capt.get(0).unwrap().start()))
            .collect();
        let mut muls: Vec<StackItem> = mul_regex
            .captures_iter(input_str.as_ref())
            .map(|capt| {
                let l = capt[1].parse::<i32>().unwrap();
                let r = capt[2].parse::<i32>().unwrap();
                StackItem::Mul(capt.get(0).unwrap().start(), l as u64, r as u64)
            })
            .collect();

        let mut stack: Vec<StackItem> = Vec::new();
        stack.append(&mut dos);
        stack.append(&mut donts);
        stack.append(&mut muls);

        stack.sort_by(|a, b| usize::cmp(&a.pos(), &b.pos()));

        let mut enabled = current_enabled;
        let mut result: u64 = 0;

        for i in stack {
            match i {
                StackItem::Do(_) => enabled = true,
                StackItem::Dont(_) => enabled = false,
                StackItem::Mul(_, l, r) if enabled => result += l * r,
                _ => {}
            }
        }

        (enabled, acc + result)
    });

    println!("{}", result);

    Ok(())
}

enum StackItem {
    Do(usize),
    Dont(usize),
    Mul(usize, u64, u64),
}

impl StackItem {
    pub fn pos(self: &Self) -> usize {
        match self {
            StackItem::Do(pos) => *pos,
            StackItem::Dont(pos) => *pos,
            StackItem::Mul(pos, _, _) => *pos,
        }
    }
}
