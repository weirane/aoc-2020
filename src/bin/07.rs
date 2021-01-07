use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug)]
struct Rule(HashMap<String, HashMap<String, usize>>);

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hm = s
            .lines()
            .filter_map(|s| {
                let mut sp = s.splitn(2, " bags contain ");
                let subject = sp.next()?.to_string();
                let contain = sp
                    .next()?
                    .split(", ")
                    .filter_map(|r| {
                        let r = r
                            .trim_end_matches('.')
                            .trim_end_matches('s')
                            .trim_end_matches(" bag");
                        let mut sp = r.splitn(2, ' ');
                        let quant: usize = sp.next()?.parse().ok()?;
                        let color = sp.next()?.to_string();
                        Some((color, quant))
                    })
                    .collect();
                Some((subject, contain))
            })
            .collect();

        Ok(Self(hm))
    }
}

fn part1(rules: &Rule, orig: &str) -> usize {
    let mut can: HashSet<_> = rules
        .0
        .iter()
        .filter_map(|(k, v)| v.contains_key(orig).then(|| k))
        .collect();

    let mut changed = true;
    while changed {
        changed = false;
        for (k, v) in rules.0.iter() {
            if !can.contains(k) && v.keys().any(|k| can.contains(k)) {
                can.insert(k);
                changed = true;
            }
        }
    }
    can.len()
}

fn part2(rules: &Rule, orig: &str) -> usize {
    let mut deps = HashMap::new();

    fn helper<'a>(rules: &'a Rule, bag: &'a str, deps: &mut HashMap<&'a str, usize>) -> usize {
        if let Some(&n) = deps.get(bag) {
            n
        } else if let Some(rule) = rules.0.get(bag) {
            let x = rule
                .iter()
                .map(|(k, v)| v + v * helper(rules, k, deps))
                .sum();
            deps.insert(bag, x);
            x
        } else {
            unreachable!()
        }
    }
    helper(rules, orig, &mut deps)
}

fn main() {
    let rules: Rule = aoc_2020::stdin_string()
        .ok()
        .and_then(|rule| rule.parse().ok())
        .unwrap();

    let x = part1(&rules, "shiny gold");
    println!("part1: {}", x);

    let x = part2(&rules, "shiny gold");
    println!("part2: {}", x);
}
