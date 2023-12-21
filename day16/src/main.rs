use std::{fs, ops::Add, collections::{HashMap, HashSet}, sync::atomic::AtomicI32};
// use nom::{self, sequence::{pair, preceded}, branch::alt, character::complete::{alpha1, char, i32}, combinator::{map, value}};

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = contents.split("\n").filter(|s| !s.is_empty()).map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    WIDTH.store(lines[0].len() as i32, std::sync::atomic::Ordering::Relaxed);
    HEIGHT.store(lines.len() as i32, std::sync::atomic::Ordering::Relaxed);

    use Obstacle::*;
    let mut obstacles: HashMap<Location, Obstacle> = Default::default();
    for (y, line) in lines.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if let Some(obstacle) = match c {
                '/' => Some(Slash),
                '\\' => Some(Backslash),
                '|' => Some(Vertical),
                '-' => Some(Horizontal),
                '.' => None,
                _ => panic!("wrong char {}", c)
            } {
                obstacles.insert(Location { x: x as i32, y: y as i32 }, obstacle);
            }
        }
    }


    println!("part 1: {}", part1(&obstacles));
    println!("part 2: {}", part2(&obstacles));
//    println!("part 2: {}", part2(lines));
    // println!("part 2: {}", part2(lines.clone()));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: i32,
    y: i32
}

impl Location {
    fn valid(&self) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < WIDTH.load(std::sync::atomic::Ordering::Relaxed) && self.y < HEIGHT.load(std::sync::atomic::Ordering::Relaxed)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Direction {
    dx: i32,
    dy: i32
}

impl Add<Direction> for Location {
    type Output = Location;

    fn add(self, rhs: Direction) -> Self::Output {
        Location { x: self.x + rhs.dx, y: self.y + rhs.dy }
    }
}

enum Obstacle {
    Slash,
    Backslash,
    Horizontal,
    Vertical
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    l: Location,
    d: Direction
}

impl Beam {
    fn advance(&mut self) {
        self.l = self.l + self.d;
    }
}

static WIDTH:AtomicI32 = AtomicI32::new(0);
static HEIGHT:AtomicI32 = AtomicI32::new(0);

fn part1(obstacles:&HashMap<Location, Obstacle>) -> i32 {
    count_energized(obstacles, Beam { l: Location { x: -1, y: 0 }, d: Direction { dx: 1, dy: 0 }})
}

fn part2(obstacles:&HashMap<Location, Obstacle>) -> i32 {
    let mut max = 0i32;
    for x in 0..WIDTH.load(std::sync::atomic::Ordering::Relaxed) {
        max = max
            .max(count_energized(obstacles, Beam { l: Location { x, y: -1 }, d: Direction { dx: 0, dy: 1 }}))
            .max(count_energized(obstacles, Beam { l: Location { x, y: HEIGHT.load(std::sync::atomic::Ordering::Relaxed) }, d: Direction { dx: 0, dy: -1 }}));
    }
    for y in 0..HEIGHT.load(std::sync::atomic::Ordering::Relaxed) {
        max = max
            .max(count_energized(obstacles, Beam { l: Location { x: -1, y }, d: Direction { dx: 1, dy: 0 }}))
            .max(count_energized(obstacles, Beam { l: Location { x: WIDTH.load(std::sync::atomic::Ordering::Relaxed), y }, d: Direction { dx: -1, dy: 0 }}));
    }
    
    max
}

fn count_energized(obstacles:&HashMap<Location, Obstacle>, start: Beam) -> i32 {
    use Obstacle::*;

    let mut beams:Vec<Beam> = vec![start];
    let mut energized: HashSet<Location> = Default::default();
    let mut visited:HashSet<Beam> = Default::default();
    // let mut cnt = 0;
    while beams.len() > 0 {
        // cnt+=1;
        // if  cnt > 45 {
        //     break;
        // }
        // for y in 0..HEIGHT.load(std::sync::atomic::Ordering::Relaxed) {
        //     for x in 0..WIDTH.load(std::sync::atomic::Ordering::Relaxed) {
        //         if energized.contains(&Location { x, y }) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }

        //     }
        //     println!();
        // }
        // println!();


        let mut beam = beams.pop().unwrap();
        beam.advance();
        if beam.l.valid() && !visited.contains(&beam) {
            visited.insert(beam.clone());
            energized.insert(beam.l);
            if let Some(obstacle) = obstacles.get(&beam.l) {
                match obstacle {
                    Slash => {
                        beam.d = Direction { dx: -beam.d.dy, dy: -beam.d.dx };
                    },
                    Backslash => {
                        beam.d = Direction { dx: beam.d.dy, dy: beam.d.dx };
                    },
                    Vertical => {
                        if beam.d.dx != 0 {
                            let fork: Beam = Beam { l: beam.l, d: Direction { dx: 0, dy: -1 }};
                            beam.d.dx = 0;
                            beam.d.dy = 1;
                            beams.push(fork);
                        }
                    },
                    Horizontal => {
                        if beam.d.dy != 0 {
                            let fork: Beam = Beam { l: beam.l, d: Direction { dx: -1, dy: 0 }};
                            beam.d.dx = 1;
                            beam.d.dy = 0;
                            beams.push(fork);
                        }
                    },
                }
    
            } 
            beams.push(beam);
            
        }

    }

    energized.len() as i32
}
