use std::fs;
use itertools::Itertools;
use nom::{self, IResult, sequence::{preceded, terminated, tuple}, multi::{separated_list1, many1}, character::complete::{i64, multispace1, line_ending, alpha1}, bytes::complete::tag, branch::alt};

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    part1(contents.clone());
    part2(contents);
    // part2(lines);
}


fn parse_seeds(input:&str) -> IResult<&str, Vec<i64>> {
    terminated(preceded(tag("seeds: "), separated_list1(multispace1, i64)), multispace1)(input)
}


fn parse_map(input:&str) -> IResult<&str, Vec<(i64,i64,i64)>> {
    preceded(tuple((many1(alt((alpha1, tag("-")))), tag(" map:"), multispace1)), separated_list1(line_ending, tuple((i64,preceded(multispace1, i64),preceded(multispace1, i64)))))(input)
}

fn parse_content(content:String) -> Data {
    let (input, seeds) =  parse_seeds(content.as_str()).unwrap();
    let (_, maps) = separated_list1(multispace1, parse_map)(input).unwrap();

    Data { seeds, maps }
}

#[derive(Debug)]
struct Data {
    seeds: Vec<i64>,
    maps: Vec<Vec<(i64,i64,i64)>>

}

fn part1(content:String) {
    let data = parse_content(content);

    // println!("{data:?}");

    let mut min = i64::MAX;
    for os in data.seeds {
        let mut s = os;
        for m in &data.maps {
            for &(dst, src, len) in m {
                if s >= src && s < src+len {
                    // println!("{} -> {} ({src} {dst} {len})", s, s - src + dst);
                    s = s - src + dst;
                    break;
                }
            }
        }
        if s < min {
            min = s;
        }
    }

    println!("{min}");
}

fn part2(content:String) {
    let data = parse_content(content);

    // println!("{data:?}");


    let mut intervals: Vec<(i64, i64)> = data.seeds.into_iter().tuples().collect();
    for m in &data.maps {
        let mut ointervals:Vec<(i64,i64)> = vec![];
        for &(dst, src, len) in m {
            let mut nintervals:Vec<(i64,i64)> = vec![];
            for (ileft, ilen) in &mut intervals {
                if *ileft >= src {
                    if *ileft < src+len {
                        let oleft = *ileft - src + dst;
                        let olen = (*ilen).min(len - (*ileft - src));
                        ointervals.push((oleft, olen));
                        *ilen -= olen;
                        *ileft += olen;
                    }
                } else {
                    if *ilen > src - *ileft {
                        if *ileft + *ilen <= src + len {
                            let oleft = dst;
                            let olen = *ilen - (src - *ileft);
                            ointervals.push((oleft, olen));
                            *ilen -= olen;
                        } else {
                            // split
                            let oleft = dst;
                            let olen = len;
                            ointervals.push((oleft, olen));

                            let nleft = src + len;
                            let nlen = *ileft + *ilen - (src + len);
                            nintervals.push((nleft, nlen));

                            *ilen = src - *ileft;
                        }
                    }
                }
            }
            intervals.extend(nintervals);        
        }
        intervals.extend(ointervals);
    }

    let mut min = i64::MAX;
    for (left, len) in intervals {
        // println!("{left} {len}");
        if len>0 && left < min {
            min = left;
        }
    
    }



    

    println!("{min}");
}

