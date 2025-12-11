import { readFileSync } from 'fs';

const input: string[] = readFileSync('./example_input', 'utf-8').trim().split('\n');

console.log(input);