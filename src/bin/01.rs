use std::collections::HashSet;

fn part1(xs: &[i64], goal: i64) -> Option<i64> {
    let mut hs = HashSet::new();
    for x in xs {
        let rem = goal - x;
        if hs.contains(&rem) {
            return Some(x * rem);
        }
        hs.insert(x);
    }
    None
}

fn part2(xs: &[i64]) -> i64 {
    for (i, x) in xs.iter().enumerate() {
        let rem = 2020 - x;
        if let Some(mult) = part1(&xs[i + 1..], rem) {
            return x * mult;
        }
    }
    unreachable!();
}

fn main() {
    let xs: Vec<i64> = aoc_2020::stdin_lines()
        .flat_map(|s| s.ok().and_then(|s| s.parse().ok()))
        .collect();

    let x = part1(&xs, 2020);
    println!("part1: {}", x.unwrap());

    let x = part2(&xs);
    println!("part2: {}", x);
}
