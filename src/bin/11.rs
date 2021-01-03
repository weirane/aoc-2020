use std::collections::HashSet;

use aoc_2020::cartesian_product;

type Pos = (i32, i32);
type NeighborCounter =
    fn(center: Pos, bound: Pos, nofloor: &HashSet<Pos>, occupied: &HashSet<Pos>) -> usize;

fn pos_valid((x, y): Pos, height: i32, width: i32) -> bool {
    x < height && y < width && x >= 0 && y >= 0
}

fn neighbors1(
    (x, y): Pos,
    (height, width): Pos,
    _: &HashSet<Pos>,
    occupied: &HashSet<Pos>,
) -> usize {
    cartesian_product(-1..=1, -1..=1)
        .filter(|&p| p != (0, 0))
        .filter_map(move |(i, j)| {
            let p = (x + i, y + j);
            pos_valid(p, height, width).then(|| p)
        })
        .filter(|p| occupied.contains(p))
        .count()
}

fn neighbors2(
    (x, y): Pos,
    (height, width): Pos,
    nofloor: &HashSet<Pos>,
    occupied: &HashSet<Pos>,
) -> usize {
    cartesian_product(-1..=1, -1..=1)
        .filter(|&p| p != (0, 0))
        .filter_map(move |(i, j)| {
            (1..)
                .map(|t| (x + i * t, y + j * t))
                .take_while(|&p| pos_valid(p, height, width))
                .filter_map(|p| (nofloor.contains(&p)).then(|| p))
                .next()
        })
        .filter(|p| occupied.contains(p))
        .count()
}

fn solve(
    nofloor: &HashSet<Pos>,
    height: i32,
    width: i32,
    thresh: usize,
    neighbors: NeighborCounter,
) -> usize {
    let mut occupied = HashSet::new();
    let mut prev_occu = vec![(0, 0)].into_iter().collect();
    while occupied != prev_occu {
        prev_occu = occupied;
        occupied = nofloor
            .iter()
            .filter_map(|&p| {
                let occ_count = neighbors(p, (height, width), &nofloor, &prev_occu);
                if prev_occu.contains(&p) {
                    // occupied. if < thresh neighbors are occupied, becomes occupied
                    (occ_count < thresh).then(|| p)
                } else {
                    // not occupied. if no occupied neighbors, becomes occupied
                    (occ_count == 0).then(|| p)
                }
            })
            .collect();
    }
    occupied.len()
}

fn main() {
    let lines: Vec<Vec<u8>> = aoc_2020::stdin_lines()
        .filter_map(|s| s.ok().map(|s| s.as_bytes().to_vec()))
        .collect();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;
    let nofloor: HashSet<_> = cartesian_product(0..height, 0..width)
        .filter(|&(i, j)| lines[i as usize][j as usize] == b'L')
        .collect();

    let x = solve(&nofloor, height, width, 4, neighbors1);
    println!("part1: {}", x);

    let x = solve(&nofloor, height, width, 5, neighbors2);
    println!("part2: {}", x);
}
