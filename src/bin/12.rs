#[derive(Debug, Clone, PartialEq)]
struct State {
    x: f64,
    y: f64,
    alpha: f64,
}

impl State {
    fn new(x: f64, y: f64, alpha: f64) -> Self {
        Self { x, y, alpha }
    }

    fn manhattan(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }
}

fn part1(insts: &[(char, f64)]) -> f64 {
    let mut pos = State::new(0., 0., 0.);
    for (dir, amount) in insts {
        match dir {
            'N' => pos.y += amount,
            'S' => pos.y -= amount,
            'E' => pos.x += amount,
            'W' => pos.x -= amount,
            'L' => pos.alpha += amount.to_radians(),
            'R' => pos.alpha -= amount.to_radians(),
            'F' => {
                pos.x += amount * pos.alpha.cos();
                pos.y += amount * pos.alpha.sin();
            }
            _ => panic!("invalid instruction: {}", dir),
        }
    }
    pos.manhattan()
}

fn part2(insts: &[(char, f64)]) -> f64 {
    let mut ship = State::new(0., 0., 0.);
    let mut waypoint = State::new(10., 1., 0.);
    for &(dir, amount) in insts {
        match dir {
            'N' => waypoint.y += amount,
            'S' => waypoint.y -= amount,
            'E' => waypoint.x += amount,
            'W' => waypoint.x -= amount,
            'L' | 'R' => {
                let theta = if dir == 'L' { amount } else { -amount };
                let State { x, y, .. } = waypoint;
                let (sin, cos) = theta.to_radians().sin_cos();
                waypoint.x = x * cos - y * sin;
                waypoint.y = x * sin + y * cos;
            }
            'F' => {
                ship.x += waypoint.x * amount;
                ship.y += waypoint.y * amount;
            }
            _ => panic!("invalid instruction: {}", dir),
        }
    }
    ship.manhattan()
}

fn main() {
    let insts: Vec<_> = aoc_2020::stdin_lines()
        .filter_map(|s| {
            s.ok().and_then(|s| {
                let dir = s.chars().next()?;
                let amount: u32 = (&s[1..]).parse().ok()?;
                Some((dir, amount as f64))
            })
        })
        .collect();

    let x = part1(&insts);
    println!("part1: {}", x);

    let x = part2(&insts);
    println!("part2: {}", x);
}
