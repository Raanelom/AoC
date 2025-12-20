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

const selectBestButton = (buttons: number[][], state: number[], endState: number[]) => {
    // console.log("\nStart selecting a button for state", state);

    const positionCoverage = new Array(endState.length).fill(0);
    for (const button of buttons) {
        for (const pos of button) {
            positionCoverage[pos]++;
        }
    }

     const bestButton = buttons.map((button) => {
        const simulateState = [...state];
        for (const pos of button) {
            simulateState[pos]++;
        }

        const overshooting = simulateState.some((val, i) => val > endState[i]);

        if (overshooting) return { button, score: Infinity };

        const score = simulateState.reduce((sum, val, i) => {
            const diff = endState[i] - val;
            return sum + Math.pow(diff, 2);
        }, 0);

        // Tiebreaker 4: Uniqueness - prioritize buttons that affect rare positions
        // Lower coverage = more unique = higher priority
        // Only count positions that still need work
        const uniqueness = button
            .filter(pos => state[pos] < endState[pos])
            .reduce((sum, pos) => sum + (1 / positionCoverage[pos]), 0);

        console.log("Score", score, "uniqueness", uniqueness.toFixed(2), button, simulateState);

        // console.log("Score", score, "Uniqueness", uniqueness, "button", button, simulateState);

        return { button, score, uniqueness };
    })
    .filter((btn) => btn.score < Infinity)
    .sort((a, b) => a.score - b.score || b.uniqueness! - a.uniqueness!);
    // console.log("best button", bestButton[0]);
    return bestButton[0];
    // console.log("Endstate - startState, diff", endState, state, stateDiff);
}

const press = async (
    buttons: number[][], 
    endState: number[], 
): Promise<number> => {
    // TODO:
    // - Stay in a while-loop until the endState is equal to the current state
    // 
    const currentState = new Array<number>(endState.length).fill(0);
    const pressed: number[][] = [];

    while (JSON.stringify(currentState) !== JSON.stringify(endState)) {
        // console.log("Current", currentState);
        // console.log("Pressed", pressed);
        // await sleep(100);
        const bestButton = selectBestButton(buttons, currentState, endState);
        if (!bestButton) {
            console.error("Stuck at state", JSON.stringify(currentState), "and Pressed", JSON.stringify(pressed));
            return -1;
        }
        pressed.push(bestButton.button);
        for (const pos of bestButton.button) {
            currentState[pos]++;
        }
        // TODO: select the BEST button to add that achieves the best balance (least difference)
        // break;
    }

    return pressed.length;
}

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
        const leastPresses = await pressDFS(buttons, joltageState);
        // const leastPresses = pressDPOptimized(buttons, joltageState);
        presses.push(leastPresses);
        console.log("Least presses", leastPresses);
        lineNo++;
    }
    console.log("\nPresses count:", presses.reduce((prev, current) => prev += current));
}());

