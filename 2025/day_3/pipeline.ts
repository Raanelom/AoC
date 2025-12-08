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
    // console.log("\nNo", no);
    let digitCountdown = 12;
    const digits = [];
    let remainder = no;
    while (digitCountdown > 0) {
        const max = maxDigit(remainder.slice(0, remainder.length - (digitCountdown - 1)));
        digits.push(max);
        // console.log(digits);
        remainder = remainder.slice(max.position + 1);
        // 818181911112111
        digitCountdown -= 1;
    }
    digits.reverse();
    // console.log(digits.map((n, index) => n.digit * Math.pow(10, index)).reduce((partialSum, a) => partialSum + a, 0));
    joltage += digits.map((n, index) => n.digit * Math.pow(10, index)).reduce((partialSum, a) => partialSum + a, 0);

    
}
console.log(joltage);

