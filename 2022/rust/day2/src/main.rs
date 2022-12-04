use std::cmp::Ordering;

const TEST_INPUT: &str = "A Y
B X
C Z";

const INPUT: &str = include_str!("../input.txt");

fn part1(input: &str) -> u32 {
    let mut score = 0;

    input.lines().for_each(|v| {
        score += match v {
            "A X" => 3 + 1, // rock + rock = draw
            "A Y" => 6 + 2, // rock + paper = win
            "A Z" => 0 + 3, // rock + scissors = loss
            "B X" => 0 + 1, // paper + rock = loss
            "B Y" => 3 + 2, // paper + paper = draw
            "B Z" => 6 + 3, // paper + scissors = win
            "C X" => 6 + 1, // scissors + rock = win
            "C Y" => 0 + 2, // scissors + paper = loss
            "C Z" => 3 + 3, // scissors + scissors = draw
            _ => 0,
        }
    });
    score
}

fn part2(input: &str) -> u32 {
    let mut score = 0;

    input.lines().for_each(|v| {
        score += match v {
            "A X" => 0 + 3, // rock, lose = scissors (3)
            "B X" => 0 + 1, // paper, lose = rock (1)
            "C X" => 0 + 2, // scissors, lose = paper (2)
            "A Y" => 3 + 1, // rock, draw = rock (1)
            "B Y" => 3 + 2, // paper, draw = paper (2)
            "C Y" => 3 + 3, // scissors, draw = scissors (3)
            "A Z" => 6 + 2, // rock, win = paper (2)
            "B Z" => 6 + 3, // paper, win = scissors (3)
            "C Z" => 6 + 1, // scissors, win = rock (1)
            _ => 0,
        }
    });
    score
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

// unused idea
#[derive(PartialEq)]
enum Rps {
    R,
    P,
    S,
}
impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Rps::*;
        Some(match (self, other) {
            (R, P) | (P, S) | (S, R) => Ordering::Less,
            (R, S) | (P, R) | (S, P) => Ordering::Greater,
            _ => Ordering::Equal,
        })
    }
}
