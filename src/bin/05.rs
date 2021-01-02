fn decode_seat(seat: &str) -> u32 {
    seat.chars().fold(0, |acc, c| {
        acc * 2 + if c == 'B' || c == 'R' { 1 } else { 0 }
    })
}

fn main() {
    let seats: Vec<_> = aoc_2020::stdin_lines()
        .map(|s| s.map(|s| decode_seat(&s)))
        .collect::<Result<_, _>>()
        .unwrap();

    let (min, max) = aoc_2020::min_max(&seats).unwrap();
    println!("part1: {}", max);

    let x = (min + max) * (max - min + 1) / 2 - seats.iter().sum::<u32>();
    println!("part2: {}", x);
}
