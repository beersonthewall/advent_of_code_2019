const part1 = (input) => {
    return getMassNumbers(input)
        .map(mass => fuelRequirements(mass))
        .reduce(add, 0);
};

const add = (a, b) => {
    if(isNaN(b)) {
        b = 0;
    }
    return a + b;
}

const fuelRequirements = (mass) => {
    return Math.floor(mass / 3) - 2;
};

const newFuelRequirements = (mass) => {
    let result = fuelRequirements(mass);
    let sum = 0;
    while(result > 0) {
        sum += result;
        result = fuelRequirements(result);
    }
    return sum;
};

const part2 = (input) => {
    return getMassNumbers(input)
        .map(mass => newFuelRequirements(mass))
        .reduce(add, 0);
};

const getMassNumbers = (input) => {
    return input
        .split('\n')
        .map(numString => parseInt(numString));
};

exports.part1 = part1;
exports.part2 = part2;
