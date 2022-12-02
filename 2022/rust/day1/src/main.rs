const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut elves: Vec<u32> = INPUT
        .split("\r\n\r\n")
        .map(|v| v.split("\r\n").map(|v| v.parse::<u32>().unwrap()).sum())
        .collect();
    elves.sort();
    let top = elves[elves.len() - 1];
    let top_three: u32 = elves.iter().rev().take(3).sum();
    println!("Part 1: {:?}", top);
    println!("Part 2: {:?}", top_three);
}
