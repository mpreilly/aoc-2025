use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn from(s: &str) -> Self {
        let mut parts = s.split(",");
        Point {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        }
    }

    fn dist(&self, b: &Point) -> f64 {
        (((self.x - b.x).pow(2) + (self.y - b.y).pow(2) + (self.z - b.z).pow(2)) as f64).sqrt()
    }
}

struct Pair {
    a: usize,
    b: usize,
    dist: f64,
}

impl Pair {
    fn from(a: usize, b: usize, points: &Vec<Point>) -> Self {
        Pair {
            a,
            b,
            dist: points[a].dist(&points[b]),
        }
    }
}

fn main() {
    let toy = false;
    let (input_path, p1_connection_limit) = if toy {
        ("input/toy.txt", 10)
    } else {
        ("input/real.txt", 1000)
    };
    let input = fs::read_to_string(input_path).unwrap();
    let points: Vec<Point> = input.lines().map(|l| Point::from(l)).collect();

    let pairs = get_sorted_pairs(&points);

    let mut box_group_map: HashMap<usize, usize> = HashMap::new();
    let mut groups: Vec<HashSet<usize>> = Vec::new();
    let mut connections_made = 0;

    for &Pair { a, b, .. } in &pairs {
        make_connection(a, b, &mut box_group_map, &mut groups);
        part1_check(&mut connections_made, p1_connection_limit, &groups);
        if let Some(result) = part2_check(a, b, &groups, &points) {
            println!("part2: {}", result);
            return;
        }
    }
}

fn get_sorted_pairs(points: &Vec<Point>) -> Vec<Pair> {
    let points_count = points.len();

    let mut pairs: Vec<Pair> = (0..points_count)
        .flat_map(|i| (i + 1..points_count).map(move |j| (i, j)))
        .map(|(a, b)| Pair::from(a, b, &points))
        .collect();

    pairs.sort_by(|a, b| a.dist.total_cmp(&b.dist));

    pairs
}

fn make_connection(
    a: usize,
    b: usize,
    box_group_map: &mut HashMap<usize, usize>,
    groups: &mut Vec<HashSet<usize>>,
) {
    match (box_group_map.get(&a), box_group_map.get(&b)) {
        (Some(&a_group), Some(&b_group)) => {
            if a_group == b_group {
                return;
            }
            // move all of b's group into a's group now that they're connected
            let b_group_set = groups[b_group].clone();
            b_group_set.iter().for_each(|x| {
                box_group_map.insert(*x, a_group);
            });
            groups[a_group].extend(b_group_set);
            groups[b_group].clear();
        }
        (Some(&a_group), None) => {
            // b can join a's group!
            box_group_map.insert(b, a_group);
            groups[a_group].insert(b);
        }
        (None, Some(&b_group)) => {
            // a can join b's group!
            box_group_map.insert(a, b_group);
            groups[b_group].insert(a);
        }
        _ => {
            // new group together!
            let next_ind = groups.len();
            let mut group_set: HashSet<usize> = HashSet::new();
            group_set.insert(a);
            group_set.insert(b);
            groups.push(group_set);
            box_group_map.insert(a, next_ind);
            box_group_map.insert(b, next_ind);
        }
    }
}

fn part1_check(connections_made: &mut usize, limit: usize, groups: &Vec<HashSet<usize>>) {
    *connections_made += 1;
    if *connections_made == limit {
        let mut group_sizes: Vec<usize> = groups.iter().map(|s| s.len()).collect();
        group_sizes.sort_by(|a, b| b.cmp(a));

        let result: usize = group_sizes[0] * group_sizes[1] * group_sizes[2];

        println!("part1: {}", result);
    }
}

fn part2_check(
    a: usize,
    b: usize,
    groups: &Vec<HashSet<usize>>,
    points: &Vec<Point>,
) -> Option<i64> {
    for g in groups {
        if g.len() == points.len() {
            return Some(points[a].x * points[b].x);
        }
    }
    None
}
