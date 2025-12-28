import { readFileSync } from 'fs';

const findPaths = (from: string, 
        edges: Map<string, string[]>, 
        path: string = from, 
        paths: Set<string> = new Set()) => {
    if (path.endsWith("out")) {
        paths.add(path);
        return;
    }
    const possiblePaths = edges.get(from);
    if (!possiblePaths || !possiblePaths.length) {
        return;
    }
    for (const nextPath of possiblePaths) {
        console.log(path);
        findPaths(nextPath, edges, `${path} ${nextPath}`, paths);
    }
    return paths;
}

const exec = () => {
    const inputLines: string[] = readFileSync('./input', 'utf-8').trim().split('\n');

    const edges = new Map<string, string[]>;

    inputLines.forEach((line) => {
        const [ source, targets ] = line.split(": ");
        const edgeSet = edges.set(source, []);
        for (const target of targets.split(" ")) {
            edgeSet.get(source)?.push(target);
        }
    });

    // const start = edges.get("you");

    console.log(edges);
    // console.log(start);
    const paths = findPaths("you", edges);
    console.log(paths?.size);
}

exec();