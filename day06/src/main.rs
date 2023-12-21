use std::{fs, iter::zip};
use regex::Regex; 

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
    let seperator = Regex::new(r"([ ]+)").expect("Invalid regex");
    let times:Vec<_> = seperator.split(lines[0].split(":").last().unwrap().trim()).map(|x| x.parse::<i32>().unwrap()).collect();
    let scores:Vec<_> = seperator.split(lines[1].split(":").last().unwrap().trim()).map(|x| x.parse::<i32>().unwrap()).collect();
    let games = zip(times, scores);
    let mut factor = 1;
    for (time, score) in games {
        let delta = ((time*time - 4*score) as f64).sqrt();
        let right = (time as f64 + delta)/2f64;
        let left = (time as f64 - delta)/2f64;

        let mut result:i32 = 0;
        if (right - right.floor()).abs() < 10f64*f64::EPSILON {
            result -= 1;
        }
        if (left - left.ceil()).abs() < 10f64*f64::EPSILON {
            result -= 1;
        }

        result += (right.floor() - left.ceil() + 1f64) as i32;
        factor *= result;
        println!("{result}");
    }
    println!("{factor}");

    
}

fn part2(lines:Vec<&str>) {
    let time = lines[0].split(":").last().unwrap().trim().chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<f64>().unwrap();
    let score = lines[1].split(":").last().unwrap().trim().chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<f64>().unwrap();

    let delta = (time*time - 4f64*score).sqrt();
    let right = (time + delta)/2f64;
    let left = (time - delta)/2f64;

    let mut result:f64 = 0f64;
    if (right - right.floor()).abs() < 10f64*f64::EPSILON {
        result -= 1f64;
    }
    if (left - left.ceil()).abs() < 10f64*f64::EPSILON {
        result -= 1f64;
    }

    result += right.floor() - left.ceil() + 1f64;

    println!("{result}");

}
