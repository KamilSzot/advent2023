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
    let mut sum_vv = 0usize;
    let mut sum_hh = 0usize;

    for puzzle in puzzles {
        let mut v = find_mirror_v(puzzle.clone(), 0);
        let mut h = find_mirror_h(puzzle.clone(), 0);


        // let mut vv = find_mirror_v(puzzle.clone(), v);
        // let mut hh = find_mirror_h(puzzle.clone(), v);

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
        // println!("{h} - {v}");
        sum_v += v;
        sum_h += h;
        // sum_vv += vv;
        // sum_hh += hh;
        
    }

    println!("{}", sum_v + sum_h*100);
    println!("{}", sum_vv + sum_hh*100);
//    part1(puzzles);

}

fn find_mirror_h(board:Vec<Vec<char>>, exclude:usize) -> usize {
    let board = transpose(board);
    return find_mirror_v(board, exclude);

}

fn find_mirror_v(board:Vec<Vec<char>>, exclude:usize) -> usize {
    let mut all_mirrors = vec![];
    // let mut all_mirrors = vec![find_mirrors(board[0].clone(), 1, (board[0].len()-1) as usize, exclude > 0)];
    // all_mirrors[0].retain(|&e| e.0 != exclude);
    // println!("{}", board.len());
    for line_no in 0..board.len() {
        // println!("{:?}", mirrors);
        let mut mirrors = find_mirrors(board[line_no].clone(), 1,  (board[0].len()-1) as usize, exclude > 0);
        mirrors.retain(|&e| e.0 != exclude);
        all_mirrors.push(mirrors);

        //all_mirrors.retain(|m| mirrors.iter().find(|&e| e == m).is_some());
        // println!("{line_no} {} {:?}", &board[line_no].iter().collect::<String>(), possible_mirrors);
        
    }
    let possible_mirrors = all_mirrors.iter().flatten(). map(|&e| e.0).sorted().unique().collect_vec();
    
    // if exclude > 0 { 
    //     for m in all_mirrors.clone() {
    //         println!("{:?}", m);
    //     }
    // }
    println!("{:?}", possible_mirrors.clone());
    for i in possible_mirrors {
        let column:Vec<(usize, bool)> = all_mirrors.iter().map(|e| e.iter().cloned().filter(|(index, _)| *index == i)).flatten().collect_vec();
        // if i == 8 {
        //     println!("{:?} => {} {exclude}", column.clone(), column.iter().filter(|(_, exact)| !exact).count());
        // }
        if exclude>0 {
            if column.iter().filter(|(_, exact)| !exact).count() == 1 {
                return i;
            } 
        } else {
            if column.iter().filter(|(_, exact)| !exact).count() == 0 {
                println!("{i}");
                return i;
            } 
        }        
    }
    return 0;
    // // println!("{:?}", mirrors);
    // if all_mirrors.len() == 0 {
    //     return 0;
    // } else if all_mirrors.len() > 1 {
    //     println!("{:?}", all_mirrors);
    //     panic!("more than one mirror");
    // }
    // return all_mirrors.into_iter().next().unwrap();
}

fn find_mirrors(c:Vec<char>, start:usize, end: usize, smudged: bool) -> Vec<(usize, bool)> {
    (start..=end).filter_map(|i| {
        let len = i.min(c.len()-i);
        let right = c[i..].iter().take(len);
        let left = c[0..i].iter().rev().take(len);
        if !eq_iterators_with_1_error_tolerance(left.clone(), right.clone()) {
            None
        } else {
            if Iterator::eq(left, right) {
                Some((i, true))
            } else {
                Some((i, false))
            }
        }
    })
    .collect_vec()
}

fn eq_iterators_with_1_error_tolerance<T: Eq>(mut a:impl Iterator<Item = T>, mut b:impl Iterator<Item = T>) -> bool {
    let mut error_count = 0;
    while let Some(aa) = a.next() {
        if let Some(bb) = b.next() {
            if aa != bb {
                if error_count > 0 {
                    return false;
                } else {
                    error_count += 1;
                }
            }
        } else {
            return false;
        }
    }
    return b.next().is_none();
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