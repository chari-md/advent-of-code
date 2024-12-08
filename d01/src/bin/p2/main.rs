fn main() -> std::io::Result<()> {
    let (l, r): (Vec<i32>, Vec<i32>) = parse("input.txt").iter().cloned().unzip();

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

    Ok(println!("{}", res))
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
