use std::fs;

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
}

fn part1(red_points: &Vec<Point>) -> i64 {
    let points_len = red_points.len();
    let unique_index_pairs: Vec<(usize, usize)> = (0..points_len)
        .flat_map(|i| (i..points_len).map(move |j| (i, j)))
        .collect();
    unique_index_pairs
        .iter()
        .map(|(i1, i2)| {
            red_points
                .get(*i1)
                .unwrap()
                .rect_area(red_points.get(*i2).unwrap())
        })
        .max()
        .unwrap()
}
