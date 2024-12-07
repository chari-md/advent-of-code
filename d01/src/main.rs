fn main() -> std::io::Result<()> {
    let (mut l, mut r) = (vec![], vec![]);

    for line in std::fs::read_to_string("input.txt").unwrap().lines() {
        let elms = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        l.push(elms[0]);
        r.push(elms[1]);
    }

    l.sort();
    r.sort();

    let res: i32 = std::iter::zip(l, r).map(|(x, y)| (x - y).abs()).sum();
    return Ok(println!("{}", res));
}
