const fs = require('fs');
const day1 = require('./day1');

describe('day one', () => {
    const input = fs.readFileSync('day-1/input.txt', 'utf8');

    it('runs part one', () => {
        console.log(day1.part1(input));
    });

    it('runs part two', () => {
        console.log(day1.part2(input));
    });
});
