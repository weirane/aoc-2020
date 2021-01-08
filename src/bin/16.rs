#[derive(Debug, Clone)]
struct Rule<'a> {
    index: usize,
    name: &'a str,
    from1: u64,
    to1: u64,
    from2: u64,
    to2: u64,
}

impl<'a> Rule<'a> {
    fn new(index: usize, name: &'a str, from1: u64, to1: u64, from2: u64, to2: u64) -> Self {
        Self {
            index,
            name,
            from1,
            to1,
            from2,
            to2,
        }
    }

    fn test_valid(&self, n: u64) -> bool {
        self.from1 <= n && n <= self.to1 || self.from2 <= n && n <= self.to2
    }
}

fn parse_range(s: &str) -> Option<(u64, u64)> {
    let dash = s.find('-')?;
    let from = s[..dash].parse().ok()?;
    let to = s[dash + 1..].parse().ok()?;
    Some((from, to))
}

fn part1(rules: &[Rule], nearby: &[Vec<u64>]) -> u64 {
    nearby
        .iter()
        .flat_map(|x| x.iter())
        .filter(|&&x| rules.iter().all(|rule| !rule.test_valid(x)))
        .sum()
}

fn part2(rules: &[Rule], mine: &[u64], nearby: &[Vec<u64>]) -> u64 {
    let valids: Vec<_> = nearby
        .iter()
        .filter(|tic| {
            tic.iter()
                .all(|&x| rules.iter().any(|rule| rule.test_valid(x)))
        })
        .collect();
    // fields valid for a rule
    let valid_rules: Vec<Vec<_>> = rules
        .iter()
        .map(|rule| {
            (0..rules.len())
                .filter(|&i| valids.iter().map(|tic| tic[i]).all(|f| rule.test_valid(f)))
                .collect()
        })
        .collect();
    // order of rules to try
    let mut order: Vec<_> = valid_rules
        .iter()
        .enumerate()
        .map(|(i, x)| (i, x.len()))
        .collect();
    order.sort_by_key(|x| x.1);
    let order: Vec<_> = order.iter().map(|x| x.0).collect();

    fn helper(
        dispatch: &mut Vec<usize>,
        curr_rule: usize,
        rules: &[Rule],
        valids: &[&Vec<u64>],
        valid_rules: &[Vec<usize>],
        order: &[usize],
    ) -> bool {
        let field_num = rules.len();
        if curr_rule == field_num {
            return true;
        }
        let rule = order[curr_rule];
        let candidates: Vec<_> = valid_rules[rule]
            .iter()
            .filter(|&tic| !dispatch.contains(tic))
            .collect();
        for &c in candidates {
            dispatch[rule] = c;
            if helper(dispatch, curr_rule + 1, rules, valids, valid_rules, order) {
                return true;
            }
            dispatch[rule] = field_num;
        }
        false
    }

    // dispatch[i] == j means rule i applies to field j
    let mut dispatch = vec![rules.len(); rules.len()];
    let succ = helper(&mut dispatch, 0, rules, &valids, &valid_rules, &order);
    assert!(succ);
    rules
        .iter()
        .filter_map(|rule| {
            rule.name
                .starts_with("departure")
                .then(|| mine[dispatch[rule.index]])
        })
        .product()
}

fn main() {
    let input = aoc_2020::stdin_string().unwrap();
    let mut it = input.split("\n\n");
    let rules: Vec<_> = it
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let colon = line.find(": ")?;
            let field = &line[..colon];
            let ranges = &line[colon + 2..];
            let or = ranges.find(" or ")?;
            let (from1, to1) = parse_range(&ranges[..or])?;
            let (from2, to2) = parse_range(&ranges[or + 4..])?;
            Some(Rule::new(i, field, from1, to1, from2, to2))
        })
        .collect::<Option<_>>()
        .unwrap();

    let mine_str = it.next().unwrap();
    // skip the literal "your ticket:"
    let pos = mine_str.find('\n').unwrap();
    let mine: Vec<u64> = mine_str[pos + 1..]
        .split(',')
        .map(|s| s.parse().ok())
        .collect::<Option<_>>()
        .unwrap();

    let nearby: Vec<Vec<u64>> = it
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().ok())
                .collect::<Option<_>>()
        })
        .collect::<Option<_>>()
        .unwrap();

    let x = part1(&rules, &nearby);
    println!("part1: {}", x);

    let x = part2(&rules, &mine, &nearby);
    println!("part2: {}", x);
}
