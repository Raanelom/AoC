import { readFileSync } from 'fs';

const productIdRanges: Array<number[]> = readFileSync('./input', 'utf-8').trim().split(',').map((item: string) => item.split("-").map((no) => parseInt(no)));

function numDigits(num: number) {
  return Math.floor(Math.log10(num)) + 1;
}

// function roundToNextEvenNumDigits(num: number) {
//     const base = Math.floor(Math.log10(num));
//     if (base % 2 === 0) {
//         return Math.pow(10, base + 1)
//     }
//     return num;
// }

// function roundToPreviousEvenNumDigits(num: number) {
//     const base = Math.floor(Math.log10(num));
//     if (base <= 0) {
//         return 0;
//     }
//     if (base % 2 === 0) {
//         return Math.pow(10, base) - 1;
//     }
//     return num;
// }

let invalidIds = 0;
for (const productIdRange of productIdRanges) {
    const [start, end] = productIdRange;

    for(let i = start; i <= end; i++) {
        const digits = numDigits(i);
        for(let j = 1; j <= digits / 2; j++) {
            const pattern = new RegExp(`^(${Math.floor(i / (Math.pow(10, digits - j)))})+$`);

            if(pattern.test(i.toString())) {
                // console.log("\nPattern", pattern);
                // console.log(i);
                invalidIds += i;
                break;
            }
        }
        
    }
}

// Should be 4174379265
// Is        4174379265
console.log("\n", invalidIds);