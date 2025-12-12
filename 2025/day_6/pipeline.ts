import { readFileSync } from 'fs';

const input: string[] = readFileSync('./input', 'utf-8').trim().split('\n');


const operators = input.slice(-1).join("").split(new RegExp(/[ ]+/));
const numbers = input.slice(0, -1);
const verticalNumbers: string[] = numbers[0].split("").map((_) => "");

for (let i = 0; i < numbers[0].length; i++) {
    numbers.forEach((item) => {
        verticalNumbers[i] += item[i];
    })
    verticalNumbers[i] = verticalNumbers[i].trim();
}

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

// Part A
// let total = 0;
// for(let i = 0; i < numbers[0].length; i++) {
//     const selectedRange = numbers.reduce((previous, current) => {
//         return [...previous, current[i]];
//     }, []);
//     total += calc(operators[i])(selectedRange)
// }

let total = 0;
for(let i = 0; i < operators.length; i++) {
    const selectedRange = verticalNumbers
        .splice(0, (verticalNumbers.indexOf("") + 1) || verticalNumbers.length)
        .filter((item) => item !== "")
        .map((item) => parseInt(item));
    total += calc(operators[i])(selectedRange)
}

// 3263827
console.log(total);