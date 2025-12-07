use std::collections::HashMap;

fn main() {
    let input = include_str!("../../data/day7.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let map = input.lines().collect::<Vec<_>>();

    let mut beams = vec![map[0].find('S').unwrap()];

    let mut split_count = 0;
    for row in map[1..].iter() {
        let mut next_beams = Vec::new();
        for beam in beams {
            if row.chars().nth(beam) == Some('^') {
                split_count += 1;
                push_unique_sorted(&mut next_beams, beam - 1);
                push_unique_sorted(&mut next_beams, beam + 1);
            } else {
                push_unique_sorted(&mut next_beams, beam);
            }
        }
        beams = next_beams;
    }

    println!("Part 1: {}", split_count);
}

fn push_unique_sorted(v: &mut Vec<usize>, val: usize) {
    if let Some(l) = v.last()
        && *l == val
    {
    } else {
        v.push(val);
    }
}

fn part2(input: &str) {
    let map = input.lines().collect::<Vec<_>>();
    let start = map[0].find('S').unwrap();
    let result = dp(0, start, &map, &mut HashMap::new());
    println!("Part 2: {}", result);
}

#[derive(Eq, PartialEq, Hash)]
struct State {
    row: usize,
    col: usize,
}

fn dp(row: usize, col: usize, map: &[&str], cache: &mut HashMap<State, usize>) -> usize {
    let current = State { row, col };
    if let Some(r) = cache.get(&current) {
        return *r;
    }

    let result = if row == map.len() - 1 {
        1
    } else {
        match map[row].chars().nth(col) {
            Some('^') => dp(row + 1, col - 1, map, cache) + dp(row + 1, col + 1, map, cache),
            _ => dp(row + 1, col, map, cache),
        }
    };

    cache.insert(current, result);
    result
}
