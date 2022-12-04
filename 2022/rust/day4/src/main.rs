const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

const INPUT: &str = include_str!("../input.txt");

fn part1(input: &str) -> u32 {
    input.lines().fold(0, |mut total, line| {
        let ranges = line
            .split(',')
            .map(|str| {
                str.split('-')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let range1 = &ranges[0];
        let range2 = &ranges[1];

        if (range1[0] >= range2[0] && range1[1] <= range2[1])
            || (range2[0] >= range1[0] && range2[1] <= range1[1])
        {
            total += 1;
        }

        total
    })
}

fn part2(input: &str) -> u32 {
    input.lines().fold(0, |mut total, line| {
        let ranges = line
            .split(',')
            .map(|str| {
                str.split('-')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let range1 = &ranges[0];
        let range2 = &ranges[1];

        // 5-10,4-7
        // 0-2,1-5
        // 0-2,2-3
        // if 1 is between 0-2, then overlap
        // else if 5 is between 0-2, then overlap
        if (range2[0] >= range1[0] && range2[0] <= range1[1])
            || (range2[1] >= range1[0] && range2[1] <= range1[1])
            || (range1[0] >= range2[0] && range1[0] <= range2[1])
            || (range1[1] >= range2[0] && range1[1] <= range2[1])
        {
            total += 1;
        }

        total
    })
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}
