fn main() {
    let input = include_str!("../../data/day6.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let data = input
        .lines()
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = 0;
    for (i, op) in data.last().unwrap().iter().enumerate() {
        let mut cur = match *op {
            "*" => 1,
            _ => 0,
        };

        for row in data[..data.len() - 1].iter() {
            let value = row[i].parse::<usize>().unwrap();
            cur = match *op {
                "*" => cur * value,
                _ => cur + value,
            };
        }
        result += cur;
    }

    println!("Part 1: {}", result);
}

fn part2(input: &str) {
    let data = input.lines().collect::<Vec<_>>();

    let ops = data
        .last()
        .expect("Operator line is missing")
        .split_whitespace();

    let mut digit_matrix = Vec::new();
    for col in 0..data[0].len() {
        let mut cur = String::with_capacity(data.len() - 1);
        for row in 0..data.len() - 1 {
            cur.push(
                data[row]
                    .chars()
                    .nth(col)
                    .expect("Malformed input of number matrix"),
            );
        }
        digit_matrix.push(cur.trim().parse::<usize>().ok());
    }

    let mut result = 0;
    let mut ptr = 0;
    for op in ops {
        let mut cur = match op {
            "*" => 1,
            _ => 0,
        };

        while ptr < digit_matrix.len()
            && let Some(num) = digit_matrix[ptr]
        {
            ptr += 1;
            cur = match op {
                "+" => cur + num,
                _ => cur * num,
            };
        }
        ptr += 1;
        result += cur;
    }

    println!("Part 2: {}", result);
}

// data = open(0).read().splitlines()
// ops = data[-1].split()

// nums = []
// for digits in zip(*data[:-1]):
//     as_str = "".join(digits)
//     nums.append(int(as_str) if not as_str.isspace() else None)

// nums = iter(nums)
// result = 0

// for op in ops:
//     cur = 0 if op == "+" else 1

//     num = cur
//     while num != None:
//         cur = cur + num if op == "+" else cur * num
//         try:
//             num = next(nums)
//         except:
//             break

//     result += cur

// print(result)
