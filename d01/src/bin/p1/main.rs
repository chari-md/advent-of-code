fn main() -> std::io::Result<()> {
    let (mut l, mut r): (Vec<i32>, Vec<i32>) = parse("input.txt").iter().cloned().unzip();

    l.sort();
    r.sort();

    let res: i32 = std::iter::zip(l, r).map(|(x, y)| (x - y).abs()).sum();
    return Ok(println!("{}", res));
}

fn parse(src: &str) -> Vec<(i32, i32)> {
    std::fs::read_to_string(src)
        .unwrap()
        .lines()
        .map(|x| {
            let elms = x
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (elms[0], elms[1])
        })
        .collect()
}
