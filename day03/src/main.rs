use std::fs;
use nom::character::complete::u32;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines:Vec<_> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    part1(lines.clone());
    part2(lines);

}

fn part2(lines:Vec<&str>) {
    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.char_indices() {
            if c == '*' {
                let mut numbers:Vec<u32> = vec![];

                get_numbers_adjecent(x, line, &mut numbers);


                if y>0 {
                    let prev_line = lines[(y as i32-1) as usize];
                    get_numbers_adjecent(x, prev_line, &mut numbers);
                }
                if y<lines.len()-1 {
                    let next_line = lines[y+1];
                    get_numbers_adjecent(x, next_line, &mut numbers);
                }

                if numbers.len() == 2 {
                    sum += numbers[0]*numbers[1];
                }
                println!("{:?}",numbers);
            }
        }
    }
    println!("{:?}",sum);
}

fn get_numbers_adjecent(x:usize, line:&str, result:&mut Vec<u32>) {
    let is_a_digit = |cc:&char| cc.is_digit(10);

    let center = line.chars().nth(x).unwrap();
    if !center.is_digit(10) {
        let right = line.chars().skip(x+1).take_while(is_a_digit).collect::<String>();
        let left = line.chars().take(x).collect::<Vec<_>>().into_iter().rev().take_while(is_a_digit).collect::<Vec<_>>().into_iter().rev().collect::<String>();
        if !right.is_empty() {
            result.push(right.parse::<u32>().unwrap());
        }
        if !left.is_empty() {
            result.push(left.parse::<u32>().unwrap());
        }
        // println!("Top Left: {left}\nTop Right: {right}");
    } else {
        let whole = line.chars().take(x+1).collect::<Vec<_>>().into_iter().rev().take_while(is_a_digit).collect::<Vec<_>>().into_iter().rev().chain(
            line.chars().skip(x+1).take_while(is_a_digit)
        ).collect::<String>();
        if !whole.is_empty() {
            result.push(whole.parse::<u32>().unwrap());
        }
        // println!("Top: {top}");
    }

}

fn part1(lines:Vec<&str>) {
    let mut sum = 0;

    let mut skip_next = 0;
    for (y, line) in lines.iter().enumerate() {
        'next_char: for (x, char) in line.char_indices() {
            if skip_next > 0 {
                skip_next -= 1;
                continue;
            }
            if char.is_digit(10) {
                let (rest, number) = u32::<&str,()>(&line[x..]).unwrap();
                // println!("{} {}", number, rest);
                skip_next = line.len() - x - rest.len();

                for sx in 0.max(x as i32-1)..((line.len() - rest.len() + 1) as i32).min(line.len() as i32) {
                    for sy in 0.max(y as i32-1)..(line.len() as i32).min(y as i32+2) {
                        let c = lines[sy as usize].chars().nth(sx as usize).unwrap();
                        if !c.is_digit(10) && c!='.' {
                            sum += number;
                            // println!("{} hit", number);
                            continue 'next_char;
                        }
                    }
                }
            }
        }
    }
    println!("{sum}");
}
