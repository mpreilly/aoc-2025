use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn from(s: &str) -> Self {
        let mut parts = s.split(",");
        Point {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
        }
    }

    fn rect_area(&self, other: &Point) -> i64 {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

struct Range(i64, i64);

impl Range {
    fn from(a: i64, b: i64) -> Self {
        if a > b { Range(b, a) } else { Range(a, b) }
    }

    fn contains(&self, x: i64) -> bool {
        // start & end both inclusive!
        return x >= self.0 && x <= self.1;
    }
}

#[derive(Default)]
struct Ranges(Vec<Range>);

impl Ranges {
    fn add(&mut self, a: i64, b: i64) {
        self.0.push(Range::from(a, b))
    }

    fn any_contains(&self, x: i64) -> bool {
        for r in &self.0 {
            if r.contains(x) {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    let toy = false;
    let input_path = if toy {
        "input/toy.txt"
    } else {
        "input/real.txt"
    };
    let input = fs::read_to_string(input_path).unwrap();
    let red_points: Vec<Point> = input.lines().map(|l| Point::from(l)).collect();

    println!("part1: {}", part1(&red_points));
    println!("part2: {}", part2(&red_points));
}

fn part1(red_points: &Vec<Point>) -> i64 {
    get_all_unique_point_pairs(red_points)
        .iter()
        .map(|(p1, p2)| p1.rect_area(p2))
        .max()
        .unwrap()
}

fn part2(red_points: &Vec<Point>) -> i64 {
    let green_ranges_per_row = build_green_ranges_per_row(&red_points);
    get_all_unique_point_pairs(red_points)
        .par_iter()
        .filter(|(p1, p2)| are_all_points_within_green(p1, p2, &green_ranges_per_row))
        .map(|(p1, p2)| p1.rect_area(p2))
        .max()
        .unwrap()
}

fn get_all_unique_point_pairs(points: &Vec<Point>) -> Vec<(&Point, &Point)> {
    (0..points.len())
        .flat_map(|i| (i..points.len()).map(move |j| (i, j)))
        .map(|(i1, i2)| (points.get(i1).unwrap(), points.get(i2).unwrap()))
        .collect()
}

fn build_green_ranges_per_row(red_points: &Vec<Point>) -> HashMap<i64, Ranges> {
    let points_len = red_points.len();
    let mut row_green_ranges: HashMap<i64, Ranges> = HashMap::new();
    let mut vert_points: HashSet<Point> = HashSet::new();
    let connected_pairs: Vec<(&Point, &Point)> = (0..points_len)
        .map(|i| {
            if i == points_len - 1 {
                (red_points.get(i).unwrap(), red_points.get(0).unwrap())
            } else {
                (red_points.get(i).unwrap(), red_points.get(i + 1).unwrap())
            }
        })
        .collect();
    connected_pairs.into_iter().for_each(|(p1, p2)| {
        if p1.y == p2.y {
            // same row; just add start and end
            vert_points.insert(Point { x: p1.x, y: p1.y });
            vert_points.insert(Point { x: p2.x, y: p2.y });
        } else {
            // must have same x; same col.
            // sort range and go from smaller to larger
            let range = Range::from(p1.y, p2.y);
            for row in range.0..=range.1 {
                vert_points.insert(Point { x: p1.x, y: row });
            }
        }
    });
    // fill in row ranges from vert points
    let max_x = red_points.iter().map(|p| p.x).max().unwrap();
    vert_points.iter().for_each(|p| {
        // scan right, looking for another vert line to end on
        for c in (p.x + 1)..=max_x {
            if vert_points.contains(&Point { x: c, y: p.y }) {
                // we've found a full line. fill in this range
                let x_ranges = row_green_ranges.entry(p.y).or_default();
                x_ranges.add(p.x, c);
                return; // stop searching. if there's more, it'll be handled
            }
        }
    });
    row_green_ranges
}

fn are_all_points_within_green(
    p1: &Point,
    p2: &Point,
    green_ranges_per_row: &HashMap<i64, Ranges>,
) -> bool {
    let (smaller_r, larger_r) = if p1.y > p2.y {
        (p2.y, p1.y)
    } else {
        (p1.y, p2.y)
    };
    let (smaller_c, larger_c) = if p1.x > p2.x {
        (p2.x, p1.x)
    } else {
        (p1.x, p2.x)
    };

    // just check edges.
    let top_row_ranges = green_ranges_per_row.get(&smaller_r);
    let bottom_row_ranges = green_ranges_per_row.get(&larger_r);
    let top_bottom_tiles_good = (smaller_c..=larger_c).into_par_iter().map(move |c| {
        top_row_ranges.map_or(false, |ranges| ranges.any_contains(c))
            && bottom_row_ranges.map_or(false, |ranges| ranges.any_contains(c))
    });

    let left_right_tiles_good = (smaller_r..=larger_r).into_par_iter().map(move |r| {
        green_ranges_per_row.get(&r).map_or(false, |ranges| {
            ranges.any_contains(smaller_c) && ranges.any_contains(larger_c)
        })
    });

    top_bottom_tiles_good.all(|b| b) && left_right_tiles_good.all(|b| b)
}
