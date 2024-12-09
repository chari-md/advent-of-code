fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    Ok(println!("{}", solve(&input)))
}

fn solve(input: &str) -> i32 {
    input.lines()
    .filter(|x| check(x.split_whitespace()
    .map(|y| y.parse::<i32>().unwrap())
    .collect()))
    .count() as i32
}

fn check(r: Vec<i32>) -> bool {
    let is_increasing = r[0] - r[1] < 0;

    for i in 0..r.len() - 1 {
        let d = r[i] - r[i + 1];

        if is_increasing && d > 0 || !is_increasing && d < 0 {
            return false;
        }

        if d.abs() < 1 || d.abs() > 3 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> std::io::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(solve(input), 2);
        Ok(())
    }
}