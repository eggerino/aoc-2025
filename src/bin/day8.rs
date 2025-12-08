use std::collections::HashMap;

fn main() {
    let input = include_str!("../../data/day8.txt");
    let n = 1000;
    part1(input, n);
    part2(input);
}

#[derive(Debug)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn parse(line: &str) -> Option<Self> {
        match &(line.split(',').map(|x| x.parse()).collect::<Vec<_>>())[..] {
            &[Ok(x), Ok(y), Ok(z)] => Some(Self { x, y, z }),
            _ => None,
        }
    }

    fn distance_sqr(&self, other: &Self) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        dx * dx + dy * dy + dz * dz
    }
}

fn part1(input: &str, n: usize) {
    let boxes = input
        .lines()
        .map(|x| Pos::parse(x).unwrap())
        .collect::<Vec<_>>();
    let pairs = &get_shortest_pairs(&boxes)[..n];
    let mut circ = boxes.iter().map(|_| None).collect::<Vec<_>>();

    for (i, (first, second, _)) in pairs.iter().enumerate() {
        match (circ[*first], circ[*second]) {
            (None, None) => {
                circ[*first] = Some(i);
                circ[*second] = Some(i);
            }
            (Some(c), None) => {
                circ[*second] = Some(c);
            }
            (None, Some(c)) => circ[*first] = Some(c),
            (Some(f), Some(s)) if f == s => (),
            (Some(f), Some(s)) => {
                circ.iter_mut().for_each(|c| {
                    if let Some(x) = c
                        && *x == s
                    {
                        c.replace(f);
                    }
                });
            }
        }
    }

    let mut counts = HashMap::new();
    for c in circ.iter() {
        if let Some(c_id) = c {
            match counts.get(c_id) {
                Some(cur) => counts.insert(*c_id, cur + 1),
                None => counts.insert(*c_id, 1),
            };
        }
    }

    let mut counts_sort = counts.into_values().collect::<Vec<_>>();
    counts_sort.sort_by(|l, r| r.cmp(l));

    let result = counts_sort[0] * counts_sort[1] * counts_sort[2];
    println!("Part 1: {}", result);
}

fn get_shortest_pairs(boxes: &[Pos]) -> Vec<(usize, usize, i64)> {
    let mut vec = Vec::with_capacity(boxes.len() * boxes.len() / 2);
    for first in 0..boxes.len() - 1 {
        for second in first + 1..boxes.len() {
            let dist = boxes[first].distance_sqr(&boxes[second]);
            vec.push((first, second, dist));
        }
    }
    vec.sort_by(|l, r| l.2.cmp(&r.2));
    vec
}

fn part2(input: &str) {
    let boxes = input
        .lines()
        .map(|x| Pos::parse(x).unwrap())
        .collect::<Vec<_>>();
    let pairs = get_shortest_pairs(&boxes);
    let mut circ = boxes.iter().map(|_| None).collect::<Vec<_>>();
    let mut circ_count = 0;

    for (i, (first, second, _)) in pairs.iter().enumerate() {
        match (circ[*first], circ[*second]) {
            (None, None) => {
                circ[*first] = Some(i);
                circ[*second] = Some(i);
                circ_count += 1;
            }
            (Some(c), None) => circ[*second] = Some(c),
            (None, Some(c)) => circ[*first] = Some(c),
            (Some(f), Some(s)) if f == s => (),
            (Some(f), Some(s)) => {
                circ.iter_mut().for_each(|c| {
                    if let Some(x) = c
                        && *x == s
                    {
                        c.replace(f);
                    }
                });
                circ_count -= 1;
            }
        }

        if circ_count == 1 && circ.iter().all(Option::is_some) {
            let result = boxes[*first].x * boxes[*second].x;
            println!("Part 2: {}", result);
            return;
        }
    }
}
