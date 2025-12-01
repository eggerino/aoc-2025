enum Instruction {
    Left(i32),
    Right(i32),
}

impl Instruction {
    fn parse(line: &str) -> Option<Self> {
        match (&line[..1], line[1..].parse::<i32>()) {
            ("R", Ok(n)) => Some(Self::Right(n)),
            ("L", Ok(n)) => Some(Self::Left(n)),
            _ => None,
        }
    }

    fn apply(&self, cur: i32) -> i32 {
        match self {
            Self::Right(n) => (cur + n) % 100,
            Self::Left(n) => (cur - n + 10_000) % 100, // Assume no number is bigger than 10_000
        }
    }

    fn passes_zero(&self, cur: i32) -> i32 {
        match self {
            Self::Right(n) => (cur + n) / 100,
            Self::Left(n) => match cur {
                0 => (100 - cur + n) / 100 - 1,
                _ => (100 - cur + n) / 100,
            },
        }
    }
}

fn main() {
    let input = include_str!("../../data/day1.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let result = input
        .lines()
        .map(|x| Instruction::parse(x).expect("Invalid instruction"))
        .fold((50, 0), |(cur, count), inst| match inst.apply(cur) {
            0 => (0, count + 1),
            x => (x, count),
        })
        .1;

    println!("Part 1: {result}");
}

fn part2(input: &str) {
    let result = input
        .lines()
        .map(|x| Instruction::parse(x).expect("Invalid instruction"))
        .fold((50, 0), |(cur, count), inst| {
            (inst.apply(cur), count + inst.passes_zero(cur))
        })
        .1;

    println!("Part 2: {result}");
}
