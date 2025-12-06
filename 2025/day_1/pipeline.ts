import { readFileSync } from 'fs';

const dialCommands: string[] = readFileSync('./input', 'utf-8').trim().split('\n');

let dial = 50;
let passwordCount = 0;

// Cases:
// - Dial LEFT while the dial is positive and ends up negative
// - Dial LEFT while the dial is negative and ends up below or at -100
// - Dial RIGHT while the dial is negative and ends up positive
// - Dial RIGHT while the dial is positive and ends up above or at 100

for (let dialCommand of dialCommands) {
    const [ direction, number ] = [ dialCommand[0], parseInt(dialCommand.slice(1)) ]
    if (direction === "L") {
        let remainder = number % 100;
        passwordCount += (dial > 0 && (dial - remainder) <= 0) || (dial < 0 && (dial - remainder <= -100)) ? 1 : 0; 
        passwordCount += Math.floor(number / 100);

        dial = dial - number
    } else {
        let remainder = number % 100;
        passwordCount += (dial < 0 && (dial + remainder) >= 0) || (dial > 0 && (dial + remainder >= 100)) ? 1 : 0;
        passwordCount += Math.floor(number / 100);

        dial = dial + number;
    }

    dial = dial % 100;
    // console.log("\nOperation is: ", direction + number);
    // console.log("Passwordcount is now ", passwordCount);
    // console.log("Dial points at ", dial);    
    
}
console.log(passwordCount);