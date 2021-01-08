#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Plus,
    Mult,
    Number(i64),
}

fn tokenize(s: &str) -> Option<Vec<Token>> {
    use Token::*;
    let mut chars = s.chars().peekable();
    let mut ret = Vec::new();
    let mut num = 0i64;
    while let Some(c) = chars.next() {
        match c {
            '(' => ret.push(OpenParen),
            ')' => ret.push(CloseParen),
            '+' => ret.push(Plus),
            '*' => ret.push(Mult),
            c if c.is_whitespace() => (),
            c if c.is_ascii_digit() => {
                num = num * 10 + (c as u8 - b'0') as i64;
                if chars.peek().map(|c| !c.is_ascii_digit()).unwrap_or(true) {
                    ret.push(Number(num));
                    num = 0;
                }
            }
            _ => return None,
        }
    }
    Some(ret)
}

mod part1 {
    use super::Token;
    pub fn solve(es: &[Vec<Token>]) -> i64 {
        es.iter().map(|e| parse_expr(e).0).sum()
    }

    fn parse_expr(es: &[Token]) -> (i64, &[Token]) {
        let (mut result, mut es) = parse_factor(es);
        while let [tok, rest @ ..] = es {
            if let Token::CloseParen = tok {
                break;
            }
            let (n, esp) = parse_factor(rest);
            es = esp;
            match tok {
                Token::Plus => result += n,
                Token::Mult => result *= n,
                _ => panic!("invalid: {:?}", tok),
            }
        }
        (result, es)
    }

    fn parse_factor(es: &[Token]) -> (i64, &[Token]) {
        match es {
            [Token::Number(n), rest @ ..] => (*n, rest),
            [Token::OpenParen, rest @ ..] => {
                if let (n, [Token::CloseParen, rest @ ..]) = parse_expr(rest) {
                    (n, rest)
                } else {
                    panic!("missing close paren")
                }
            }
            _ => panic!("invalid: {:?}", es),
        }
    }
}

/// Using top-down method to parse the following grammar:
///   E -> T * E | T
///   T -> F + T | F
///   F -> ( E ) | num
mod part2 {
    use super::Token;
    pub fn solve(es: &[Vec<Token>]) -> i64 {
        es.iter().filter_map(|e| parse_expr(e).map(|x| x.0)).sum()
    }

    fn parse_token(tok: Token) -> impl Fn(&[Token]) -> Option<(Token, &[Token])> {
        move |es| match es {
            [t, rest @ ..] if t == &tok => Some((tok, rest)),
            _ => None,
        }
    }

    fn parse_num(es: &[Token]) -> Option<(i64, &[Token])> {
        match es {
            [Token::Number(n), rest @ ..] => Some((*n, rest)),
            _ => None,
        }
    }

    fn parse_factor(es: &[Token]) -> Option<(i64, &[Token])> {
        parse_num(es).or_else(|| {
            let (_, es) = parse_token(Token::OpenParen)(es)?;
            let (n, es) = parse_expr(es)?;
            let (_, es) = parse_token(Token::CloseParen)(es)?;
            Some((n, es))
        })
    }

    fn parse_term(es: &[Token]) -> Option<(i64, &[Token])> {
        let (n, es) = parse_factor(es)?;
        if let Some((_, es)) = parse_token(Token::Plus)(es) {
            let (n2, es) = parse_term(es)?;
            Some((n + n2, es))
        } else {
            Some((n, es))
        }
    }

    fn parse_expr(es: &[Token]) -> Option<(i64, &[Token])> {
        let (n, es) = parse_term(es)?;
        if let Some((_, es)) = parse_token(Token::Mult)(es) {
            let (n2, es) = parse_expr(es)?;
            Some((n * n2, es))
        } else {
            Some((n, es))
        }
    }
}

fn main() {
    let exprs: Vec<_> = aoc_2020::stdin_lines()
        .map(|ln| ln.ok().and_then(|ln| tokenize(&ln)))
        .collect::<Option<_>>()
        .unwrap();

    let x = part1::solve(&exprs);
    println!("part1: {}", x);

    let x = part2::solve(&exprs);
    println!("part2: {}", x);
}
