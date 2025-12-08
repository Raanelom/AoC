import { readFileSync } from 'fs';

const input: string[] = readFileSync('./example_input', 'utf-8').trim().split('\n');

// function maxDigit(no: number, step = 0): { digit: number, position: number } {
//     if (no === 0) {
//         return { digit: 0, position: step };
//     }
//     const remainder = no % 10;
//     const nextDigit = maxDigit((no - remainder) / 10, step + 1);
//     if (nextDigit.digit > remainder) {
//         return nextDigit;
//     }
//     return { digit: remainder, position: step };
// }

// function mapToNumbers(str: string) {
//     const chunkSize = 10;
//     const chunks: number[] = [];
//     for (let i = 0; i < str.length; i += chunkSize) {
//         const chunk = parseInt(str.slice(i, i + chunkSize));
//         chunks.push(chunk);
//     }
//     return chunks;
// }

let joltage = 0;
for (const no of input) {
    console.log("\nNo", no);
    const chunks = mapToNumbers(no);
    const max = maxDigit(no);
    // Extract the exact digit
    let subtract = (max.digit * Math.pow(10, max.position))
    if (max.position > 0) {
        // Extract all digits beyond the position
        subtract = Math.floor(no / Math.pow(10, max.position)) * Math.pow(10, max.position);
    }
    const nextNo = no - subtract;
    console.log("Nextno", nextNo)
    const nextMax = maxDigit(nextNo);
    if (max.position === 0) {
        joltage += nextMax.digit * 10 + max.digit;
    }
    else {
        joltage += max.digit * 10 + nextMax.digit;
    }
    console.log(joltage);
}

console.log(joltage);