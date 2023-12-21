use std::{fs, collections::HashMap};
use nom::{self, IResult, multi::separated_list1, character::complete::{i32, space0, space1} , sequence::{preceded, delimited,  separated_pair}, bytes::complete::tag, branch::alt};

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = contents.split("\n").filter(|s| !s.is_empty());

    println!("part 1: {}", part(1, lines.clone()));
    println!("part 2: {}", part(2, lines));
}

fn part<'a>(no:u32, lines: impl Iterator<Item = &'a str>) -> i32 {
    use Color::*;
    let bounds:HashMap<Color, i32> = [(Red, 12), (Green, 13), (Blue, 14)].into_iter().collect();

    let mut id_sum = 0;
    let mut power_sum = 0;
    for line in lines {
//        println!("{}", line);
        let mut within_bounds = true;
        let (_, game) = game(line).unwrap();

        let mut mins:HashMap<Color, i32> = [(Red, 0), (Green, 0), (Blue, 0)].into_iter().collect();
        for set in game.sets {
            for (color, amount) in set {
                if amount > *bounds.get(&color).unwrap() {
                    within_bounds = false;
                }
                if amount > *mins.get(&color).unwrap() {
                    mins.insert(color, amount);
                }
            }
        }
        if within_bounds {
            id_sum += game.number;
        }
        power_sum += mins.get(&Red).unwrap()*mins.get(&Green).unwrap()*mins.get(&Blue).unwrap();

    }
    return if no == 1 { 
        id_sum 
    } else { 
         power_sum
    };
}


#[derive(Debug)]
struct Game {
    number: i32,
    sets: Vec<Set>
}

type Set = HashMap<Color, i32>;

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
enum Color {
    Red, Green, Blue
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "red" => Self::Red,
            "blue" => Self::Blue,
            "green" => Self::Green,
            _ => panic!("Unknown color")
        }
    }
}



fn game(input:&str) -> IResult<&str, Game> {
    let (input, number) = delimited(tag("Game "), i32, tag(":"))(input)?;
    let (input, sets) = separated_list1(tag(";"), set)(input)?;

    return Ok((input, Game { number, sets:sets }));
}

fn set(input:&str) -> IResult<&str, Set> {
    let mut set = Set::new();
    let (input, items) = separated_list1(tag(","), preceded(space0, separated_pair(i32, space1, alt((tag("red"), tag("green"), tag("blue"))))))(input)?;
    for (amount, color) in items {
        set.insert(color.into(), amount);
    }
    return Ok((input, set))
}
