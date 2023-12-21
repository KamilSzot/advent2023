use std::{fs, collections::HashMap, iter};
use itertools::Itertools;
use nom::{self, sequence::separated_pair, multi::{separated_list1, many1}, character::complete::{line_ending, space1, u8, char}, bytes::complete::tag, branch::alt};

fn main() {
    let file_path = "input.txt";
    // eprintln!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part(contents.trim());
}


fn part(input:&str) {
    let (_, puzzles) = separated_list1(line_ending::<&str, ()>, separated_pair(many1(alt((char('.'), char('#'), char('?')))), space1, separated_list1(tag(","), u8)))(input).unwrap();
//    part1(puzzles);
    part1(puzzles.clone());
    part2(puzzles);
}


fn part1(puzzles: Vec<(Vec<char>, Vec<u8>)>) {
    let mut sum = 0;
    for (board, groups) in puzzles {
        let puzzle = Puzzle { board: board.as_slice(), groups: groups.as_slice(), anchored: false };
        let mut memo:HashMap<PuzzleKey, i128> = Default::default();
        sum += puzzle.count_solutions(&mut memo);
//        println!("{}, {puzzle:?}", );
    }
    println!("{sum}");
}

#[allow(unstable_name_collisions)] // intersperse
fn part2(puzzles: Vec<(Vec<char>, Vec<u8>)>) {
    let mut sum = 0;
    for (board, groups) in puzzles {
        let groups = groups.repeat(5);
        let board = iter::repeat(board).take(5).intersperse(vec!['?']).flatten().collect_vec();
        let puzzle = Puzzle { board: board.as_slice(), groups: groups.as_slice(), anchored: false };
        let mut memo:HashMap<PuzzleKey, i128> = Default::default();

        // println!("{puzzle:?}", );
        sum += puzzle.count_solutions(&mut memo);
    }
    println!("{sum}");
}

#[derive(Debug)]
struct Puzzle<'a> {
    board: &'a [char],
    groups: &'a [u8],
    anchored: bool
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PuzzleKey(String, Vec<u8>, bool);

impl<'a> Puzzle<'a> {
    fn key(&self) -> PuzzleKey {
        PuzzleKey(self.board.iter().collect(), self.groups.iter().cloned().collect_vec(), self.anchored)
    }
}

impl<'a> Puzzle<'a> {
    fn count_solutions(&self, memo:&mut HashMap<PuzzleKey, i128>) -> i128 {
        if let Some(&count) = memo.get(&self.key()) {
            return count;
        }
        
        // eprintln!("{:?}", &self);
        let &Puzzle { board, groups, anchored } = self;
        let result = if board.len() == 0 {
            if groups.len() == 0 {
                // eprintln!("Solution!!!");
                1
            } else {
                // eprintln!("Soem groups left at the end");
                0
            }
        } else {
            match board[0] {
                '.' =>  if anchored && groups.len() > 0 {
                            // eprintln!("Can start from empty because the first group is a continuation");
                            0
                        } else {
                            Puzzle { board: &board[1..], groups, anchored: false }.count_solutions(memo)
                        },
                '#' =>  if groups.len() == 0 {
                            // eprintln!("Not possible to place group");
                            0
                        } else {
                            if groups[0] == 1 {
                                if board.len() == 1 {
                                    Puzzle { board: &board[1..], groups: &groups[1..], anchored: false }.count_solutions(memo)
                                } else {
                                    if board[1] == '#' { 
                                        // eprintln!("No space or end after 1 group.");
                                        0
                                    } else { // ? must be . 
                                        Puzzle { board: &board[2..], groups: &groups[1..], anchored: false }.count_solutions(memo)
                                    }
                                }
                            } else {
                                Puzzle { board: &board[1..], groups: [&[groups[0]-1], &groups[1..]].concat().as_slice(), anchored: true }.count_solutions(memo)
                            }
                        },
                '?' =>  Puzzle { board: [&['.'],&board[1..]].concat().as_slice(), groups, anchored }.count_solutions(memo) +
                        Puzzle { board: [&['#'],&board[1..]].concat().as_slice(), groups, anchored }.count_solutions(memo),
                _   => panic!("wrong character in puzzle")

            }
        };
        memo.insert(self.key(), result);
        return result;
    }
}