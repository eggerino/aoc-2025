fn main() {
    let input = include_str!("../../data/day2.txt");
    part1(input);
    part2(input);
}

fn sum_ids<F>(input: &str, f: F) -> u64
where
    F: Fn(u64) -> bool,
{
    let mut count = 0;
    for r in input.trim_end().split(',').map(|x| {
        x.split('-')
            .map(|n| n.parse::<u64>().expect("number for range def"))
            .collect::<Vec<_>>()
    }) {
        for n in r[0]..=r[1] {
            if f(n) {
                count += n;
            }
        }
    }
    count
}

fn repeat_exact_twice(num: u64) -> bool {
    let string = num.to_string();
    string[..string.len() / 2] == string[string.len() / 2..]
}

fn repeat_atleast_twice(num: u64) -> bool {
    let string = num.to_string();
    for repeats in 2..=string.len() {
        if string.len() % repeats != 0 {
            continue;
        }

        let sub_len = string.len() / repeats;

        let first = &string[..sub_len];
        let is_true_repeat = (1..repeats)
            .map(|i| &string[i * sub_len..(i + 1) * sub_len])
            .all(|x| x == first);
        if is_true_repeat {
            return true;
        }
    }

    return false;
}

fn part1(input: &str) {
    let result = sum_ids(input, repeat_exact_twice);
    println!("Part 1: {}", result);
}

fn part2(input: &str) {
    let result = sum_ids(input, repeat_atleast_twice);
    println!("Part 2: {}", result);
}
