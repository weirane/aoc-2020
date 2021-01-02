fn part1(xs: &[u64]) -> u64 {
    let mut c1 = 0;
    let mut c3 = 0;
    for d in xs.windows(2).map(|w| w[1] - w[0]) {
        if d == 1 {
            c1 += 1;
        } else if d == 3 {
            c3 += 1;
        }
    }
    c1 * c3
}

fn part2(xs: &[u64]) -> u64 {
    let mut choices = vec![0; xs.len()];
    choices[0] = 1;
    for (i, xi) in xs.iter().enumerate().skip(1) {
        for (j, xj) in xs[..i].iter().enumerate().rev() {
            if xi - xj > 3 {
                break;
            }
            choices[i] += choices[j];
        }
    }

    *choices.last().unwrap()
}

fn main() {
    let xs = {
        let mut xs: Vec<u64> = aoc_2020::stdin_lines()
            .filter_map(|s| s.ok().and_then(|s| s.parse().ok()))
            .collect();
        xs.push(0);
        xs.sort();
        xs.push(xs.last().unwrap() + 3);
        xs
    };

    let x = part1(&xs);
    println!("part1: {}", x);

    let x = part2(&xs);
    println!("part2: {}", x);
}
