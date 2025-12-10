import { readFileSync } from 'fs';
const {
  createHmac,
} = await import('node:crypto');

const input: string[] = readFileSync('./input', 'utf-8').trim().split('\n');

let tpScore = 0;
const rowLength = input[0].length;
const TP = "@";
const noTP = ".";
const secret = "toiletpaper"

const matchTP = (item: string | undefined, index: number) => {
 return (item?.[index] === TP && 1 || 0)
}

const lookForTP = (previousRow: string | undefined, currentRow: string, nextRow: string | undefined, index: number) => {
    return matchTP(previousRow, index - 1) + matchTP(previousRow, index) +  matchTP(previousRow, index + 1)
        + matchTP(currentRow, index - 1) + matchTP(currentRow, index + 1)
        + matchTP(nextRow, index - 1) + matchTP(nextRow, index) + matchTP(nextRow, index + 1);
}

const hash = (value: string) => createHmac("sha256", secret).update(value).digest("hex");

let newInput = [...input];
let hashDigest = hash("init");

while (hashDigest !== hash(JSON.stringify(newInput))) {
    hashDigest = hash(JSON.stringify(newInput));
    newInput = newInput.map((row, rowIndex) => {
        const newRow = [];
        for(let columnIndex = 0; columnIndex < rowLength; columnIndex++) {
            if (row[columnIndex] !== TP) {
                newRow.push(noTP);
                continue;
            }
            const tpNeighbours = lookForTP(newInput?.[rowIndex - 1], row, newInput?.[rowIndex + 1], columnIndex)
            // console.log("\nIndices", rowIndex, columnIndex);
            // console.log("Score", tpNeighbours);
            const removeTP = tpNeighbours <= 3;
            if (removeTP) {
                tpScore++;
                newRow.push(noTP)
            }
            else {
                newRow.push(TP);
            }
        }
        return newRow.join("");
    });
}

console.log("\n\nFinal score:", tpScore);