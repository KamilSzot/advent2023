use std::fs;
use itertools::Itertools;
// use nom::{self, sequence::{pair, preceded}, branch::alt, character::complete::{alpha1, char, i32}, combinator::{map, value}};

fn main() {
    use Direction::*;

    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let sides1 = contents.split("\n").filter(|s| !s.is_empty()).map(|line| line.split(' ').next_tuple().unwrap()).map(|(dir, len, _)| Side::new(dir, len)).collect_vec();
    let sides2 = contents.split("\n").filter(|s| !s.is_empty()).map(|line| line.split(' ').next_tuple().unwrap()).map(|(_, _, col)| {
        let col = &col[2..col.len()-1];
        let dir = match col.chars().last().unwrap() {
            '0' => Right,
            '1' => Down,
            '2' => Left,
            '3' => Up,
            _ => panic!("wrong last hex digit")
        };
        Side { direction: dir, length: i128::from_str_radix(&col[..col.len()-1], 16).unwrap() }
}).collect_vec();

println!("{}", calculate_area(sides1));
println!("{}", calculate_area(sides2));


}

fn calculate_area(sides:Vec<Side>) -> i128 {
    use Direction::*;

    let mut circumference = 0i128;
    let mut area = 0i128;
    let mut y = 1i128;

    for side in &sides {
        circumference += side.length;

        match side.direction {
            Right => area -= 2 * side.length * y,
            Left =>  area += 2 * side.length * y,
            Up => y -= side.length,
            Down => y += side.length
        }
    }

    area /= 2;
    area = area.abs();
    area += 1 + circumference / 2;



    area
}

#[derive(Debug)]
struct Side {
    direction: Direction,
    length: i128
}

impl Side {
    fn new(dir:&str, len:&str) -> Side {
        Side {
            direction: Direction::new(dir),
            length: len.parse::<i128>().unwrap()
        }
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}

impl Direction {
    fn new(s:&str) -> Direction {
        use Direction::*;
        match s {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("wrong char {s}")
        }
    }
}