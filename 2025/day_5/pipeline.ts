import { readFileSync } from 'fs';

const input: string[] = readFileSync('./example_input', 'utf-8').trim().split('\n\n');
const freshRanges = input[0].split("\n").map((freshRange) => freshRange.split("-").map((edge) => parseInt(edge)));
const ingredients = input[1].split("\n").map((ingredient) => parseInt(ingredient));

let freshIngredients = 0;
for (const ingredient of ingredients) {
    const isFresh = freshRanges.find((freshRange) => freshRange[0] <= ingredient && ingredient <= freshRange[1]);
    if (isFresh) {
        freshIngredients++;
    }
}

console.log(freshIngredients);