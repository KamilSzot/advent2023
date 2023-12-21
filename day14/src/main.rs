use std::fs;
use itertools::Itertools;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines:Vec<_> = contents.split("\n").filter(|s| !s.is_empty()).map(|s| s.chars().collect::<Vec<_>>()).collect();

    println!("part 1: {}", part1(lines.clone()));
    println!("part 2: {}", part2(lines.clone()));

}

fn part1(mut board:Vec<Vec<char>>) -> usize {
    tilt_north(&mut board);
    calculate_weight(&board)
}

fn part2(mut board:Vec<Vec<char>>) -> usize {

    let mut values:Vec<usize> = Default::default();
    for _ in 0..300 {
        spin(&mut board);
        values.push(calculate_weight(&board));
    }

    let minimum = values[values.len()-100..].iter().min().unwrap();
    let (last, previous) = values.iter().enumerate().filter(|(_, v)| *v == minimum).map(|(i, _)| i).rev().take(2).next_tuple().unwrap();
    // for line in &board {
    //     println!("{}", line.iter().collect::<String>());
    // }

    let target = 1000000000usize;
    let cycles = (target - last)/(last-previous);
    let ultimate_last = cycles*(last-previous)+last;

    let result = values[previous+target-ultimate_last-1];


    // print!("{}, {}, {}, {}", minimum, previous, last, result );
    result
    
}

fn calculate_weight(board:&Vec<Vec<char>>) -> usize {
    let mut total_weight = 0usize;
    let mut weight = board.len();
    for line in board {
        total_weight += line.iter().filter(|&&c| c == 'O').count()*weight;
        weight -= 1;
    }
    total_weight
}

fn tilt_north(board:&mut Vec<Vec<char>>) {
    for y in 1..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == 'O' {
                let mut fy = y;
                while fy > 0 && board[fy-1][x] == '.' {
                    fy -= 1;
                }
                if fy < y {
                    board[fy][x] = 'O';
                    board[y][x] = '.';
                }
            }
        }
    }
}

fn tilt_south(board:&mut Vec<Vec<char>>) {
    for y in (0..board.len()-1).rev() {
        for x in 0..board[y].len() {
            if board[y][x] == 'O' {
                let mut fy = y;
                while fy < board.len()-1 && board[fy+1][x] == '.' {
                    fy += 1;
                }
                if fy > y {
                    board[fy][x] = 'O';
                    board[y][x] = '.';
                }
            }
        }
    }
}


fn tilt_west(board:&mut Vec<Vec<char>>) {
    for x in 1..board[0].len() {
        for y in 0..board.len() {
            if board[y][x] == 'O' {
                let mut fx = x;
                while fx > 0 && board[y][fx-1] == '.' {
                    fx -= 1;
                }
                if fx < x {
                    board[y][fx] = 'O';
                    board[y][x] = '.';
                }
            }
        }
    }
}

fn tilt_east(board:&mut Vec<Vec<char>>) {
    for x in (0..board[0].len()-1).rev() {
        for y in 0..board.len() {
            if board[y][x] == 'O' {
                let mut fx = x;
                while fx < board.len()-1 && board[y][fx+1] == '.' {
                    fx += 1;
                }
                if fx > x {
                    board[y][fx] = 'O';
                    board[y][x] = '.';
                }
            }
        }
    }
}

fn spin(board:&mut Vec<Vec<char>>) {
    tilt_north(board);
    tilt_west(board);
    tilt_south(board);
    tilt_east(board);
}
