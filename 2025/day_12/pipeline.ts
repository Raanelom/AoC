import { readFileSync } from 'fs';

const parseInput = () => {
    const input: string[][] = readFileSync('./example_input', 'utf-8')
        .trim()
        .split('\n\n')
        .map((present: string, index: number, items: string[]) => {
            return present.split("\n")
                .slice(index < items.length - 1 ? 1 : 0);
        });

    const regions = input.splice(-1).flat();
    const presents = input;

    const output = regions.map((region) => {
        const [size, ...positions] = region.split(": ")
            .map((item, index) => index > 0 ? item.split(" ").map((no) => parseInt(no)).flat() : [item.split("x").map((size) => parseInt(size))])
            .flat();
        const selectedPresents = positions
            .map((present, index) => new Array<string[]>(present as number).fill(presents[index]))
            .flat();
        return { size: size as number[], selectedPresents }
    });

    return output
}

const input = parseInput();

for (const line of input) {
    // TODO: think of something better than dots
    const region = new Array(line.size[1]).fill(new Array(line.size[0]).fill("."));
    console.log(region);
}