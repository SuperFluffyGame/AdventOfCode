use std::collections::HashSet;

const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

const INPUT: &str = include_str!("../input.txt");

fn get_prio(c: char) -> u8 {
    if c.is_lowercase() {
        (c as u8) - b'a' + 1
    } else {
        (c as u8) - b'A' + 27
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|sack| {
            let compartments = sack.split_at(sack.len() / 2);
            let comp_a: HashSet<char> = HashSet::from_iter(compartments.0.chars());
            let comp_b: HashSet<char> = HashSet::from_iter(compartments.1.chars());

            comp_a
                .intersection(&comp_b)
                .map(|char| get_prio(*char) as u32)
                .sum::<u32>()
        })
        .sum::<u32>()

    // todo!()
}

fn part2(input: &str) -> u32 {
    let mut groups: Vec<Vec<&str>> = Vec::new();
    input.lines().enumerate().for_each(|(i, sack)| {
        if i % 3 == 0 {
            groups.push(vec![sack])
        } else {
            groups[i / 3].push(sack)
        }
    });

    let commons = groups
        .iter()
        .map(|group| {
            let first: HashSet<char> = HashSet::from_iter(group.first().unwrap().chars());
            group.iter().skip(1).fold(first, |total, sack| {
                let this_set: HashSet<char> = HashSet::from_iter(sack.chars());
                total.intersection(&this_set).copied().collect()
            })
        })
        .collect::<Vec<_>>();
    commons
        .iter()
        .map(|set| get_prio(*set.iter().next().unwrap()) as u32)
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}
