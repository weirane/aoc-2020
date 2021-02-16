use std::collections::HashMap;

#[derive(Debug)]
enum Rule<'a> {
    Literal(&'a str),
    Composite(Vec<Vec<&'a str>>),
}

type Parser<'a, 'b> = Box<dyn Fn(&'b str) -> Option<&'b str> + 'a>;

fn make_parser<'a, 'b: 'a>(name: &'a str, rules: &'a HashMap<&'a str, Rule>) -> Parser<'a, 'b> {
    match &rules[name] {
        Rule::Literal(lit) => {
            Box::new(move |s: &'b str| s.starts_with(lit).then(|| &s[lit.len()..]))
        }
        Rule::Composite(components) => Box::new(move |s: &'b str| {
            components.iter().fold(None, |acc, or| {
                acc.or_else(|| {
                    or.iter()
                        .fold(Some(s), |acc, and| acc.and_then(make_parser(and, rules)))
                })
            })
        }),
    }
}

// 0: 8 11
// 8: 42 | 42 8
// 11: 42 31 | 42 11 31
// ==> 0: (m occurences of 42) (n occurences of 31)
//        where m > n >= 1
fn make_parser2<'a, 'b: 'a>(rules: &'a HashMap<&'a str, Rule>) -> Parser<'a, 'b> {
    Box::new(move |s: &'b str| {
        (0..s.len())
            .scan(Some(s), |state, i| {
                *state = state.and_then(make_parser("42", &rules));
                Some((i, *state))
            })
            .find_map(|(i, rest)| {
                (0..i)
                    .scan(rest, |state, _| {
                        *state = state.and_then(make_parser("31", &rules));
                        *state
                    })
                    .last()
            })
    })
}

fn solve<'a>(tests: &'a [&str], parser: Parser<'a, 'a>) -> usize {
    tests.iter().filter(|s| parser(s) == Some("")).count()
}

fn main() {
    let input = aoc_2020::stdin_string().unwrap();
    let split = input.find("\n\n").unwrap();
    let rules: HashMap<_, _> = input[..split]
        .lines()
        .map(|line| {
            let colon = line.find(": ")?;
            let name = &line[..colon];
            let rest = &line[colon + 2..];
            let rule = if rest.starts_with('"') {
                Rule::Literal(rest.trim_matches('"'))
            } else {
                Rule::Composite(
                    rest.split(" | ")
                        .map(|or| or.split(' ').collect())
                        .collect(),
                )
            };
            Some((name, rule))
        })
        .collect::<Option<_>>()
        .unwrap();
    let tests: Vec<_> = input[split + 2..].lines().collect();

    let x = solve(&tests, make_parser("0", &rules));
    println!("part1: {}", x);

    let x = solve(&tests, make_parser2(&rules));
    println!("part2: {}", x);
}
