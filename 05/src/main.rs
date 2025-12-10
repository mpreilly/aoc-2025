use std::{cmp, fmt, fs};

struct State {
    fresh_ranges: Vec<Range>,
    ingredients: Vec<u64>,
}

impl State {
    fn from(s: &str) -> Self {
        let mut parts = s.split("\n\n");
        let range_part = parts.next().unwrap();
        let ingredient_part = parts.next().unwrap();
        let raw_ranges: Vec<Range> = range_part
            .trim()
            .split("\n")
            .map(|l| Range::from(l))
            .collect();

        State {
            fresh_ranges: Self::merge_ranges(raw_ranges),
            ingredients: ingredient_part
                .trim()
                .split("\n")
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }

    fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
        let mut cur_ranges: Vec<Option<Range>> = ranges.into_iter().map(|r| Some(r)).collect();
        let range_count = cur_ranges.len();
        let mut ranges_changed = true;
        while ranges_changed {
            ranges_changed = false;
            // for each range, consider each range after it (so each pair is done just once)
            // check if they can be merged, throwing out the ones that are merged into current
            for i in 0..range_count {
                let Some(mut cur_r) = cur_ranges[i].clone() else {
                    continue;
                };
                for j in (i + 1)..range_count {
                    let Some(other_r) = cur_ranges[j].clone() else {
                        continue;
                    };
                    if (other_r.start <= cur_r.start && other_r.end >= cur_r.start)
                        || (other_r.end >= cur_r.end && other_r.start <= cur_r.end)
                    {
                        // replace cur with merged range
                        cur_r = Range {
                            start: cmp::min(other_r.start, cur_r.start),
                            end: cmp::max(other_r.end, cur_r.end),
                        };
                        cur_ranges[i] = Some(cur_r.clone());
                        // replace other with None because we used it up
                        cur_ranges[j] = None;
                        ranges_changed = true;
                    } else if cur_r.start <= other_r.start && cur_r.end >= other_r.end {
                        // other is dominated by cur
                        cur_ranges[j] = None;
                        ranges_changed = true;
                    }
                }
            }
        }
        cur_ranges.into_iter().flatten().collect()
    }
}

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn from(s: &str) -> Range {
        let mut parts = s.split("-");
        Range {
            start: parts.next().unwrap().parse().unwrap(),
            end: parts.next().unwrap().parse().unwrap(),
        }
    }

    fn contains(&self, val: u64) -> bool {
        val >= self.start && val <= self.end
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

fn main() {
    let input = fs::read_to_string("input/real.txt").unwrap();
    let state = State::from(&input);

    println!("part1: {}", part1(&state));
    println!("part2: {}", part2(&state));

    // assert!(part1(&state) == 811);
    // assert!(part2(&state) == 338189277144473);
}

fn part1(state: &State) -> usize {
    state
        .ingredients
        .iter()
        .filter(|&&n| is_fresh(n, &state.fresh_ranges))
        .count()
}

fn is_fresh(ingredient: u64, fresh_ranges: &Vec<Range>) -> bool {
    for r in fresh_ranges {
        if r.contains(ingredient) {
            return true;
        }
    }
    false
}

fn part2(state: &State) -> u64 {
    state.fresh_ranges.iter().map(|r| r.end - r.start + 1).sum()
}
