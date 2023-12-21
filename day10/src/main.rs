use std::{fs, collections::{HashMap, HashSet}, ops::{Add, Sub}};
use itertools::Itertools;
use lazy_static::lazy_static;



fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let board = Board(contents.split("\n").filter(|s| !s.is_empty()).map(|line| line.chars().collect_vec()).collect_vec());

    let mut nodes:HashMap<Location, Vec<Location>> = HashMap::default();

    let mut start_location:Location = Location { x:0, y: 0 };

    for y in 0..board.height() {
        for x in 0..board.width() {
            let here = Location { x, y };
            if board.get(here) == 'S' {
                start_location = here;
            }
            for &o in &OFFSET[&board.get(here)] {
                let next_to = here + o;
                if board.location_valid(next_to) {
                    if OFFSET[&board.get(next_to)].iter().any(|&ofs| next_to + ofs == here) {
                        // there's conection between here and next_to
                        nodes.entry(here).or_default().push(next_to);
                    }
                }
                
            }
        }
    }
    // println!("{:?}", nodes);
    println!("{:?}", start_location);
    let mut visited:HashSet<Location> = HashSet::default();
    visited.insert(start_location);

    let mut max_length = 0;
    let mut area_for_max_length = 0;    
    for &begin in &nodes[&start_location] {
        // println!("{begin:?}");
        let mut cur = begin;
        let mut length = 0;
        let mut area = 0;

        // if board.get(begin) == '-' { 
            let dx = (begin-start_location).dx;
            if dx == -1 {
                area += 2*begin.y + 1;
            } else if dx == 1 {
                area -= 2*begin.y + 1;
            }
        // }
        // println!("first step: {area:?} {:?}", &board.get(begin));


        while length < 1 || cur != start_location {
            length += 1;
            visited.insert(cur);
            if let Some(&next) = &nodes[&cur].iter().find(|&l| !visited.contains(l)) {
                // if board.get(next) == '-' {
                    let dx = (next-cur).dx;
                    if dx == -1 {
                        area += 2*next.y + 1;
                    } else if dx == 1 {
                        area -= 2*next.y + 1;
                    }
                // }
                cur = next;
                // println!("{area:?}");
                continue;
            }
            break;
        }
        if length > max_length {
            max_length = length;
            area_for_max_length = area;
        }

    }
    area_for_max_length /= 2;
    area_for_max_length = area_for_max_length.abs();
    area_for_max_length -= (max_length+1)/2 - 1;
    println!("Length: {}", (max_length+1)/2);
    println!("Area: {}", area_for_max_length);
    
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
    fn height(&self) -> i32 {
        self.0.len() as i32
    }
    fn width(&self) -> i32 {
        self.0[0].len() as i32
    }

    fn location_valid(&self, l: Location) -> bool {
        let Location { x, y } = l;
        !(y<0 || y>=self.height() || x<0 || x>=self.width())
    }
}

#[derive(Copy, Clone)]
struct Offset {
    dx: i32,
    dy: i32
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Location {
    x: i32,
    y: i32
}

impl Add<Offset> for Location {
    type Output = Location;

    fn add(self, rhs: Offset) -> Self::Output {
        let x = self.x + rhs.dx;
        let y = self.y + rhs.dy;
        Location  { x, y }
    }
    
}

impl Sub<Location> for Location {
    type Output = Offset;

    fn sub(self, rhs: Location) -> Self::Output {
        Offset { dx: self.x - rhs.x, dy: self.y - rhs.y }
    }
    
}



lazy_static! {
    static ref OFFSET:HashMap<char, Vec<Offset>> = HashMap::from([
        ('-', vec![Offset { dx: -1, dy:  0 }, Offset { dx:  1, dy:  0 }, ]),
        ('|', vec![Offset { dx:  0, dy: -1 }, Offset { dx:  0, dy:  1 }, ]),
        ('L', vec![Offset { dx:  0, dy: -1 }, Offset { dx:  1, dy:  0 }, ]),
        ('F', vec![Offset { dx:  0, dy:  1 }, Offset { dx:  1, dy:  0 }, ]),
        ('7', vec![Offset { dx: -1, dy:  0 }, Offset { dx:  0, dy:  1 }, ]),
        ('J', vec![Offset { dx: -1, dy:  0 }, Offset { dx:  0, dy: -1 }, ]),
        ('.', vec![]),
        ('S', vec![Offset { dx: -1, dy:  0 }, Offset { dx:  1, dy:  0 }, Offset { dx:  0, dy: -1 }, Offset { dx:  0, dy:  1 }, ]),
    ]);
}