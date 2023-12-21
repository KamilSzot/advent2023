use std::{self, collections::HashSet, fs};
use nom::{self, sequence::{separated_pair, delimited, preceded, pair}, character::complete::{multispace0, u32, multispace1}, bytes::complete::tag, multi::separated_list1, Err, error::Error};

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines:Vec<_> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    part1(lines.clone());
    // part2(lines);
}

#[derive(Debug)]
struct Card {
    number: usize,
    winning: HashSet<u32>,
    owned: HashSet<u32>
}

fn parse_card(input:&str) -> Card {
    match pair(delimited(tag("Card"), preceded(multispace1, u32), tag(":")),
        separated_pair(parse_numbers, tag("|"), parse_numbers))(input) {
            Ok((_, data)) => {
                return Card {
                    number: data.0 as usize, 
                    winning: HashSet::from_iter(data.1.0.into_iter()),
                    owned: HashSet::from_iter(data.1.1.into_iter())
                }
            },
            Err(input) => {
                panic!("Error {}", input);
            }
        }
    
}

fn parse_numbers(input:&str) -> Result<(&str, Vec<u32>), Err<Error<&str>>> {
    delimited(multispace0, separated_list1(multispace1, u32), multispace0)(input)
}

fn part1(lines:Vec<&str>) {
    let mut sum = 0;
    let mut counter:Vec<u32> = vec![1; lines.len()].into();

    for line in lines {
        let card = parse_card(line);

        let matches = card.owned.intersection(&card.winning).count();

        let score = if matches == 0 { 0 } else { 1u32 << (matches - 1) };
        sum += score;

        for i in card.number..(card.number+matches) {
            counter[i] += counter[card.number-1];
        }
        // println!("{:?} {}", card, score);
    }
    println!("{sum}");
    println!("{:?}", counter.into_iter().sum::<u32>());
}