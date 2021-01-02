use std::io::Read;

fn part1(buf: &str) -> usize {
    buf.split("\n\n")
        .map(|group| ('a'..='z').filter(|&c| group.contains(c)).count())
        .sum()
}

fn part2(buf: &str) -> usize {
    buf.split("\n\n")
        .map(|group| {
            ('a'..='z')
                .filter(|&c| group.lines().all(|li| li.contains(c)))
                .count()
        })
        .sum()
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let x = part1(&buf);
    println!("part1: {}", x);

    let x = part2(&buf);
    println!("part2: {}", x);
}
