// @ts-check
import fs from "fs";
const input = fs.readFileSync("./input.txt").toString();

const elves = input.split("\r\n\r\n").map(v =>
    v
        .split("\r\n")
        .map(v => +v)
        .reduce((a, b) => a + b)
);
const top = elves.reduce((a, b) => Math.max(a, b));

const topThree = elves
    .sort((a, b) => b - a)
    .slice(0, 3)
    .reduce((a, b) => a + b);

console.log(`Part 1: ${top}`);
console.log(`Part 2: ${topThree}`);
