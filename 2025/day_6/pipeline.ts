import { readFileSync } from 'fs';

const input: string[] = readFileSync('./input', 'utf-8').trim().split('\n');


const operators = input.slice(-1).join("").split(new RegExp(/[ ]+/));
const numbers = input.slice(0, -1).map((row) => row.trim().split(new RegExp(/[ ]+/)).map((no) => parseInt(no)));

const ADD = "+";
const MULTIPLY = "*";

const add = (range: number[]) => {
    return range.reduce((aggregate, number) => {
        aggregate += number;
        return aggregate;
    });
}

const multiply = (range: number[]) => {
    return range.reduce((aggregate, number) => {
        aggregate *= number;
        return aggregate;
    });
}

const calc = (operator: string) => {
    switch (operator) {
        case ADD:
            return add;
        case MULTIPLY:
            return multiply;
        default:
            throw new Error("Not supported");
    }
}

let total = 0;
for(let i = 0; i < numbers[0].length; i++) {
    // Use reduce instead of filter here
    const selectedRange = numbers.reduce((previous, current) => {
        return [...previous, current[i]];
    }, []);
    total += calc(operators[i])(selectedRange)
}

console.log(total);