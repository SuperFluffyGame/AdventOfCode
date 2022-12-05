const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

const INPUT: &str = include_str!("../input.txt");

fn part1(input: &str) -> String {
    // parsing the first part (by hand)
    let (row_idx, num_row) = input
        .lines()
        .enumerate()
        .find(|(idx, line)| line.starts_with(" 1"))
        .unwrap();
    let num_stacks = num_row[(num_row.len() - 2)..(num_row.len() - 1)]
        .parse::<usize>()
        .unwrap();

    let mut stacks = vec![String::new(); num_stacks];

    let stack_lines = input.lines().take(row_idx).collect::<Vec<_>>();

    stack_lines.iter().for_each(|line| {
        for i in 0..num_stacks {
            let str_index = i / 4;
            dbg!(str_index);
            dbg!(line);
            let char = line.chars().nth(str_index + 1).unwrap();
            stacks[i].push(char);
        }
    });

    dbg!(stacks);
    todo!()
}

fn part2(input: &str) -> u32 {
    todo!()
}

fn main() {
    println!("Part 1: {}", part1(TEST_INPUT));
    println!("Part 2: {}", part2(INPUT));
}
