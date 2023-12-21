use std::{fs, collections::{HashMap, HashSet}, iter};
use itertools::Itertools;
use nom::{IResult, sequence::{separated_pair, delimited}, multi::{separated_list1, many1, many_m_n}, character::complete::{line_ending, space1, u8, char, not_line_ending}, bytes::complete::{tag, take}, error::Error, branch::alt, combinator::map};

fn main() {
    let file_path = "input.txt";
    // eprintln!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part(contents.trim());
}


fn part(input:&str) {
    let (_, puzzles) = separated_list1(many_m_n(2,2, line_ending::<&str, ()>), separated_list1(line_ending, many1(alt((char('#'), char('.'))))))(input).unwrap();

    // println!("{:?}", puzzles);
    let mut sum_v = 0usize;
    let mut sum_h = 0usize;

    for puzzle in puzzles {
        let mut v = find_mirror_v(puzzle.clone());
        let mut h = find_mirror_h(puzzle.clone());


        // let mut vv = find_smudged_mirror_v(puzzle.clone(), v);

        // if h == 1 || v == 1 {
            // println!("{h} - {v}");
            // for line in puzzle.clone() {
            //     println!("{}", line.iter().collect::<String>());
            // }
            
            // if (h as i32 - (puzzle.len()/2) as i32).abs() > (v as i32 - (puzzle[0].len()/2) as i32).abs() {
            //     h = 0;
            // } else {
            //     v = 0;
            // }
        // }
        println!("{}", h+v);
        sum_v += v;
        sum_h += h;
        
    }

    println!("{}", sum_v + sum_h*100)
//    part1(puzzles);

}

fn find_mirror_h(board:Vec<Vec<char>>) -> usize {
    let board = transpose(board);
    return find_mirror_v(board);

}

fn find_smudged_mirror_v(board:Vec<Vec<char>>,  ) {

}

fn find_mirror_v(board:Vec<Vec<char>>) -> usize {
    let mut all_mirrors = find_mirrors(board[0].clone(), 1, (board[0].len()-1) as usize);
    // println!("{}", board.len());
    for line_no in 1..board.len() {
        // println!("{:?}", mirrors);
        if all_mirrors.len() == 0 {
            return 0;
        }
        let mirrors = find_mirrors(board[line_no].clone(), all_mirrors[0], all_mirrors[all_mirrors.len()-1]);

        all_mirrors.retain(|m| mirrors.iter().find(|&e| e == m).is_some());
        // println!("{line_no} {} {:?}", &board[line_no].iter().collect::<String>(), possible_mirrors);
        
    }
    // println!("{:?}", mirrors);
    if all_mirrors.len() == 0 {
        return 0;
    } else if all_mirrors.len() > 1 {
        panic!("more than one mirror");
    }
    return all_mirrors.into_iter().next().unwrap();
}

fn find_mirrors(c:Vec<char>, start:usize, end: usize) -> Vec<usize> {
    let res = (start..=end).filter(|&i| {
        let len = i.min(c.len()-i);
        let right = c[i..].iter().take(len);
        let left = c[0..i].iter().rev().take(len);
        Iterator::eq(left, right)
    }).collect_vec();
    return res;
}


fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}