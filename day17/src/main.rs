use std::{fs, rc::Rc, ops::{Add, AddAssign, Mul}, collections::{BinaryHeap, HashMap}, cmp};
// use nom::{self, sequence::{pair, preceded}, branch::alt, character::complete::{alpha1, char, i32}, combinator::{map, value}};

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let cost = contents.split("\n").filter(|s| !s.is_empty()).map(|line| line.chars()).map(|row| row.map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>()).collect::<Vec<_>>();
    let board = Board { 
        width: cost[0].len() as _, 
        height: cost.len() as _, 
        cost 
    };

    println!("{}", part1(board.clone()));
    println!("{}", part2(board));
}

#[derive(Debug, Clone)]
struct Board {
    cost: Vec<Vec<i32>>,
    width: i32,
    height: i32
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct State {
    location: Location,
    last_direction: Direction,
    steps_left: i32
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct NodeStruct {
    parent: NodePointer,
    state: State
}

type Node = Rc<NodeStruct>;

type NodePointer = Option<Node>;

impl PartialOrd for NodeStruct {
    fn partial_cmp(&self, _: &Self) -> Option<cmp::Ordering> {
        None
    }
}

impl Ord for NodeStruct {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        cmp::Ordering::Equal
    }
}

fn part1(board:Board) -> i32 {
    let mut open_heap: BinaryHeap<(i32, Node)> = Default::default();
    let mut open: HashMap<State, i32> = Default::default();
    let mut closed: HashMap<State, i32> = Default::default();

    let start = Location { x: 0, y: 0 };
    let finish = Location {
        x: board.width-1,
        y: board.height-1
    };


    let initial = Rc::new(NodeStruct { parent: None, state: State { location: start, last_direction: Direction { dx: 1, dy: 0 }, steps_left: 3 }});
    open.insert(initial.state.clone(), start.distance(&finish));
    open_heap.push((-start.distance(&finish), initial));

    while let Some((f, q)) = open_heap.pop() {
        let mut successors = vec![];

        if (q.state.location + q.state.last_direction.left()).valid(&board) {
            successors.push(Rc::new(NodeStruct { parent: Some(q.clone()), state: State { location: q.state.location + q.state.last_direction.left(), last_direction: q.state.last_direction.left(), steps_left: 2 }}));
        }
        if (q.state.location + q.state.last_direction.right()).valid(&board) {
            successors.push(Rc::new(NodeStruct { parent: Some(q.clone()), state: State { location: q.state.location + q.state.last_direction.right(), last_direction: q.state.last_direction.right(), steps_left: 2 }}));
        }
        if q.state.steps_left > 0 && (q.state.location + q.state.last_direction).valid(&board) {
            successors.push(Rc::new(NodeStruct { parent: Some(q.clone()), state: State { location: q.state.location + q.state.last_direction, last_direction: q.state.last_direction, steps_left: q.state.steps_left - 1 }}));
        }
        // println!("{} {:?} {:?}", f, q, successors);
        // break;
        'successor: for succ in successors {
            let nf = -f - q.state.location.distance(&finish) // previous cost, which means est - heruistics
                + board.cost[succ.state.location.y as usize][succ.state.location.x as usize] as i32  // plus additional cost
                + succ.state.location.distance(&finish);  // plus heuristic

            if succ.state.location == finish  {
                return nf; //        win                                            *********************
            }
    
            if let Some(of) = open.get(&succ.state) {
                if -of < nf {
                    continue 'successor;
                }
            }
            if let Some(of) = closed.get(&succ.state) {
                if -of < nf {
                    continue 'successor;
                }
            }
            open.insert(succ.state.clone(), nf);
            open_heap.push((-nf, succ));
        }


        closed.insert(q.state.clone(), f);
    }

    0
}


