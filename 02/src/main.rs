use std::{fs, str::Split};

struct Range {
    start: u64,
    end_inc: u64,
}

impl Range {
    fn from(range: &str) -> Self {
        let mut vals: Split<&str> = range.split("-");
        Range {
            start: vals
                .next()
                .expect("didn't find start of range")
                .parse()
                .expect("couldn't parse start of range"),
            end_inc: vals
                .next()
                .expect("didn't find end of range")
                .parse()
                .expect("couldn't parse end of range"),
        }
    }
}

fn main() {
    let ranges = parse_ranges(false);
    println!("part 1: {}", part1(&ranges));
    println!("part 2: {}", part2(&ranges));
}

fn parse_ranges(toy: bool) -> Vec<Range> {
    let filepath = if toy {
        "input/toy.txt"
    } else {
        "input/real.txt"
    };
    let input = fs::read_to_string(filepath).expect("file read fail");
    input.trim().split(",").map(|r| Range::from(r)).collect()
}

fn part1(ranges: &Vec<Range>) -> u64 {
    get_sum_of_invalid_ids(ranges, is_invalid_p1)
}

fn part2(ranges: &Vec<Range>) -> u64 {
    get_sum_of_invalid_ids(ranges, is_invalid_p2)
}

fn get_sum_of_invalid_ids(ranges: &Vec<Range>, is_invalid: fn(&str) -> bool) -> u64 {
    ranges
        .into_iter()
        .flat_map(|r| r.start..=r.end_inc)
        .filter(|id| is_invalid(&id.to_string()))
        .sum()
}

fn is_invalid_p1(id: &str) -> bool {
    let len = id.len();
    if len % 2 != 0 {
        false
    } else {
        let (s1, s2) = id.split_at(len / 2);
        s1 == s2
    }
}

fn is_invalid_p2(id: &str) -> bool {
    for pat_len in 1..=(id.len() / 2) {
        let pattern = &id[0..pat_len];
        if is_string_all_repeats_of_pattern(id, pattern) {
            return true;
        }
    }
    false
}

fn is_string_all_repeats_of_pattern(s: &str, pat: &str) -> bool {
    let pat_len = pat.len();
    if s.len() % pat_len != 0 {
        return false;
    }
    let mut start = pat_len;
    let mut end = start + pat_len;
    while end <= s.len() {
        if &s[start..end] != pat {
            return false;
        }
        start = end;
        end = start + pat_len;
    }
    true
}
