use std::collections::HashMap;

fn solve(start: &[usize], upper: usize) -> usize {
    let mut mem: HashMap<_, _> = start
        .iter()
        .take(start.len() - 1)
        .enumerate()
        .map(|(i, &x)| (x, i + 1))
        .collect();
    let mut last = *start.last().unwrap();

    for x in start.len()..upper {
        match mem.get(&last) {
            None => {
                mem.insert(last, x);
                last = 0;
            }
            Some(&n) => {
                mem.insert(last, x);
                last = x - n;
            }
        }
    }
    last
}

fn main() {
    let start: Vec<usize> = aoc_2020::stdin_string()
        .unwrap()
        .trim()
        .split(',')
        .filter_map(|li| li.parse().ok())
        .collect();

    let x = solve(&start, 2020);
    println!("part1: {}", x);

    let x = solve(&start, 30000000);
    println!("part2: {}", x);
}
