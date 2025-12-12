import { readFileSync } from 'fs';

const isEqualArray = <T>(a: T[], b: T[]) => {
  return JSON.stringify(a) === JSON.stringify(b);
};

const edgeIngredients = new Set<number>();

const input: string[] = readFileSync('./input', 'utf-8').trim().split('\n\n');
const freshRanges = input[0]
    .split("\n")
    .map((freshRange) => freshRange.split("-").map((edge) => parseInt(edge)))
    .filter((current, currentIndex, array) => {
        // Filter out:
        // - duplicates
        // - arrays that are completely in another range
        return !(array?.find((other, otherIndex) => current !== other && isEqualArray(current, other) && currentIndex < otherIndex)
        || array.find((other) => current !== other && current[0] >= other[0] && current[1] <= other[1] && !isEqualArray(current, other)));
    })
    .sort((a, b) =>  a[0] - b[0]);

const freshRangesDiff = freshRanges
    .reduce((previous, current, _, array) => {
        const fallsInStartRange = array?.find((item) => {
            return item !== current && current[0] < item[1] && current[0] > item[0] && current[1] >= item[1]
        });
        if (fallsInStartRange) {
            edgeIngredients.add(current[1]);
            // If range start falls in another range, but range end doesn't, add the difference
            return [...previous, current[1] - fallsInStartRange[1]]
        }
        const bonus = edgeIngredients.has(current[0]) ? 0 : 1;
        edgeIngredients.add(current[1]);
        // Otherwise: no overlap, add current difference entirely
        return [...previous, current[1] - current[0] + bonus];
    }, []);


// Part 1
// let freshIngredients = 0;
// const ingredients = input[1].split("\n").map((ingredient) => parseInt(ingredient));
// for (const ingredient of ingredients) {
//     const isFresh = freshRanges.find((freshRange) => freshRange[0] <= ingredient && ingredient <= freshRange[1]);
//     if (isFresh) {
//         freshIngredients++;
//     }
// }
// freshRanges.sort((a, b) => (a[0] < b[0]) ? a : b).map((freshRange) => freshRange[1] - freshRange[0]).reduce((freshRange, current) => freshRange += current)

// 330702145638994 too low
// 354234690740142 too high
// 354234690740129 is ?
// 353716783056996 is ?
// 353716783056994 is correct!
console.log(freshRangesDiff.reduce((acc, current) => acc + current, 0));

