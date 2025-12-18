import { readFileSync } from 'fs';

const LIGHT_ON = "#";

const press = async (remaining: number[][], endState: boolean[]): Promise<number> => {
    // TODO: implement queue instead of recursion which gives me headache
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
    })

    while (queue.length) {
        const current = queue.shift();

        if (!current) {
            throw new Error("Queue has length, but is also empty?");
        }

        if (JSON.stringify(current.state) === JSON.stringify(endState)) {
            // This is the first viable solution
            return current.pressed.length;
        }

        if (!current.remaining.length) {
            // We can continue, this state is not worth investigating
            continue;
        }

        for (let i = 0; i < current.remaining.length; i++) {
            const next = structuredClone(current);
            const buttonToPress = next.remaining.splice(i, 1)[0];
            // console.log("Button to press", buttonToPress);
            next.pressed.push(buttonToPress);
            for (const b of buttonToPress) {
                next.state[b] = !next.state[b];
            }
            next.pressed.sort();
            queue.push(next);
        }
    }
    return Infinity;    
}

const input: {
    endState: boolean[];
    buttons: number[][];
}[] = readFileSync('./example_input', 'utf-8')
    .trim()
    .split('\n')
    .map((line: string) => {
        const lineItems = line.split(" ");
        const endState = lineItems[0]
            .slice(1, -1)
            .split("")
            .map((l) => l === LIGHT_ON);
        const buttons = lineItems
            .slice(1, -1)
            .map((b) => b.slice(1, -1).split(",").map((no) => parseInt(no)));
        return { endState, buttons }
    });

(async function() {
    let lineNo = 0;
    for (const line of input) {
        
        console.log("\n");
        console.log("Start processing line", lineNo, line.endState);
        const statesTable = new Map<string, number>();
        const leastPresses = await press(line.buttons, line.endState);
        console.log("Least presses", leastPresses);
        lineNo++;
    }
}());

