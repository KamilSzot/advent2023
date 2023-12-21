use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = contents.split("\n").filter(|s| !s.is_empty());

    println!("part 1: {}", part(1, lines.clone()));
    println!("part 2: {}", part(2, lines));

}

fn part<'a>(no:u32, lines: impl Iterator<Item = &'a str>) -> u32 {

    let mut sum:u32 = 0;
    for line in lines {
        // println!("{line}");

        let first_digit:u32 = find_first_digit(line, line.char_indices(), no == 1);
        let last_digit:u32 = find_first_digit(line, line.char_indices().rev(), no == 1);

        sum += first_digit*10+last_digit;
    }
    sum
}


static DIGITS:&'static [&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
fn find_first_digit(line: &str, indices: impl Iterator<Item = (usize, char)>, just_normal_digits: bool) -> u32 {
    let mut first_digit:u32 = 0;

    'first_digit_finder: for (p, c) in indices {
        if c.is_digit(10) {
            first_digit = c.to_digit(10).unwrap();
            break;

        } else {
            if !just_normal_digits {
                for (i, &d) in DIGITS.iter().enumerate() {
                    if d.eq(&line[p..(p+d.len()).min(line.len())]) {
                        first_digit = u32::try_from(i+1).unwrap();
                        break 'first_digit_finder;
                    }
                }
            }
        }
    }

    return first_digit;
}