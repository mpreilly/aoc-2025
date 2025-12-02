use std::fs;

struct Rotation {
    is_right: bool,
    clicks: i32,
}

struct Dial(i32);

impl Dial {
    fn turn(&mut self, rotation: &Rotation) {
        let sign = if rotation.is_right { 1 } else { -1 };
        self.0 = (self.0 + (sign * rotation.clicks)) % 100;
    }
}

fn main() {
    let input = fs::read_to_string("input/real.txt").expect("file read fail");
    let rotations = parse_rotations(&input);

    // let mut dial = Dial(50);
    // let mut zero_count = 0;
    // rotations.iter().for_each(|r| {
    //     dial.turn(r);
    //     if dial.0 == 0 {
    //         zero_count += 1;
    //     }
    // });

    // part 2
    let mut dial = 50;
    let mut zero_count = 0;
    rotations.iter().for_each(|r| {
        let sign = if r.is_right { 1 } else { -1 };
        // let start_hundo = dial / 100;
        // let start_dial_sign = dial > 0;
        // dial = dial + (sign * r.clicks);
        // let end_hundo = dial / 100;
        // let sign_bonus = if start_dial_sign != (dial >= 0) { 1 } else { 0 };
        // zero_count += (end_hundo - start_hundo).abs() + sign_bonus
        for _ in 0..r.clicks {
            dial = (dial + 1 * sign) % 100;
            if dial == 0 {
                zero_count += 1;
            }
        }
    });

    println!("{}", zero_count)
}

fn parse_rotations(input: &str) -> Vec<Rotation> {
    input.lines().map(line_to_rotation).collect()
}

fn line_to_rotation(line: &str) -> Rotation {
    if line.len() < 2 {
        panic!("bad input: line too small")
    }
    let mut chars = line.chars();
    Rotation {
        is_right: chars.next().expect("empty line") == 'R',
        clicks: chars
            .collect::<String>()
            .parse()
            .expect("couldn't parse int clicks"),
    }
}
