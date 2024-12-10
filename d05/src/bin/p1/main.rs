use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let input = include_str!("../../../input.txt");
    Ok(println!("{}", solve(&input)))
}

fn solve(input: &str) -> i32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut rules_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for rule in rules.lines() {
        let (id, rule) = rule.split_once("|").unwrap();

        if rules_map.contains_key(id) {
            rules_map.get_mut(id).unwrap().push(rule);
        } else {
            rules_map.insert(id, vec![rule]);
        }
    }

    let mut middle_pages = vec![];

    for update in updates.lines() {
        let pages: Vec<&str> = update.split(",").collect();

        let mut safe = true;
        for (i, page) in pages.iter().enumerate() {
            // First page is always safe
            if i == 0 {
                continue;
            }

            if rules_map.contains_key(page) {
                let page_rules = rules_map.get(page).unwrap().clone();
                if !check(&pages, &page_rules, i) {
                    safe = false;
                    break;
                }
            }
        }

        if safe {
            let middle_page = pages[pages.len() / 2];
            middle_pages.push(middle_page.parse::<i32>().unwrap());
        }
    }

    middle_pages.iter().sum()
}

fn check(pages: &Vec<&str>, rules: &Vec<&str>, i: usize) -> bool {
    for rule in rules {
        let mut j = 0;
        while j < i {
            if pages[j] == *rule {
                return false;
            }
            j += 1;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(solve(&input), 143);
    }
}
