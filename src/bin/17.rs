use std::collections::HashSet;

use aoc_2020::cartesian_product as cp;

type Pos2 = (i32, i32);
type Pos3 = (i32, i32, i32);
type Pos4 = (i32, i32, i32, i32);

fn neighbors3(&(x, y, z): &Pos3) -> impl Iterator<Item = Pos3> {
    cp(cp(-1..=1, -1..=1), -1..=1).filter_map(move |((dx, dy), dz)| {
        ((dx, dy, dz) != (0, 0, 0)).then(|| (x + dx, y + dy, z + dz))
    })
}

fn neighbors4(&(x, y, z, w): &Pos4) -> impl Iterator<Item = Pos4> {
    cp(cp(cp(-1..=1, -1..=1), -1..=1), -1..=1).filter_map(move |(((dx, dy), dz), dw)| {
        ((dx, dy, dz, dw) != (0, 0, 0, 0)).then(|| (x + dx, y + dy, z + dz, w + dw))
    })
}

fn part1(activepos: &HashSet<Pos2>) -> usize {
    let mut activepos: HashSet<_> = activepos.iter().map(|&(i, j)| (i, j, 0)).collect();
    for _ in 0..6 {
        activepos = activepos
            .iter()
            .flat_map(neighbors3)
            .filter(|p| {
                let n = neighbors3(p).filter(|x| activepos.contains(x)).count();
                n == 3 || (activepos.contains(&p) && n == 2)
            })
            .collect();
    }
    activepos.len()
}

fn part2(activepos: &HashSet<Pos2>) -> usize {
    let mut activepos: HashSet<_> = activepos.iter().map(|&(i, j)| (i, j, 0, 0)).collect();
    for _ in 0..6 {
        activepos = activepos
            .iter()
            .flat_map(neighbors4)
            .filter(|p| {
                let n = neighbors4(p).filter(|x| activepos.contains(x)).count();
                n == 3 || (activepos.contains(&p) && n == 2)
            })
            .collect();
    }
    activepos.len()
}

fn main() {
    let lines: Vec<_> = aoc_2020::stdin_lines().collect::<Result<_, _>>().unwrap();
    let activepos: HashSet<_> = lines
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(j, ch)| (ch == '#').then(|| (i as i32, j as i32)))
        })
        .collect();

    let x = part1(&activepos);
    println!("part1: {}", x);

    let x = part2(&activepos);
    println!("part2: {}", x);
}
