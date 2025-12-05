fn main() {
    let input = include_str!("../../data/day5.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let ranges_input = parts[0];
    let food_input = parts[1];

    let mut ranges = Vec::new();
    for range in ranges_input.lines() {
        let ids = range
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        ranges.push((ids[0], ids[1]));
    }

    let mut result = 0;
    for food in food_input.lines() {
        let id: usize = food.parse().unwrap();

        for (l, u) in ranges.iter() {
            if *l <= id && id <= *u {
                result += 1;
                break;
            }
        }
    }
    println!("Part 1: {}", result);
}

fn part2(input: &str) {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let ranges_input = parts[0];

    let mut ranges = Vec::new();
    for range in ranges_input.lines() {
        let ids = range
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        insert_unique((ids[0], ids[1]), &mut ranges);
    }

    let mut result = 0;
    for (l, u) in ranges {
        result += u - l + 1;
    }
    println!("Part 2: {}", result);
}

fn insert_unique(range: (usize, usize), ranges: &mut Vec<(usize, usize)>) {
    let mut segments = split(vec![range], ranges);
    ranges.append(&mut segments);
}

fn split(
    segments: Vec<(usize, usize)>,
    remaining_ranges: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    if remaining_ranges.len() == 0 {
        return segments;
    }

    let mut next_segments = Vec::new();
    let (ex_lower, ex_upper) = remaining_ranges[0];
    for (cur_lower, cur_upper) in segments {
        // Excludion range fully contains the current range
        if ex_lower <= cur_lower && cur_upper <= ex_upper {
            continue;
        }

        // Exclusion range does not intersect the current range
        if cur_upper < ex_lower || ex_upper < cur_lower {
            next_segments.push((cur_lower, cur_upper));
            continue;
        }

        // Exclusion range splits current range into two segments
        if cur_lower < ex_lower && ex_upper < cur_upper {
            next_segments.push((cur_lower, ex_lower - 1));
            next_segments.push((ex_upper + 1, cur_upper));
            continue;
        }

        // Exclusion range trims upper part of current range
        if ex_lower <= cur_upper && ex_lower > cur_lower {
            next_segments.push((cur_lower, ex_lower - 1));
            continue;
        }

        // Exclusion range trims lower part of current range
        if ex_upper >= cur_lower && ex_upper < cur_upper {
            next_segments.push((ex_upper + 1, cur_upper));
            continue;
        }
    }

    split(next_segments, &remaining_ranges[1..])
}
