import { readFileSync } from 'fs';

const LIGHT_ON = "#";

const press = async (state: boolean[], endState: boolean[], buttons: number[][], buttonsPressed: number[][], table: Map<string, number>): Promise<number> => {
    // TODO: implement queue instead of recursion which gives me headache
    console.log("Buttons to press", buttons.length);
    const presses: Promise<number>[] = [];
    for (let i = 0; i < buttons.length; i++) {
        const newState = [...state];
        const newButtons: number[][] = JSON.parse(JSON.stringify(buttons));
        const buttonToPress = newButtons.splice(i, 1)[0];
        for (const b of buttonToPress) {
            newState[b] =! newState[b];
        }
        const newButtonsPressed: number[][] = JSON.parse(JSON.stringify(buttonsPressed));
        newButtonsPressed.push(buttonToPress);
        newButtonsPressed.sort();

        const stateKey = JSON.stringify(newButtonsPressed);
        const existingState = table.get(stateKey)
        if(existingState) {
            presses.push(Promise.resolve(existingState));
            // console.log("Added existing state", existingState);
        }
        else if (JSON.stringify(newState) === JSON.stringify(endState)) {
            console.log("Correct state", newButtonsPressed);
            table.set(stateKey, newButtonsPressed.length);
            return newButtonsPressed.length;
        }
        else if (newButtons.length === 0) {
            // console.log("Infinity", newButtons);
            table.set(stateKey, Infinity);
            presses.push(Promise.resolve(Infinity));
        } else {
            // TODO: calculate presses here first. Then search for new states
            presses.push(press(newState, endState, newButtons, newButtonsPressed, table));
        }
        
    }
    return Math.min(...(await Promise.all(presses))); // Returns infinity for empty lists
    
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
        const lightState: boolean[] = new Array<boolean>(line.endState.length).fill(false);
        console.log("\n");
        console.log("Start processing line", lineNo, line.endState);
        const statesTable = new Map<string, number>();
        const leastPresses = await press(lightState, line.endState, line.buttons, [], statesTable);
        console.log("Least presses", leastPresses);
        lineNo++;
    }
}());

