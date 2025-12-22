import { readFileSync } from 'fs';

const LIGHT_ON = "#";

function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

//  a * (0, 1, 1, 1, 1, 1)
//  b * (0, 1, 0, 1, 1, 1)
//  c * (0, 0, 0, 1, 1, 0)
//  d * (0, 0, 1, 1, 0, 1)
//  e * (1, 1, 1, 0, 0, 1)
//  = [ 29,40,23,42,39,52 ]
// Optimize a + b + c + d + e

const press = async (
    buttons: number[][],
    state: number[] = [],
    knownPresses: Map<string, number> = new Map()
): Promise<number> => {
    // Thanks to https://old.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/ <3
    if (state.every((num) => num === 0)) {
        // State is empty? We're done
        return 0;
    }

    const stateKey = JSON.stringify(state);

    if (knownPresses.has(stateKey)) {
        return knownPresses.get(stateKey)!;
    }
    
    const oddPositions = state.map((val) => val % 2 === 1);

    let solution = Infinity;
    const nextButtonSets = getNextButtons(buttons, oddPositions);

    for (const nextButtons of nextButtonSets) {
        const newState = [...state];
        
        for (const button of nextButtons) {
            for (const b of button) {
                newState[b]--;
            }
        }
        newState.forEach((s, index) => newState[index] = s / 2);

        if (newState.some((val) => val < 0)) {
            // This state is invalid
            continue;
        }

        const newStateKey = JSON.stringify(newState);
        const nextPress = knownPresses.get(newStateKey) ?? await press(buttons, newState, knownPresses);
        
        knownPresses.set(newStateKey, nextPress);
        solution = Math.min(solution, 2 * (nextPress) + nextButtons.length);
    }
    knownPresses.set(stateKey, solution);
    return solution;
};

const buttonCache = new Map<string, number[][][]>();

const getNextButtons = (
    buttons: number[][], 
    endState: boolean[]
): number[][][] => {
    const cacheKey = JSON.stringify({ buttons, endState });
    
    if (buttonCache.has(cacheKey)) {
        return buttonCache.get(cacheKey)!;
    }

    const solutions: number[][][] = [];
    
    const totalCombinations = Math.pow(2, buttons.length);
        
    for (let mask = 0; mask < totalCombinations; mask++) {
        const state = new Array<boolean>(endState.length).fill(false);
        const pressed: number[][] = [];
        
        for (let i = 0; i < buttons.length; i++) {
            if (mask & (1 << i)) {
                pressed.push(buttons[i]);
                for (const pos of buttons[i]) {
                    state[pos] = !state[pos];
                }
            }
        }
        if (JSON.stringify(state) === JSON.stringify(endState)) {
            solutions.push(pressed);
        }
    }

    buttonCache.set(cacheKey, solutions);
    return solutions;
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
    });

(async function() {
    
    let lineNo = 0;
    const presses: number[] = [];
    for (const line of input) {
        const { lightState, buttons, joltageState } = line;
        console.log("\n");
        console.log("Start processing line", lineNo);
        const leastPresses = await press(buttons, joltageState);
        presses.push(leastPresses);
        console.log("Least presses", leastPresses);
        lineNo++;
    }
    console.log("\nPresses count:", presses.reduce((prev, current) => prev += current));
    // 19293
}());

