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
        .find(|(_idx, line)| line.starts_with(" 1"))
        .unwrap();
    let num_stacks = num_row[(num_row.len() - 2)..(num_row.len() - 1)]
        .parse::<usize>()
        .unwrap();

    let mut stacks = vec![vec![]; num_stacks];

    let stack_lines = input.lines().take(row_idx).collect::<Vec<_>>();
    dbg!(&stack_lines);
    stack_lines.iter().enumerate().for_each(|(i, line)| {
        for stack_i in 0..num_stacks {
            let str_index = stack_i * 4;

            let char = line.chars().nth(str_index + 1).unwrap();
            if !char.is_whitespace() {
                stacks[stack_i].push(char);
            }
        }
    });
    stacks.iter_mut().for_each(|stack| {
        stack.reverse();
    });

    input
        .lines()
        .skip(row_idx + 2)
        .enumerate()
        .map(|(i, line)| {
            dbg!(line);

            let o = (
                line.chars()
                    .skip(5)
                    .take_while(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
                {
                    let str = line
                        .chars()
                        .skip(12)
                        .take_while(|c| c.is_numeric())
                        .collect::<String>();
                    dbg!(&str);
                    str
                }
                .parse::<usize>()
                .unwrap_or_else(|e| panic!("{}", i)),
                line.chars()
                    .skip(17)
                    .take_while(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            );
            dbg!(&o);
            o
        })
        .for_each(|mov| {
            let (num, from, to) = mov;

            for _ in 0..num {
                let v = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(v);
            }
        });

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn part2(input: &str) -> u32 {
    todo!()
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}
