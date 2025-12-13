import { readFileSync } from 'fs';

const input: string[] = readFileSync('./input', 'utf-8').trim().split('\n');

const SPACE = ".";
const SPLITTER = "^";
const START = "S";

const knownSplits = new Map<string, number>;

const calcBeamSplit = (index: number, row: number, matrix: string[]): number => {
    if (row === matrix.length) {
        return 1;
    }
    if (matrix[row][index] === SPACE) {
        return calcBeamSplit(index, row + 1, matrix);
    }
    const key = `${row}-${index}`;
    if (matrix[row][index] === SPLITTER && !knownSplits.get(key)) {
        // console.log("hit the SPLITTER", row, index);
        knownSplits.set(key, calcBeamSplit(index - 1, row, matrix) + calcBeamSplit(index + 1, row, matrix));
    }
    if (matrix[row][index] === SPLITTER) {
        return knownSplits.get(key) || 0;
    }
    throw new Error("Undefined element");
}

const splits = calcBeamSplit(input[0].indexOf(START), 1, input);

console.log(splits);