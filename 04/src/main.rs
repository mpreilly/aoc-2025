use std::fs;

type Grid = Vec<Vec<char>>;

fn main() {
    let input = fs::read_to_string("input/real.txt").expect("file read fail");
    let mut grid: Grid = input.lines().map(|l| l.chars().collect()).collect();

    println!("part 1: {}", part1(&mut grid));
    println!("part 2: {}", part2(&mut grid));
}

fn part1(grid: &mut Grid) -> u32 {
    count_removable(grid, false)
}

fn part2(grid: &mut Grid) -> u32 {
    let mut total_removed = 0;
    let mut removed = count_removable(grid, true);
    total_removed += removed;
    while removed > 0 {
        removed = count_removable(grid, true);
        total_removed += removed;
    }
    total_removed
}

fn count_removable(grid: &mut Grid, remove: bool) -> u32 {
    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '.' {
                continue;
            }
            if can_access(&grid, r, c) {
                count += 1;
                if remove {
                    grid[r][c] = '.';
                }
            }
        }
    }
    count
}

fn can_access(grid: &Grid, r: usize, c: usize) -> bool {
    let mut adjacent_rolls: u32 = 0;
    if r > 0
        && let Some(row) = grid.get(r - 1)
    {
        if c > 0
            && let Some(val) = row.get(c - 1)
        {
            inc(&mut adjacent_rolls, val);
        }
        if let Some(val) = row.get(c) {
            inc(&mut adjacent_rolls, val);
        }
        if let Some(val) = row.get(c + 1) {
            inc(&mut adjacent_rolls, val);
        }
    }
    if let Some(row) = grid.get(r + 1) {
        if c > 0
            && let Some(val) = row.get(c - 1)
        {
            inc(&mut adjacent_rolls, val);
        }
        if let Some(val) = row.get(c) {
            inc(&mut adjacent_rolls, val);
        }
        if let Some(val) = row.get(c + 1) {
            inc(&mut adjacent_rolls, val);
        }
    }
    let row = &grid[r];
    if c > 0
        && let Some(val) = row.get(c - 1)
    {
        inc(&mut adjacent_rolls, val);
    }
    if let Some(val) = row.get(c + 1) {
        inc(&mut adjacent_rolls, val);
    }
    adjacent_rolls < 4
}

fn inc(counter: &mut u32, val: &char) {
    if *val == '@' {
        *counter += 1
    }
}