fn part2(board:Board) -> i32 {
    let mut open_heap: BinaryHeap<(i32, Node)> = Default::default();
    let mut open: HashMap<State, i32> = Default::default();
    let mut closed: HashMap<State, i32> = Default::default();

    let finish = Location {
        x: board.width-1,
        y: board.height-1
    };



    let initial = Rc::new(NodeStruct { parent: None, state: State { location: Location { x:4, y:0 }, last_direction: Direction { dx: 1, dy: 0 }, steps_left: 6 }});
    let cost:i32 = board.cost[0][1..=4].iter().sum::<i32>() + initial.state.location.distance(&finish);
    
    open.insert(initial.state.clone(), cost);
    open_heap.push((-cost, initial));

    let initial = Rc::new(NodeStruct { parent: None, state: State { location: Location { x:0, y:4 }, last_direction: Direction { dx: 0, dy: 1 }, steps_left: 6 }});
    let cost:i32 = board.cost.iter().map(|row| row[0]).skip(1).take(4).sum::<i32>() + initial.state.location.distance(&finish);
    open.insert(initial.state.clone(), cost);
    open_heap.push((-cost, initial));

    while let Some((f, q)) = open_heap.pop() {
        let mut successors = vec![];

        if (q.state.location + q.state.last_direction.left() * 4).valid(&board) {
            let succ = Rc::new(NodeStruct { parent: Some(q.clone()), state: State { location: q.state.location + q.state.last_direction.left()*4, last_direction: q.state.last_direction.left(), steps_left: 6 }});
            let mut cost: i32 = 0;
            let mut cur = q.state.location.clone();
            loop {
                cur  += q.state.last_direction.left();
                cost += board.cost[cur.y as usize][cur.x as usize] as i32;
                if  cur == succ.state.location { break; }
            }
            successors.push((cost, succ));
        }
        if (q.state.location + q.state.last_direction.right() * 4).valid(&board) {
            let succ = Rc::new(NodeStruct { parent: Some(q.clone()), state: State { location: q.state.location + q.state.last_direction.right()*4, last_direction: q.state.last_direction.right(), steps_left: 6 }});
            let mut cost: i32 = 0;
            let mut cur = q.state.location.clone();
            loop {
                cur  += q.state.last_direction.right();
                cost += board.cost[cur.y as usize][cur.x as usize] as i32;
                if  cur == succ.state.location { break; }
            }
            successors.push((cost, succ));
        }
        if q.state.steps_left > 0 && (q.state.location + q.state.last_direction).valid(&board) {
            let succ = Rc::new(NodeStruct { parent: Some(q.clone()), state: State { location: q.state.location + q.state.last_direction, last_direction: q.state.last_direction, steps_left: q.state.steps_left - 1 }});
            let cost = board.cost[succ.state.location.y as usize][succ.state.location.x as usize] as i32;
            successors.push((cost, succ));
        }
        'successor: for (cost, succ) in successors {
            let nf = -f - q.state.location.distance(&finish) // previous cost, which means est - heruistics
                + cost  // plus additional cost
                + succ.state.location.distance(&finish);  // plus heuristic

            if succ.state.location == finish  {
                // let mut node = succ;
                // loop {
                //     println!("{:?}", node.state);
                //     if node.parent == None {
                //         break;
                //     }
                //     node = node.parent.clone().unwrap();
                // }
                return nf; //        win                                            *********************
            }
    
            if let Some(of) = open.get(&succ.state) {
                if -of < nf {
                    continue 'successor;
                }
            }
            if let Some(of) = closed.get(&succ.state) {
                if -of < nf {
                    continue 'successor;
                }
            }
            open.insert(succ.state.clone(), nf);
            open_heap.push((-nf, succ));
        }


        closed.insert(q.state.clone(), f);
    }

    0
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: i32,
    y: i32
}

impl Location {
    fn valid(&self, board: &Board) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < board.width && self.y < board.height
    }
    fn distance(&self, oth:&Location) -> i32 {
        (self.x - oth.x).abs() + (self.y - oth.y).abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Direction {
    dx: i32,
    dy: i32
}

impl Direction {
    fn left(&self) -> Direction {
        Direction {
            dx: self.dy,
            dy: -self.dx
        }
    }
    fn right(&self) -> Direction {
        Direction {
            dx: -self.dy,
            dy: self.dx
        }
    }
}

impl Add<Direction> for Location {
    type Output = Location;

    fn add(self, rhs: Direction) -> Self::Output {
        Location { x: self.x + rhs.dx, y: self.y + rhs.dy }
    }
}

impl AddAssign<Direction> for Location {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl Mul<i32> for Direction {
    type Output = Direction;

    fn mul(self, rhs: i32) -> Self::Output {
        Direction {
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}

