import { readFileSync } from 'fs';

const input: string[] = readFileSync('./example_input', 'utf-8').trim().split('\n');

const operators = input[-1].split(new RegExp("[ ]+"));
const numbers = input.slice(0, -1).map((row) => row.split(new RegExp("[ ]+")).map((no) => parseInt(no)));

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
    const selectedRange = numbers.filter((_, index) => i === index).flat(1);
    total += calc(operators[i])(selectedRange)

}
console.log(total);