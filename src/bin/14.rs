use std::collections::HashMap;

fn parse_mask(s: &str) -> (u64, u64, u64) {
    let mut or_mask = 0;
    let mut and_mask = !0;
    let mut z_mask = 0;
    for (i, c) in s.chars().rev().enumerate() {
        if c == '1' {
            or_mask |= 1 << i;
        } else if c == '0' {
            and_mask &= !(1 << i);
        } else if c == 'X' {
            z_mask |= 1 << i;
        }
    }
    (or_mask, and_mask, z_mask)
}

fn part1(lines: &[String]) -> u64 {
    let mut or_mask = !0;
    let mut and_mask = 0;
    let mut memory = HashMap::new();

    for line in lines.iter() {
        if line.starts_with("mask = ") {
            let (or, and, _) = parse_mask(&line[7..]);
            or_mask = or;
            and_mask = and;
        } else {
            let num_end = line.find(']').unwrap();
            let addr: u64 = line[4..num_end].parse().unwrap();
            let val: u64 = line[num_end + 4..].parse().unwrap();
            memory.insert(addr, (val & and_mask) | or_mask);
        }
    }
    memory.values().sum()
}

fn span_zmask(mut z_mask: u64, mut n: u64) -> u64 {
    let mut result = 0;
    for _ in 0..36 {
        result <<= 1;
        if (z_mask & 1) == 1 {
            result |= n & 1;
            n >>= 1;
        }
        z_mask >>= 1;
    }
    result.reverse_bits() >> 28
}

fn part2(lines: &[String]) -> u64 {
    let mut or_mask = !0;
    let mut z_mask = 0;
    let mut one_cnt_p2 = 0;
    let mut memory = HashMap::new();

    for line in lines {
        if line.starts_with("mask = ") {
            let (or, _, z) = parse_mask(&line[7..]);
            or_mask = or;
            z_mask = z;
            one_cnt_p2 = 2u64.pow(z_mask.count_ones());
        } else {
            let num_end = line.find(']').unwrap();
            let addr: u64 = line[4..num_end].parse().unwrap();
            let addr = (addr | or_mask) & !z_mask;
            let val: u64 = line[num_end + 4..].parse().unwrap();
            for m in 0..one_cnt_p2 {
                memory.insert(addr | span_zmask(z_mask, m), val);
            }
        }
    }
    memory.values().sum()
}

fn main() {
    let lines: Vec<_> = aoc_2020::stdin_lines().collect::<Result<_, _>>().unwrap();

    let x = part1(&lines);
    println!("part1: {}", x);

    let x = part2(&lines);
    println!("part2: {}", x);
}
