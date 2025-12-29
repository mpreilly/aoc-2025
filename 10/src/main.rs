use std::{collections::VecDeque, fs};

const LARGEST_LIGHT_COUNT: usize = 10;

struct Machine {
    goal_state: [bool; LARGEST_LIGHT_COUNT],
    buttons: Vec<Button>,
    joltage_requirements: Vec<u16>,
}

impl Machine {
    fn from(s: &str) -> Self {
        let mut bracket_split = s[1..].split("]");
        let lights_part = bracket_split.next().unwrap();
        let mut goal_state = [false; LARGEST_LIGHT_COUNT];
        for (i, c) in lights_part.chars().enumerate() {
            if c == '#' {
                goal_state[i] = true
            }
        }

        let mut curly_split = bracket_split.next().unwrap().split("{");
        let buttons_part = curly_split.next().unwrap();
        let buttons: Vec<Button> = buttons_part
            .trim()
            .split_whitespace()
            .map(|s| Button::from(s))
            .collect();

        let joltage_part = curly_split.next().unwrap();
        let no_curly = &joltage_part[..joltage_part.len() - 1];
        let joltage_requirements: Vec<u16> =
            no_curly.split(",").map(|n| n.parse().unwrap()).collect();

        Machine {
            goal_state,
            buttons,
            joltage_requirements,
        }
    }

    fn presses_required(&self) -> u32 {
        let mut frontier: VecDeque<SearchNode> = VecDeque::new();
        frontier.push_back(SearchNode {
            state: [false; LARGEST_LIGHT_COUNT],
            presses: 0,
        });

        while let Some(node) = frontier.pop_front() {
            if node.state == self.goal_state {
                return node.presses;
            }
            for b in &self.buttons {
                frontier.push_back(SearchNode {
                    state: b.press(node.state),
                    presses: node.presses + 1,
                });
            }
        }

        panic!("correct configuration not found!")
    }
}

struct SearchNode {
    state: [bool; LARGEST_LIGHT_COUNT],
    presses: u32,
}

struct Button {
    lights_toggled: Vec<u16>,
}

impl Button {
    fn from(s: &str) -> Self {
        let no_parens = &s[1..(s.len() - 1)];
        Button {
            lights_toggled: no_parens.split(",").map(|n| n.parse().unwrap()).collect(),
        }
    }

    fn press(&self, mut state: [bool; LARGEST_LIGHT_COUNT]) -> [bool; LARGEST_LIGHT_COUNT] {
        for i in &self.lights_toggled {
            state[*i as usize] = !state[*i as usize]
        }
        return state;
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
    let mut machines: Vec<Machine> = input.lines().map(|l| Machine::from(l)).collect();

    println!("part1: {}", part1(&mut machines));
}

fn part1(machines: &mut Vec<Machine>) -> u32 {
    machines.iter_mut().map(|m| m.presses_required()).sum()
}
