use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let src = File::open("input.txt")?;
    let reader = io::BufReader::new(src);

    let mut ops = vec![];
    for line in reader.lines() {
        let line = line?;
        scrape_line(&line, &mut ops);
    }

    Ok(println!("{}", ops.iter().sum::<i32>()))
}

fn scrape_line(line: &str, ops: &mut Vec<i32>) {
    let mul = "mul(";

    let mut i = 0;
    while i < line.len() - mul.len() {
        if &line[i..i + mul.len()] == mul {
            i = i + mul.len();
            let mut j = i;

            while j < line.len() {
                let c = line.chars().nth(j).unwrap();

                if c.is_numeric() || c == ',' {
                    j += 1;
                    continue;
                } else if c == ')' {
                    let operands = &line[i..j]
                        .split(",")
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    ops.push(operands[0] * operands[1]);

                    i = j;
                    break;
                } else {
                    i = j;
                    break;
                }
            }
        }

        i += 1;
    }
}
