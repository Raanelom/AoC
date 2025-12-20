import { readFileSync } from 'fs';

const LIGHT_ON = "#";

function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

// (0,5) (1,2,3,4,5) (1,3,4,5) (3,4) (2,3,5) (0,1,2,5) [29,40,23,42,39,52]
// TODO: sort buttons based on adding the most balance in the result set
const sortButtons = (buttons: number[][], joltageState: number[]) => {
    const joltageTotal = joltageState.reduce((acc, current) => acc + current, 0);
    const joltageWeights = joltageState.map((value) => value / joltageTotal);
    console.log(joltageState);
    console.log(JSON.stringify(joltageWeights));
    // const buttonsWeights = buttons.map((btn) => {
    //     let uniqueness = buttons.length;
    //     for (const b of btn) {
    //         uniqueness = Math.min(uniqueness, buttons.filter((button) => button !== btn && button.includes(b)).length || 1);
    //     }
    //     // The lower the number, the more unique
    //     return 1 / uniqueness;
    // });
    // console.log(buttons);
    // console.log(buttonsWeights);
    // throw new Error("Stop");

    // console.log("Weights", joltageWeights);
    // const joltageOrder = joltageState.map((value, index) => ({ value, index })).sort((a, b) => b.value - a.value);
    return buttons.sort((a, b) => {
        // const uniqueA = buttonsWeights[buttons.indexOf(a)];
        // const uniqueB = buttonsWeights[buttons.indexOf(b)];
        const weightA = a.reduce((acc, current) => acc + joltageWeights[current], 0);
        const weightB = b.reduce((acc, current) => acc + joltageWeights[current], 0);
        // console.log("\nSorting", a, b);
        // console.log("Weights", weightA, weightB);
        return weightB - weightA || b.length - a.length;
    });
}

const stateDiff = (currentState: number[], endState: number[]) => {
    const diffState = structuredClone(endState);
    for (let i = 0; i < currentState.length; i++) {
        diffState[i] -= currentState[i];
    }
    return diffState;
}

const press = async (
    buttons: number[][], 
    endState: number[], 
    state: number[] = new Array<number>(endState.length).fill(0), 
    pressed: number[][] = [],
    knownStates: Set<string> = new Set()
): Promise<number> => {
    // console.log("Pressed", JSON.stringify(pressed));
    // console.log("State", JSON.stringify(state));
    // console.log("\n");
    // await sleep(100);
    if (JSON.stringify(state) === JSON.stringify(endState)) {
        // This is the first viable solution
        return pressed.length;
    }
    if (knownStates.has(JSON.stringify(pressed))) {
        // console.log("KNOWN STATE");
        return Infinity;
    }

    const selectedButtons = buttons.filter((btn) => {
        let isValidButton = true;
        for (const b of btn) {
            isValidButton = isValidButton && (endState[b] - state[b] > 0)
        }
        return isValidButton;
    })
    sortButtons(selectedButtons, stateDiff(state, endState));
    // console.log(selectedButtons);
    for (let i = 0; i < selectedButtons.length; i++) {
        const nextState = structuredClone(state);
        const newPressed = structuredClone(pressed);
        const buttonToPress = selectedButtons[i];
        newPressed.push(buttonToPress);
        newPressed.sort();
        for (const b of buttonToPress) {
            nextState[b] = nextState[b] + 1;
        }
        
        const res = await press(selectedButtons, endState, nextState, newPressed, knownStates);
        if (res < Infinity) {
            return res;
        }
        knownStates.add(JSON.stringify(newPressed));
    }
    return Infinity
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
        
        // sortButtons(buttons, joltageState);
        // console.log(joltageState);
        // console.log(buttons);
        const leastPresses = await press(buttons, joltageState);
        presses.push(leastPresses);
        console.log("Least presses", leastPresses);
        lineNo++;
    }
    console.log("\nPresses count:", presses.reduce((prev, current) => prev += current));
}());

