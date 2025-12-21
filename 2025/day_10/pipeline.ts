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
): Promise<number> => {
    // Let's implement this algorithm: https://old.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
    if (state.every((num) => num === 0)) {
        // State is empty? We're done
        return 0;
    }
    
    const oddPositions = state.map((val) => val % 2 === 1);

    // console.log("\nState", state);
    // console.log("Odd positions", oddPositions);
    let solution = Infinity;
    const nextButtonSets = await getNextButtons(buttons, oddPositions);

    for (const nextButtons of nextButtonSets) {
        const newState = [...state];
        // console.log("Press buttons", nextButtons);
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
        // console.log("New state", newState);
        // console.log("Next buttons", nextButtons);

        solution = Math.min(solution, 2 * (await press(buttons, newState)) + nextButtons.length);
    }
    return solution;
};

const getNextButtons = async (remaining: number[][], endState: boolean[]): Promise<number[][][]> => {
    // console.log("Buttons to press", remaining.length);
    const queue: {
        state: boolean[];
        pressed: number[][];
        remaining: number[][];
    }[] = [];

    const emptyState: boolean[] = new Array<boolean>(endState.length).fill(false);

    queue.push({
        state: emptyState,
        pressed: [],
        remaining
    });

    const knownStates: Set<string> = new Set();
    const solutions = new Set<string>();

    while (queue.length) {
        const current = queue.shift();
        if (!current) {
            throw new Error("Queue has length, but is also empty?");
        }

        if (JSON.stringify(current.state) === JSON.stringify(endState)) {
            // This is the first viable solution
            solutions.add(JSON.stringify(current.pressed));
            // console.log("Done immediately. Continue nonetheless");
        }

        if (!current.remaining.length) {
            // We can continue, this state is not worth investigating
            continue;
        }

        if (knownStates.has(JSON.stringify(current.pressed))) {
            // We already processed this state, continue
            continue;
        }

        knownStates.add(JSON.stringify(current.pressed))

        for (let i = 0; i < current.remaining.length; i++) {
            const next = structuredClone(current);
            const buttonToPress = next.remaining.splice(i, 1)[0];

            next.pressed.push(buttonToPress);

            for (const b of buttonToPress) {
                next.state[b] = !next.state[b];
            }

            next.pressed.sort();
            queue.push(next);
        }
    }
    // console.log(solutions);
    return [...solutions].map((solution) => JSON.parse(solution));
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
    }).slice(0, 2);

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

