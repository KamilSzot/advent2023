use std::fs;
use itertools::Itertools;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines:Vec<_> = contents.split("\n").filter(|s| !s.is_empty()).collect();

    part(lines);
}

fn part(lines:Vec<&str>)  {
    // all sequences reversed
    let sequences:Vec<Vec<i128>> = lines.into_iter().map(|line| line.split(" ").map(|n| n.parse::<i128>().unwrap()).collect_vec()).collect_vec();
    println!("part 1: {}", sequences.clone().into_iter().map(|v| next_element(v.into_iter().rev())).sum::<i128>());
    println!("part 2: {}", sequences.clone().into_iter().map(|v| next_element(v.into_iter())).sum::<i128>());    
    // deep(10, [1].into_iter());
}

fn next_element<T: Iterator<Item = i128> + Clone>(sequnce: T) -> i128 
{   
    let first = sequnce.clone().next().unwrap();

    let result = if sequnce.clone().all(|el| el == first) {
        first
    } else {
        first + next_element(sequnce.tuple_windows::<(i128, i128)>().map(|(f, s)| f-s).collect_vec().iter().cloned())
    };
    result
}

// trait CloneableIterator<I>: Iterator<Item = I::Item> + Clone where 
//     I: Iterator+ Clone,
//     I::Item : Clone
// {
//     fn iter_clonable() -> Self;
// }

// impl<T: Clone, I: Iterator<Item = T> + Clone> CloneableIterator<I> for <Vec<T> as IntoIterator>::IntoIter {
//     fn iter_clonable() -> Box<dyn CloneableIterator<I>> {
        
//     }
// }

// fn deep(cnt:i32, q:impl Iterator<Item = i32>) {
//     if cnt == 0 { return; }
//     deep(cnt-1, q.map(|a| a));
// }
