fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    Ok(println!("{}", solve(&input)))
}

fn solve(input: &str) -> i32 {
    let (mut l, mut r): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|x| {
            let elms = x
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (elms[0], elms[1])
        })
        .collect();

    l.sort();
    r.sort();

    std::iter::zip(l, r).map(|(x, y)| (x - y).abs()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> std::io::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(solve(input), 11);
        Ok(())
    }
}
