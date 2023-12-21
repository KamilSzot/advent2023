use std::fs;
use nom::{self, sequence::{pair, preceded}, branch::alt, character::complete::{alpha1, char, i32}, combinator::{map, value}};

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = contents.split("\n").filter(|s| !s.is_empty()).map(|line| line.split(",").collect::<Vec<&str>>()).flatten().collect::<Vec<_>>();

    println!("part 1: {}", part1(lines.clone()));
//    println!("part 2: {}", part2(lines));
    println!("part 2: {}", part2(lines.clone()));
}

#[derive(Clone, Debug)]
enum Op {
    Add(i32),
    Delete
}

fn part2(sequence:Vec<&str>) -> i32 {
    let mut boxes: Vec<Vec<(&str, i32)>> = vec![Default::default(); 256];
    for step in sequence {
        let (_, (label, operation)) = pair(alpha1::<&str, ()>, alt((map(preceded(char('='), i32), |f| Op::Add(f)), value(Op::Delete, char('-')))))(step).unwrap();

        // println!("{label} {operation:?}");

        let i = hash(label);
        let b = &mut boxes[i];

        match operation {
            Op::Add(focal) => {
                if let Some(found) = b.iter_mut().find(|(l, _)| *l == label) {
                    found.1 = focal;
                } else {
                    b.push((label, focal));
                }
            },
            Op::Delete => {
                b.retain(|(l, _)| *l != label);
            }
        }
        

    }

    calculate_focusing_power(boxes)
}

fn calculate_focusing_power(boxes:Vec<Vec<(&str, i32)>>) -> i32 {
    let mut sum = 0;
    for (box_no, b) in boxes.iter().enumerate() {
        for (slot_no, &(_, f)) in b.iter().enumerate() {
            sum += (box_no as i32 + 1)*(slot_no as i32 + 1)*f;
        }
    }
    sum
}

fn part1(sequence:Vec<&str>) -> usize {
    sequence.iter().map(|el| hash(el)).sum()
}

fn hash(v:&str) -> usize {
    let mut h: usize = 0;
    for c in v.bytes() {
        h += c as usize;
        h = (h*17) % 256;
    }
    return h;
}