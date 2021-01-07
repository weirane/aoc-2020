use std::collections::HashMap;

fn solve(buf: &str, predicate: impl Fn(&HashMap<&str, &str>) -> bool) -> usize {
    buf.split("\n\n")
        .map(|passport| {
            passport
                .split_ascii_whitespace()
                .filter_map(|entry| {
                    let mut sp = entry.split(':');
                    let key = sp.next()?;
                    let val = sp.next()?;
                    Some((key, val))
                })
                .collect()
        })
        .filter(predicate)
        .count()
}

fn part2_validate(kvmap: &HashMap<&str, &str>) -> Option<bool> {
    let byr: u32 = kvmap.get("byr")?.parse().ok()?;
    if byr < 1920 || byr > 2002 {
        return Some(false);
    }

    let iyr: u32 = kvmap.get("iyr")?.parse().ok()?;
    if iyr < 2010 || iyr > 2020 {
        return Some(false);
    }

    let eyr: u32 = kvmap.get("eyr")?.parse().ok()?;
    if eyr < 2020 || eyr > 2030 {
        return Some(false);
    }

    let hgt = kvmap.get("hgt")?;
    if hgt.ends_with("cm") {
        let x: u32 = hgt[..hgt.len() - 2].parse().ok()?;
        if x < 150 || x > 193 {
            return Some(false);
        }
    } else if hgt.ends_with("in") {
        let x: u32 = hgt[..hgt.len() - 2].parse().ok()?;
        if x < 59 || x > 76 {
            return Some(false);
        }
    } else {
        return Some(false);
    }

    let hcl = kvmap.get("hcl")?;
    if !(hcl.starts_with("#")
        && hcl.len() == 7
        && hcl
            .chars()
            .skip(1)
            .all(|c| '0' <= c && c <= '9' || 'a' <= c && c <= 'f'))
    {
        return Some(false);
    }

    let ecl = kvmap.get("ecl")?;
    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(ecl) {
        return Some(false);
    }

    let pid = kvmap.get("pid")?;
    if !(pid.len() == 9 && pid.chars().all(|c| '0' <= c && c <= '9')) {
        return Some(false);
    }

    Some(true)
}

fn main() {
    let buf = aoc_2020::stdin_string().unwrap();

    let needed = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let x = solve(&buf, |kvmap| needed.iter().all(|k| kvmap.contains_key(k)));
    println!("part1: {}", x);

    let x = solve(&buf, |kvmap| part2_validate(kvmap) == Some(true));
    println!("part2: {}", x);
}
