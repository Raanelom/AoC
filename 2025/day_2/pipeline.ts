import { readFileSync } from 'fs';

const productIdRanges: Array<number[]> = readFileSync('./input', 'utf-8').trim().split(',').map((item: string) => item.split("-").map((no) => parseInt(no)));

function numDigits(num: number) {
  return Math.floor(Math.log10(num)) + 1;
}

function roundToNextEvenNumDigits(num: number) {
    const base = Math.floor(Math.log10(num));
    if (base % 2 === 0) {
        return Math.pow(10, base + 1)
    }
    return num;
}

function roundToPreviousEvenNumDigits(num: number) {
    const base = Math.floor(Math.log10(num));
    if (base <= 0) {
        return 0;
    }
    if (base % 2 === 0) {
        return Math.pow(10, base) - 1;
    }
    return num;
}

let invalidIds = 0;
for (const productIdRange of productIdRanges) {
    const [start, end] = productIdRange;

    for(let i = roundToNextEvenNumDigits(start); i <= roundToPreviousEvenNumDigits(end); i++) {
        const digits = numDigits(i);
        const firstPart = Math.floor(i / (Math.pow(10, digits / 2)));
        const secondPart = (firstPart * Math.pow(10, digits / 2));
        // console.log("\n");
        // console.log(i);
        // console.log(firstPart);
        // console.log(secondPart);
        invalidIds += i - firstPart === secondPart ? i : 0;
    }
}

console.log(invalidIds);