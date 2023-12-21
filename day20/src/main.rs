use std::fs;
use nom::Err;
use itertools::Itertools;

fn main() {

    let file_path = "test01.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = contents.split("\n").filter(|s| !s.is_empty()).map(|line| parse_element(line)).collect_vec();
    

}

fn parse_element(input:&str) -> Result<(String, Element), Err<()>> {

}

struct Element {
    kind: ElementKind,
    outputs: Vec<String>
}

enum ElementKind {
    Broadcaster,
    FlipFlop(bool),
    Conjunction
}