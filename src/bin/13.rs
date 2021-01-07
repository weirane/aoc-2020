fn part1(earliest: i64, in_service: &[Option<i64>]) -> i64 {
    let (bus, time) = in_service
        .iter()
        .filter_map(|x| x.as_ref())
        .map(|&b| (b, (b - earliest % b) % b))
        .min_by_key(|x| x.1)
        .unwrap();
    bus * time
}

fn part2(in_service: &[Option<i64>]) -> i64 {
    let mut bus_rem: Vec<_> = in_service
        .iter()
        .enumerate()
        .filter_map(|(i, bus)| bus.map(|b| (b, (b - i as i64 % b) % b)))
        .collect();
    bus_rem.sort_by_key(|x| -x.0);
    // assuming buses are pairwise coprime

    let (mut interval, mut ret) = bus_rem.pop().unwrap();
    while let Some((bus, rem)) = bus_rem.pop() {
        while ret % bus != rem {
            ret += interval;
        }
        interval *= bus;
    }
    // ret has to satisfy: bus_rem.iter().all(|(bus, rem)| ret % bus == rem)
    ret
}

fn main() {
    let mut lines = aoc_2020::stdin_lines().take(2);
    let earliest: i64 = lines.next().and_then(|ln| ln.ok()?.parse().ok()).unwrap();
    let in_service: Vec<Option<i64>> = lines
        .next()
        .and_then(|li| Some(li.ok()?.split(',').map(|x| x.parse().ok()).collect()))
        .unwrap();

    let x = part1(earliest, &in_service);
    println!("part1: {}", x);

    let x = part2(&in_service);
    println!("part2: {}", x);
}
