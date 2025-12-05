const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

struct Grid {
    rows: Vec<Vec<bool>>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        Self {
            rows: input
                .lines()
                .map(|x| x.chars().map(|c| c == '@').collect())
                .collect(),
        }
    }

    fn row_count(&self) -> usize {
        self.rows.len()
    }

    fn col_count(&self) -> usize {
        self.rows[0].len()
    }

    fn is_pickable_at(&self, row: usize, col: usize) -> bool {
        if !self.rows[row][col] {
            return false;
        }

        let mut count = 0;
        for (dr, dc) in DIRS.iter() {
            let r = row as i32 + dr;
            let c = col as i32 + dc;

            if r < 0 || c < 0 || r >= self.row_count() as i32 || c >= self.col_count() as i32 {
                continue; // Out of grid
            }
            if self.rows[r as usize][c as usize] {
                count += 1;
            }
        }
        count < 4
    }

    fn pick_at(&mut self, row: usize, col: usize) {
        self.rows[row][col] = false;
    }
}

fn main() {
    let input = include_str!("../../data/day4.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let grid = Grid::parse(input);

    let mut result = 0;
    for row in 0..grid.row_count() {
        for col in 0..grid.col_count() {
            if grid.is_pickable_at(row, col) {
                result += 1;
            }
        }
    }

    println!("Part 1: {}", result);
}

fn part2(input: &str) {
    let mut grid = Grid::parse(input);

    let mut previous_run = -1;
    let mut result = 0;
    while previous_run != result {
        previous_run = result;

        for row in 0..grid.row_count() {
            for col in 0..grid.col_count() {
                if grid.is_pickable_at(row, col) {
                    result += 1;
                    grid.pick_at(row, col);
                }
            }
        }
    }

    println!("Part 2: {}", result);
}
