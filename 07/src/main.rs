use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Eq, Hash, PartialEq)]
struct Point {
    row: usize,
    col: usize,
}

struct State {
    start_beam: Point,
    splitters: HashSet<Point>,
    row_count: usize,
}

impl State {
    fn from(s: &str) -> Self {
        let mut start_beam: Option<Point> = None;
        let mut splitters = HashSet::new();
        let lines = s.lines();
        let mut row_count = 0;
        lines.enumerate().for_each(|(r, l)| {
            l.chars().enumerate().for_each(|(c, val)| {
                if val == 'S' {
                    start_beam = Some(Point { row: r + 1, col: c });
                } else if val == '^' {
                    splitters.insert(Point { row: r, col: c });
                }
            });
            row_count += 1;
        });
        State {
            start_beam: start_beam.expect("start point not found"),
            splitters,
            row_count,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input/real.txt").unwrap();
    let state = State::from(&input);

    let p1 = part1(&state);
    println!("part1: {}", p1);

    let p2 = part2(&state);
    println!("part2: {}", p2);
}

fn part1(state: &State) -> u32 {
    let start_row = state.start_beam.row;
    let mut beam_cols = HashSet::new();
    beam_cols.insert(state.start_beam.col);

    let mut split_count = 0;
    for r in start_row..state.row_count {
        let mut new_beam_cols = HashSet::new();
        for c in beam_cols {
            if state.splitters.contains(&Point { row: r + 1, col: c }) {
                split_count += 1;
                new_beam_cols.insert(c - 1);
                new_beam_cols.insert(c + 1);
            } else {
                new_beam_cols.insert(c);
            }
        }
        beam_cols = new_beam_cols;
    }

    split_count
}

fn part2(state: &State) -> u64 {
    // same thing, but keep track of how many paths converged on each col
    // so that we know how many different paths there were at the end
    let start_row = state.start_beam.row;
    let mut beam_cols = HashMap::new();
    beam_cols.insert(state.start_beam.col, 1);

    let mut path_count = 1;
    for r in start_row..state.row_count {
        let mut new_beam_cols = HashMap::new();
        for (c, count) in beam_cols {
            if state.splitters.contains(&Point { row: r + 1, col: c }) {
                path_count += count;
                let cur_left_path_count = new_beam_cols.entry(c - 1).or_insert(0);
                *cur_left_path_count += count;
                let cur_right_path_count = new_beam_cols.entry(c + 1).or_insert(0);
                *cur_right_path_count += count;
            } else {
                let cur_straight_path_count = new_beam_cols.entry(c).or_insert(0);
                *cur_straight_path_count += count;
            }
        }
        beam_cols = new_beam_cols;
    }

    path_count
}
