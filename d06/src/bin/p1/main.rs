fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt").unwrap();
    Ok(println!("{}", solve(&input)))
}

fn solve(input: &str) -> i32 {
    let mut map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut pos = (0, 0);
    let mut dir = (1, 0);
    let mut res = 1; // Start at 1 because we start at a position

    // Find the starting position and set the direction
    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '^' {
                pos = (i as i32, j as i32);
                dir = (-1, 0);
                break;
            } else if *col == 'v' {
                pos = (i as i32, j as i32);
                dir = (1, 0);
                break;
            } else if *col == '<' {
                pos = (i as i32, j as i32);
                dir = (0, -1);
                break;
            } else if *col == '>' {
                pos = (i as i32, j as i32);
                dir = (0, 1);
                break;
            }
        }
    }

    // Mark the starting position as visited
    map[pos.0 as usize][pos.1 as usize] = 'x';

    loop {
        // Move forward
        pos = (pos.0 + dir.0, pos.1 + dir.1);
        let (x, y) = pos;

        // Check if we are out of bounds
        if x < 0 || y < 0 || x >= map.len() as i32 || y >= map[0].len() as i32 {
            break;
        }
        // Mark as visited
        else if map[x as usize][y as usize] == '.' {
            map[x as usize][y as usize] = 'x';
            res += 1;
        }
        // Move back and change direction to the right
        else if map[x as usize][y as usize] == '#' {
            pos = (pos.0 - dir.0, pos.1 - dir.1);

            if dir == (0, 1) {
                dir = (1, 0);
            } else if dir == (1, 0) {
                dir = (0, -1);
            } else if dir == (0, -1) {
                dir = (-1, 0);
            } else if dir == (-1, 0) {
                dir = (0, 1);
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> std::io::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(solve(input), 41);
        Ok(())
    }
}
