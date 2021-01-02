fn validate1(line: &str) -> Option<bool> {
    let mut sp = line.splitn(3, ' ');
    let times = sp.next()?;
    let ch = sp.next()?.chars().nth(0)?;
    let passw = sp.next()?;

    let mut sp = times.splitn(2, '-');
    let min: usize = sp.next()?.parse().ok()?;
    let max: usize = sp.next()?.parse().ok()?;

    let cnt = passw.chars().filter(|&c| c == ch).count();
    Some(cnt >= min && cnt <= max)
}

fn validate2(line: &str) -> Option<bool> {
    let mut sp = line.splitn(3, ' ');
    let times = sp.next()?;
    let ch = sp.next()?.chars().nth(0)?;
    let mut passw = sp.next()?.chars();

    let mut sp = times.splitn(2, '-');
    let p1: usize = sp.next()?.parse().ok()?;
    let p2: usize = sp.next()?.parse().ok()?;

    let c1 = passw.nth(p1 - 1)?;
    let c2 = passw.nth(p2 - p1 - 1)?;

    Some((c1 == ch) ^ (c2 == ch))
}

fn count_good(lines: &[String], validate: fn(&str) -> Option<bool>) -> usize {
    lines
        .iter()
        .filter(|li| validate(&li).unwrap_or(false))
        .count()
}

fn main() {
    let lines: Vec<_> = aoc_2020::stdin_lines().collect::<Result<_, _>>().unwrap();

    let x = count_good(&lines, validate1);
    println!("part1: {}", x);

    let x = count_good(&lines, validate2);
    println!("part2: {}", x);
}
