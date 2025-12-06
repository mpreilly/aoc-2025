use std::fs;

type Joltage = u32;
type Bank = Vec<Joltage>;

const NUM_DIGITS_PART1: usize = 2;
const NUM_DIGITS_PART2: usize = 12;

fn main() {
    let banks = parse_banks(false);

    println!("part1: {}", part1(&banks));
    println!("part2: {}", part2(&banks));
}

fn parse_banks(toy: bool) -> Vec<Bank> {
    let filepath = if toy {
        "input/toy.txt"
    } else {
        "input/real.txt"
    };
    let input = fs::read_to_string(filepath).expect("file read fail");
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("u16 parse failed"))
                .collect()
        })
        .collect()
}

fn part1(banks: &Vec<Bank>) -> u64 {
    banks
        .iter()
        .map(|b| max_joltage(b, &mut [0; NUM_DIGITS_PART1]))
        .sum()
}

fn part2(banks: &Vec<Bank>) -> u64 {
    banks
        .iter()
        .map(|b| max_joltage(b, &mut [0; NUM_DIGITS_PART2]))
        .sum()
}

fn max_joltage(bank: &Bank, digits: &mut [u32]) -> u64 {
    for i in 0..bank.len() {
        let chars_left = bank.len() - i;
        // if subtraction would be negative that means we have enough to replace all
        let lowest_index = digits.len().checked_sub(chars_left).unwrap_or(0);
        for d in lowest_index..digits.len() {
            if bank[i] > digits[d] {
                digits[d] = bank[i];
                // clear remaining digits bc we've used a later digit
                for d2 in (d + 1)..digits.len() {
                    digits[d2] = 0;
                }
                break; // we've used this digit, so move on
            }
        }
    }
    digits_to_num(&digits)
}

fn digits_to_num(digits: &[u32]) -> u64 {
    let mut result = 0;
    let mut place = 1;
    digits.iter().rev().for_each(|d| {
        result += (*d as u64) * place;
        place *= 10;
    });
    result
}
