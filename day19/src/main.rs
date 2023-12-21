use std::{fs, fmt::Debug, collections::HashMap};
use itertools::Itertools;
use nom::{multi::separated_list1, character::complete::{line_ending, char, i32, alpha1, one_of}, sequence::{delimited, preceded, tuple}, branch::alt, bytes::complete::tag, Err, combinator::map};
// use nom::{self, sequence::{pair, preceded}, branch::alt, character::complete::{alpha1, char, i32}, combinator::{map, value}};
use small_map::SmallMap;

fn main() {

    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let (workflows_data, parts_data) = contents.split("\n\n").filter(|s| !s.is_empty()).next_tuple().unwrap();

    let parts = parse_parts(parts_data);
    let workflows:HashMap<String, Workflow> = parse_workflows(workflows_data).into_iter().map(|w| (w.name.clone(), w)).collect();
    part1(&parts,  &workflows);
    part2(&workflows);


}


fn part2(workflows:&HashMap<String, Workflow>) {
    let mut initial = PartGroup { fields: Default::default() };
    initial.fields.insert(Field::X, (1, 4000));
    initial.fields.insert(Field::M, (1, 4000));
    initial.fields.insert(Field::A, (1, 4000));
    initial.fields.insert(Field::S, (1, 4000));

    let mut accepted:Vec<PartGroup> = Default::default();
    let mut rejected:Vec<PartGroup> = Default::default();

    let mut groups: Vec<(String, PartGroup)> = vec![("in".to_owned(), initial)];
    while let Some((workflow_name, mut group)) = groups.pop() {
        for rule in &workflows[&workflow_name].rules {
            match rule.condition {
                Condition::Always => match rule.target.as_str() {
                    "A" => { accepted.push(group); break; },
                    "R" => { rejected.push(group); break; },
                    _ => { groups.push((rule.target.clone(), group)); break; }
                },
                Condition::LessThan(field, value) => {
                    let mut fitting = group.clone();
                    let (left, right) = fitting.fields.get_mut(&field).unwrap();
                    *right = (value-1).min(*right);
                    if left <= right {
                        match rule.target.as_str() {
                            "A" => { accepted.push(fitting); },
                            "R" => { rejected.push(fitting); },
                            _ => { groups.push((rule.target.clone(), fitting)); }
                        }                        
                    }
                    let (left, right) = group.fields.get_mut(&field).unwrap();
                    *left = value.max(*left);
                    if left>right {
                        break;
                    }
                },
                Condition::MoreThan(field, value) => {
                    let mut fitting = group.clone();
                    let (left, right) = fitting.fields.get_mut(&field).unwrap();
                    *left = (value+1).max(*left);
                    if left <= right {
                        match rule.target.as_str() {
                            "A" => { accepted.push(fitting); },
                            "R" => { rejected.push(fitting); },
                            _ => { groups.push((rule.target.clone(), fitting)); }
                        }                        
                    }
                    let (left, right) = group.fields.get_mut(&field).unwrap();
                    *right = value.min(*right);
                    if left>right {
                        break;
                    }
                },
            }
        }
    }
    let sum = accepted.iter().map(|g| g.total()).sum::<i128>();
    // for a in accepted { println!("{:?}", a); }
    println!("{sum}");

}

fn part1(parts:&Vec<Part>, workflows:&HashMap<String, Workflow>) {
    let mut sum = 0;
    for part in parts {
        if part.accepted(workflows) {
            sum += part.total();
        }
    }

    println!("{sum}");
}

impl Part {
    fn accepted(&self, workflows:&HashMap<String, Workflow>) -> bool {
        let mut current = &workflows[&"in".to_owned()];
        loop {
            // println!("{}", current.name);
            for rule in &current.rules {
                match &rule.condition {
                    Condition::Always => match rule.target.as_str() {
                        "A" => { return true; },
                        "R" => { return false; },
                        _ => { current = &workflows[&rule.target]; break; }
                    },
                    Condition::LessThan(field, value) => {
                        // println!("{} < {}", self.fields.get(field).unwrap(), value);
                        if self.fields.get(field).unwrap() < value {
                            match rule.target.as_str() {
                                "A" => { return true; },
                                "R" => { return false; },
                                _ => { current = &workflows[&rule.target]; break; }
                            }
                        }
                    },
                    Condition::MoreThan(field, value) => {
                        if self.fields.get(field).unwrap() > value {
                            match rule.target.as_str() {
                                "A" => { return true; },
                                "R" => { return false; },
                                _ => { current = &workflows[&rule.target]; break; }
                            }
                        }
                    },


                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct PartGroup {
    fields: HashMap<Field, (i32, i32)>,
}

impl PartGroup {
    fn total(&self) -> i128 {
        self.fields.iter().map(|(_, (from, to))| ((to-from)+1) as i128).product()
    }
}

struct Part {
    fields: SmallMap<4, Field, i32>,
}

impl Part {
    fn total(&self) -> i32 {
        self.fields.iter().map(|(_, &v)| v).sum()
    }
}

impl Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Part")
        .field("x", &self.fields.get(&Field::X))
        .field("m", &self.fields.get(&Field::M))
        .field("a", &self.fields.get(&Field::A))
        .field("s", &self.fields.get(&Field::S))
        .finish()
    }
}

fn parse_parts(input:&str) -> Vec<Part> {
    let (_, parts) = separated_list1(line_ending::<&str, ()>, delimited(char('{'), separated_list1(char(','), preceded(alt((tag("x="), tag("m="), tag("a="), tag("s="))), i32)) ,char('}')))(input).unwrap();
    parts.iter().map(|part| {
        let (x,m,a,s) = part.iter().cloned().next_tuple().unwrap();
        let mut fields:SmallMap<4, Field, i32> = Default::default();
        use Field::*;
        fields.insert(X, x);
        fields.insert(M, m);
        fields.insert(A, a);
        fields.insert(S, s);
        Part { fields }
    }).collect_vec()
}

fn parse_workflows(input:&str) -> Vec<Workflow> {
    let (_, workflows) = separated_list1(line_ending, 
        map(
            tuple((alpha1, delimited(char('{'), separated_list1(char(','), parse_rule), char('}')))),
            |(name, rules)| Workflow { name: name.to_owned(), rules }
    ))(input).unwrap();
    workflows
}

fn parse_rule(input:&str) -> Result<(&str, Rule), Err<()>> {
    alt((
        map(tuple((
            one_of::<&str, _, ()>("xmas"), 
            one_of("<>"), 
            i32,
            preceded(char(':'), alpha1)
        )), |(fld, op, val, target_name)| Rule { condition: if op == '<' { Condition::LessThan(Field::from(fld), val) } else { Condition::MoreThan(Field::from(fld), val) }, target: target_name.to_owned() }),
        map(alpha1, |target_name:&str| Rule { condition:Condition::Always, target: target_name.to_owned() })
    ))(input)
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>
}

#[derive(Debug)]
struct Rule {
    condition: Condition,
    target: String
}

#[derive(Debug)]
enum Condition {
    Always,   
    LessThan(Field, i32),
    MoreThan(Field, i32)    
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Field {
    X, M, A, S
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        use Field::*;
        match value {
            'x' => X,
            'm' => M,
            'a' => A,
            's' => S,
            _ => panic!("wrong char {value}")
        }
    }
}

