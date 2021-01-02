use std::collections::HashSet;

fn sum_to_goal(xs: &[i64], goal: i64) -> Option<i64> {
    let mut hs = HashSet::new();
    for x in xs {
        let rem = goal - x;
        if hs.contains(&rem) {
            return Some(*x);
        }
        hs.insert(x);
    }
    None
}

fn part1(xs: &[i64], window_size: usize) -> i64 {
    let mut it = xs.windows(window_size + 1).skip_while(|xs| {
        if let [pre @ .., goal] = xs {
            sum_to_goal(pre, *goal).is_some()
        } else {
            unreachable!("window size is (window_size + 1), so the last element exists");
        }
    });
    *it.next().and_then(|it| it.last()).unwrap()
}

fn part2(xs: &[i64], goal: i64) -> i64 {
    let acc: Vec<_> = xs
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();

    for (i, &x) in acc.iter().enumerate() {
        if let Ok(end) = acc[i + 1..].binary_search(&(goal + x)) {
            let from = i + 1;
            let to = from + end;
            let (mn, mx) = aoc_2020::min_max(&xs[from..=to]).unwrap();
            return mn + mx;
        }
    }
    panic!("cannot find answer");
}

fn main() {
    let xs: Vec<i64> = aoc_2020::stdin_lines()
        .filter_map(|s| s.ok().and_then(|s| s.parse().ok()))
        .collect();
    // should be 5 when using sample input
    let window_size = 25;

    let x = part1(&xs, window_size);
    println!("part1: {}", x);

    let x = part2(&xs, x);
    println!("part2: {}", x);
}
