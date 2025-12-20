import { readFileSync } from 'fs';

const LIGHT_ON = "#";

function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

// (0,5) (1,2,3,4,5) (1,3,4,5) (3,4) (2,3,5) (0,1,2,5) [29,40,23,42,39,52]

//  a * (0, 1, 1, 1, 1, 1)
//  b * (0, 1, 0, 1, 1, 1)
//  c * (0, 0, 0, 1, 1, 0)
//  d * (0, 0, 1, 1, 0, 1)
//  e * (1, 1, 1, 0, 0, 1)
//  = [ 29,40,23,42,39,52 ]
// Optimize a + b + c + d + e

const press = async (
    buttons: number[][], 
    endState: number[], 
): Promise<number> => {
    const numButtons = buttons.length;
    const numPositions = endState.length;
    
    // Build the matrix where matrix[pos][button] = 1 if button affects pos
    const matrix: number[][] = Array(numPositions).fill(0).map(() => Array(numButtons).fill(0));
    
    for (let b = 0; b < numButtons; b++) {
        for (const pos of buttons[b]) {
            matrix[pos][b] = 1;
        }
    }
    // Solve matrix, dive into study material
};

const input: {
    lightState: boolean[];
    buttons: number[][];
    joltageState: number[];
}[] = readFileSync('./input', 'utf-8')
    .trim()
    .split('\n')
    .map((line: string) => {
        const lineItems = line.split(" ");
        const lightState = lineItems[0]
            .slice(1, -1)
            .split("")
            .map((l) => l === LIGHT_ON);
        const buttons = lineItems
            .slice(1, -1)
            .map((b) => b.slice(1, -1).split(",").map((no) => parseInt(no)));
        const joltageState = lineItems
            .slice(-1)[0]
            .slice(1,-1)
            .split(",").map((no) => parseInt(no));
        return { lightState, buttons, joltageState }
    }).slice(0, 1);

(async function() {
    
    let lineNo = 0;
    const presses: number[] = [];
    for (const line of input) {
        const { lightState, buttons, joltageState } = line;
        console.log("\n");
        console.log("Start processing line", lineNo);
        const leastPresses = await press(buttons, joltageState);
        // const leastPresses = pressDPOptimized(buttons, joltageState);
        presses.push(leastPresses);
        console.log("Least presses", leastPresses);
        lineNo++;
    }
    console.log("\nPresses count:", presses.reduce((prev, current) => prev += current));
}());

