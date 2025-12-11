use std::fs;

struct State {
    operations: Vec<char>,
    nums: Vec<Vec<u16>>,
}

impl State {
    fn from_p1(s: &str) -> Self {
        let mut lines = s.lines().map(|l| l.split_whitespace()).rev();
        State {
            operations: lines
                .next()
                .into_iter()
                .flat_map(|opt| opt.map(|c| c.parse().unwrap()))
                .collect(),
            nums: lines
                .map(|li| li.into_iter().map(|n| n.parse().unwrap()).collect())
                .collect(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input/real.txt").unwrap();

    let state_p1 = State::from_p1(&input);
    let p1_res = part1(&state_p1);
    let p2_res = part2(&input);
    println!("part1: {}", p1_res);
    println!("part2: {}", p2_res);
    assert!(p1_res == 6417439773370);
    assert!(p2_res == 11044319475191);
}

fn part1(state: &State) -> u64 {
    let row_count = state.nums.len();
    let mut sum = 0;
    for (c, &op) in state.operations.iter().enumerate() {
        if op == '*' {
            let mut col_total = 1;
            for r in 0..row_count {
                col_total *= state.nums[r][c] as u64;
            }
            sum += col_total;
        } else {
            let mut col_total = 0;
            for r in 0..row_count {
                col_total += state.nums[r][c] as u64;
            }
            sum += col_total;
        }
    }
    sum
}

fn part2(s: &str) -> u64 {
    // if you're reading this, code quality went out the window when they asked
    // such a silly question
    let lines: Vec<&str> = s.lines().collect();
    let operations: Vec<char> = lines[lines.len() - 1]
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();
    let lines = &lines[0..lines.len() - 1];
    let max_line_len = lines.iter().map(|l| l.len()).max().unwrap();
    let num_lines = lines.len();
    let mut col_nums: Vec<Vec<u32>> = Vec::with_capacity(operations.len());
    let mut cur_nums: Vec<u32> = Vec::new();
    for c in 0..max_line_len {
        let mut num = 0;
        let mut multiplier = 1;
        for r in (0..num_lines).rev() {
            let ch = lines[r].chars().nth(c).unwrap_or(' ');
            if ch != ' ' {
                let n: u32 = ch.to_digit(10).unwrap();
                num += n * multiplier;
                multiplier *= 10;
            }
        }
        if num != 0 {
            // we've built a number from this column, add it to the list
            cur_nums.push(num);
        } else {
            // this column was all empty so this set of numbers is done. wrap it up
            col_nums.push(cur_nums);
            cur_nums = Vec::new();
        }
    }
    // dump the last set to cur_nums
    col_nums.push(cur_nums);

    let mut sum = 0;
    for (i, num_set) in col_nums.iter().enumerate() {
        if operations[i] == '*' {
            let mut col_total = 1;
            for n in num_set {
                col_total *= *n as u64;
            }
            sum += col_total;
        } else {
            let mut col_total = 0;
            for n in num_set {
                col_total += *n as u64;
            }
            sum += col_total;
        }
    }
    sum
}
