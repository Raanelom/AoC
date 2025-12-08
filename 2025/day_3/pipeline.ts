import { readFileSync } from 'fs';

const input: string[] = readFileSync('./input', 'utf-8').trim().split('\n');

function maxDigit(no: string, step = 0): { digit: number, position: number } {
    if (!no.length) {
        return { digit: 0, position: step };
    }
    const remainder = parseInt(no[0]);
    const nextDigit = maxDigit(no.slice(1), step + 1);
    if (nextDigit.digit > remainder) {
        return nextDigit;
    }
    return { digit: remainder, position: step };
}

let joltage = 0;
for (const no of input) {
    console.log("\nNo", no);
    let max = maxDigit(no);
    let remainder = no.slice(max.position + 1);
    if (!remainder.length) {
        console.log("Equal length");
        max = maxDigit(no.slice(0, -1), 1);
        remainder = no.slice(max.position);
    }
    const nextMax = maxDigit(remainder);
    // console.log(max, nextMax);
    joltage += (max.digit * 10 + nextMax.digit);
    
}

console.log(joltage);