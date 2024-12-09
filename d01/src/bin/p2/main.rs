fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    Ok(println!("{}", solve(&input)))
}

fn solve(input: &str) -> i32 {
    let (l, r): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|x| {
            let elms = x
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (elms[0], elms[1])
        })
        .collect();

    let mut h = std::collections::HashMap::new();
    let mut res: i32 = 0;

    for i in l.iter() {
        if !h.contains_key(i) {
            h.insert(i, r.iter().filter(|&x| x == i).count() as i32);
        }

        if h.get(i).unwrap() > &0 {
            res += i * h.get(i).unwrap();
        }
    }

    res
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
        assert_eq!(solve(input), 31);
        Ok(())
    }
}
