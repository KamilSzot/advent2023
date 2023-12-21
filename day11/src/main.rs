use std::{fs, collections::HashSet};
use itertools::Itertools;



fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let board = Board(contents.split("\n").filter(|s| !s.is_empty()).map(|line| line.chars().collect_vec()).collect_vec());

    part(2, &board);
    part(1000000, &board);
    
}

fn part(factor:i128, board:&Board) {
    let mut galaxies: Vec<Location> = Vec::default();
    let mut xs: HashSet<i128> = Default::default();
    let mut ys: HashSet<i128> = Default::default();

    for y in 0..board.height() {
        for x in 0..board.width() {
            let here = Location { x, y };
            match board.get(here) {
                '#' => {
                    galaxies.push(here);
                    xs.insert(x);
                    ys.insert(y);
                },
                '.' => {
                
                },
                _ => panic!("unknown char")
            }
        }
    }

    let exp_xs = (0..board.width()).filter(|&x| !xs.contains(&x)).collect_vec();
    let exp_ys = (0..board.height()).filter(|&y| !ys.contains(&y)).collect_vec();

    
    for g in &mut galaxies {
        g.x += (factor-1)*exp_xs.iter().filter(|&&x| x<g.x).count() as i128;
        g.y += (factor-1)*exp_ys.iter().filter(|&&y| y<g.y).count() as i128;
    }
    
    // g.x += board.width() - xs.len() as i128;
    // g.y += board.height() - ys.len() as i128;

    let mut distance = 0;
    for i in 0..galaxies.len()-1 {
        for j in i+1..galaxies.len() {
            distance += (galaxies[i].x - galaxies[j].x).abs() + (galaxies[i].y - galaxies[j].y).abs();
            // println!("{} {:?} {:?}", (galaxies[i].x - galaxies[j].x).abs() + (galaxies[i].y - galaxies[j].y).abs(), galaxies[i], galaxies[j]);
        }
    }

    println!("{distance}");
}

struct Board(Vec<Vec<char>>);
impl Board {
    fn get(&self, l:Location) -> char {
        if !self.location_valid(l) {
            panic!("Out of bounds access");
        }
        let Location { x, y } = l;
        return self.0[y as usize][x as usize];
    }
    fn height(&self) -> i128 {
        self.0.len() as i128
    }
    fn width(&self) -> i128 {
        self.0[0].len() as i128
    }

    fn location_valid(&self, l: Location) -> bool {
        let Location { x, y } = l;
        !(y<0 || y>=self.height() || x<0 || x>=self.width())
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Location {
    x: i128,
    y: i128
}