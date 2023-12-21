use std::fs;

use itertools::Itertools;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines:Vec<_> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    part1(lines.clone());
    part2(lines);
    
}

fn part1(lines:Vec<&str>) {
    let mut hands:Vec<(usize,usize,usize, &str)> = vec![];

    for line in lines {
        let (cards, score) = line.split_once(" ").unwrap();
        let groups:Vec<_> = cards.chars().sorted().group_by(|c| *c).into_iter().map(|(_, g)| g.count()).sorted().rev().collect();
        
        let order = if groups[0] == 1 {
            0   // nothing
        } else if groups[0] == 2 {
            if groups[1] == 1 {
                1 // 1 pair
            } else {
                2 // 2 pairs
            }
            
        } else if groups[0] == 3 {
            if groups[1] == 1 {
                3 // three
            } else {
                4 // full
            }
        } else if groups[0] == 4 {
            5 // four of a kind
        } else if groups[0] == 5 {
            6 // five of a kind
        } else { panic!("impossibru") };

        let mut suborder = 0;
        for c in cards.chars() {
            let v = "23456789TJQKA".find(c).unwrap();
            suborder *= 16;
            suborder += v;
        }

        hands.push((order, suborder, score.parse::<usize>().unwrap(), cards));



    }
    hands.sort();
    let mut sum = 0;
    for (rank, (_, _, score, _)) in hands.iter().enumerate() {
        sum += (rank+1) * score;
    }
    println!("{:?}", sum);


}


fn part2(lines:Vec<&str>) {
    let mut hands:Vec<(usize,usize,usize, &str)> = vec![];

    for line in lines {
        let (cards, score) = line.split_once(" ").unwrap();
        let groups:Vec<_> = cards.chars().filter(|c| *c != 'J').sorted().group_by(|c| *c).into_iter().map(|(_, g)| g.count()).sorted().rev().collect();
        let jokers = cards.chars().filter(|c| *c == 'J').count();
        
        let order = if jokers == 5 {
            6 // 5 jokers and nothing else
        } else if groups[0]+jokers == 1 {
            0   // nothing
        } else if groups[0]+jokers == 2 {
            if groups[1] == 1 {
                1 // 1 pair
            } else {
                2 // 2 pairs
            }
            
        } else if groups[0]+jokers == 3 {
            if groups[1] == 1 {
                3 // three
            } else {
                4 // full
            }
        } else if groups[0]+jokers == 4 {
            5 // four of a kind
        } else if groups[0]+jokers == 5 {
            6 // five of a kind
        } else { panic!("impossibru") };

        let mut suborder = 0;
        for c in cards.chars() {
            let v = "J23456789TQKA".find(c).unwrap();
            suborder *= 16;
            suborder += v;
        }

        hands.push((order, suborder, score.parse::<usize>().unwrap(), cards));



    }
    hands.sort();
    let mut sum = 0;
    for (rank, (_, _, score, _)) in hands.iter().enumerate() {
        sum += (rank+1) * score;
    }
    println!("{:?}", sum);


}