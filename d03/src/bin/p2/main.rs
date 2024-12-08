use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let src = File::open("input.txt")?;
    let reader = io::BufReader::new(src);

    let mut ops = vec![];
    let mut is_do = true;
    for line in reader.lines() {
        let line = line?;
        is_do = scrape_line(&line, &mut ops, is_do);
    }

    Ok(println!("{}", ops.iter().sum::<i32>()))
}

fn scrape_line(line: &str, ops: &mut Vec<i32>, is_do: bool) -> bool {
    let mul = "mul(";

    let do_op = "do()";
    let do_not_op = "don't()";
    let mut is_do = is_do;

    let mut i = 0;
    while i < line.len() - mul.len() {
        if i < line.len() - do_op.len() && &line[i..i + do_op.len()] == do_op {
            is_do = true;
        } else if i < line.len() - do_not_op.len() && &line[i..i + do_not_op.len()] == do_not_op {
            is_do = false;
        } else if &line[i..i + mul.len()] == mul {
            i += mul.len();

            if !is_do {
                continue;
            }

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

    return is_do;
}
