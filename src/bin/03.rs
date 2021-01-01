use std::collections::HashSet;

type Position = (usize, usize);

fn part1(treepos: &HashSet<Position>, height: usize, width: usize, (dx, dy): Position) -> usize {
    (1..height)
        .map(|i| (i * dx % width, i * dy))
        .filter(|pos| treepos.contains(pos))
        .count()
}

fn part2(treepos: &HashSet<Position>, height: usize, width: usize) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&pos| part1(treepos, height, width, pos))
        .product()
}

fn main() {
    let lines: Vec<_> = aoc_2020::stdin_lines().collect::<Result<_, _>>().unwrap();
    let height = lines.len();
    let width = lines[0].chars().count();
    let treepos: HashSet<_> = lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(j, ch)| if ch == '#' { Some((j, i)) } else { None })
        })
        .collect();

    let x = part1(&treepos, height, width, (3, 1));
    println!("part1: {}", x);

    let x = part2(&treepos, height, width);
    println!("part2: {}", x);
}
