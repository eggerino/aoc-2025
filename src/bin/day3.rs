fn main() {
    let input = include_str!("../../data/day3.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let mut count = 0;
    for bank in input.lines().map(|x| {
        x.chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    }) {
        let mut i = 0;
        let mut first = bank[0];
        for idx in 0..bank.len() - 1 {
            if bank[idx] > first {
                i = idx;
                first = bank[idx];
            }
        }

        let mut second = 0;
        for idx in i + 1..bank.len() {
            if bank[idx] > second {
                second = bank[idx];
            }
        }

        count += first * 10 + second;
    }
    println!("Part 1: {}", count);
}

fn part2(input: &str) {
    let mut count = 0;
    for bank in input.lines().map(|x| {
        x.chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<_>>()
    }) {
        let cur = max_joltage(&bank, 12);
        count += cur;
    }
    println!("Part 2: {}", count);
}

fn max_joltage(batteries: &[u32], num: usize) -> usize {
    let mut cur = 0;

    let mut left = 0;
    for i in 0..num {
        let (peak, digit) = max_at(batteries, left, batteries.len() - num + i + 1);

        left = peak + 1;
        cur += (digit as usize) * (10_usize.pow((num - i - 1) as u32));
    }
    cur
}

fn max_at(nums: &[u32], start: usize, stop: usize) -> (usize, u32) {
    let mut idx = 0;
    let mut cur = 0;
    for i in start..stop {
        if nums[i] > cur {
            idx = i;
            cur = nums[i];
        }
    }
    (idx, cur)
}
