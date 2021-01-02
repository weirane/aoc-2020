use std::collections::HashSet;

fn part1(insts: &[(String, i32)]) -> (bool, i32) {
    let mut acc = 0;
    let mut pc = 0;
    let mut executed = HashSet::new();
    let len = insts.len() as i32;

    while pc < len {
        if pc < 0 || executed.contains(&pc) {
            return (false, acc);
        }
        executed.insert(pc);
        let (ref inst, op) = insts[pc as usize];
        match (inst.as_str(), op) {
            ("nop", _) => pc += 1,
            ("acc", x) => {
                acc += x;
                pc += 1
            }
            ("jmp", x) => pc += x,
            _ => unreachable!("invalid instruction"),
        }
    }
    (true, acc)
}

fn part2(insts: &[(String, i32)]) -> i32 {
    let mut imem = insts.to_vec();

    for (i, (inst, _)) in insts.iter().enumerate().filter(|(_, (i, _))| i != "acc") {
        imem[i].0 = if inst == "nop" { "jmp" } else { "nop" }.to_string();
        let (terminated, acc) = part1(&imem);
        if terminated {
            return acc;
        }
        imem[i].0 = inst.clone();
    }
    unreachable!()
}

fn main() {
    let insts: Vec<_> = aoc_2020::stdin_lines()
        .filter_map(|s| {
            let s = s.ok()?;
            let mut sp = s.splitn(2, ' ');
            let inst = sp.next()?.to_string();
            let op: i32 = sp.next()?.parse().ok()?;
            Some((inst, op))
        })
        .collect();

    let (terminated, x) = part1(&insts);
    assert!(!terminated);
    println!("part1: {}", x);

    let x = part2(&insts);
    println!("part2: {}", x);
}
