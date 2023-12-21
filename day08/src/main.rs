use std::{fs, collections::HashMap};
use itertools::Itertools;
use nom::{self, IResult, sequence::{separated_pair, delimited}, multi::separated_list1, character::complete::line_ending, bytes::complete::{tag, take}, error::Error};
use num::integer::lcm;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1_2(contents.as_str());
    // part2(contents);
}

fn parse_id(input:&str) -> IResult<&str, &str, Error<&str>> {
    Ok(take(3usize)(input)?)
}

fn parse_node(input:&str) -> IResult<&str, (&str,Vec<&str>), Error<&str>> {
    Ok(separated_pair(parse_id, tag(" = "), delimited(tag("("),separated_list1(tag(", "), parse_id), tag(")")))(input).unwrap())
}

fn part1_2(input:&str) {
    let (seq, input) = input.split_once('\n').unwrap();
    let seq = seq.chars().map(|c| if c == 'L' { 0 } else { 1 } as usize).collect_vec();
    let (_, graph) = separated_list1(line_ending, parse_node)(input.trim()).unwrap();

    let graph:HashMap<&str, Vec<&str>> = graph.into_iter().collect();
    // println!("{:?}", graph);

    let mut current = "AAA";
    let mut choice = seq.iter().cycle();
    let mut count = 0;
    while current != "ZZZ" {
        current = graph[current][*choice.next().unwrap()];
        count += 1;
    }
    println!("{count}");
    
    let currents = graph.keys().cloned().filter(|k| k.ends_with("A")).collect_vec();
//    let mut choice = seq.iter().cycle();
    let mut counts:Vec<usize> = vec![0; currents.len()];
    for i in 0..currents.len() {
        let mut current = currents[i];
        let mut choice = seq.iter().cycle();
        counts[i] = 0;
        while !current.ends_with("Z") {
            current = graph[current][*choice.next().unwrap()];
            counts[i] += 1;
        }
    }
    // while !currents.iter().all(|c| c.ends_with("Z")) {
    //     let ch = *choice.next().unwrap();
    //     for cur in &mut currents {
    //         *cur = graph[*cur][ch];
    //     }
    //     count += 1;
    // }
    let lc = counts.into_iter().fold(1usize, |a, b| lcm(a, b));
    println!("{lc:?}");

    
}