import { readFileSync } from 'fs';

const LIGHT_ON = "#";

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const press = async (buttons: number[][], endState: number[]): Promise<number> => {
    // TODO: implement queue instead of recursion which gives me headache
    // console.log("Buttons to press", remaining.length);
    const queue: {
        state: number[];
        pressed: number[][];
        index: number
    }[] = [];

    const emptyState = new Array<number>(endState.length).fill(0);

    queue.push({
        state: emptyState,
        pressed: [],
        index: 0
    })

    const knownStates: Set<string> = new Set();
    // TODO: build a recursive solution for this problem => depth-first in this case

    buttons.sort((a, b) => b.reduce((acc, current) => acc + current) - a.reduce((acc, current) => acc + current));

    while (queue.length) {
        const current = queue.shift();

        if (!current) {
            throw new Error("Queue has length, but is also empty?");
        }

        if (JSON.stringify(current.state) === JSON.stringify(endState)) {
            // This is the first viable solution
            return current.pressed.length;
        }

        if (knownStates.has(JSON.stringify(current))) {
            // We already processed this state, continue
            continue;
        }

        if (current.index === buttons.length) {
            continue;
        }

        // If any of the numbers exceeded the endstate, stop
        const exceeded = (state: number[]) => !!state.find((item, index) => item > endState[index]);
        if (exceeded(current.state)) {
            knownStates.add(JSON.stringify(current));
            
            const next = structuredClone(current);
            next.index++;
            const buttonToRemove = next.pressed.pop()!;
            for (const b of buttonToRemove) {
                next.state[b] -= 1;
            }
            queue.push(next);
            continue;
        }
        // console.log(current);

        

        knownStates.add(JSON.stringify(current.pressed));

        // for (let i = 0; i < buttons.length; i++) {
        const next = structuredClone(current);
        const buttonToPress = buttons[current.index];
        // console.log(buttonToPress);
        // console.log("Button to press", buttonToPress);
        next.pressed.sort();
        next.pressed.push(buttonToPress);
        for (const b of buttonToPress) {
            next.state[b] = next.state[b] + 1;
        }
        next.pressed;
        queue.push(next);
                console.log(next);
            await sleep(1000);
        // }
    }
    return Infinity;
}

const input: {
    lightState: boolean[];
    buttons: number[][];
    joltageState: number[];
}[] = readFileSync('./example_input', 'utf-8')
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
}());

