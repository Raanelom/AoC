import { readFileSync } from 'fs';

const getSortedIndegree = (edges: Map<string, string[]>) => {
    // Let's do some topological sort first
    let indegree = new Map<string, number>();
    // Set nodes without indegree to zero
    [...edges.keys()].filter((node) => {
        return !([...edges.values()].some((nodeList) => nodeList.includes(node)));
    }).forEach((key) => {
        indegree.set(key, 0);
    });
    for (const edge of edges) {
        for (const target of edge[1]) {
            indegree.set(target, (indegree.get(target) || 0) + 1)
        }
    }
    const sortedIndegree = [...indegree.entries()].sort((a, b) => a[1] - b[1]);

    return new Map(sortedIndegree);
}

// Inspired by https://www.geeksforgeeks.org/dsa/count-possible-paths-two-vertices/#approach-1-using-depthfirst-search-o2n-time-and-on-space
const findPaths = (
    from: string,
    to: string,
    edges: Map<string, string[]>
) => {
    // Let's do some topological sort first
    const indegree = getSortedIndegree(edges);

    const order: string[] = [];
    // Start with indegree 0
    const queue = [...indegree.entries()].filter((n) => n[1] === 0).map((n) => n[0]);

    while (queue.length > 0) {
        const node = queue.shift()!;
        order.push(node);

        if (!edges.has(node)) {
            continue;
        }
        for (const next of edges.get(node)!) {
            indegree.set(next, indegree.get(next)! - 1);
            if (indegree.get(next) === 0) {
                queue.push(next);
            }
        }
    }

    const reversePaths = new Map<string, string[]>();
    reversePaths.set(from, [from]);

    const noOfPaths = new Map<string, number>();
    noOfPaths.set(from, 1);

    for (const node of order) {
        const prev = reversePaths.get(node) || [];
        if (edges.has(node)) {
            for (const next of edges.get(node)!) {
                noOfPaths.set(next, (noOfPaths.get(next) || 0) + (noOfPaths.get(node) || 0));
                // console.log(node, next);
                const nextPaths = reversePaths.get(next) || [];
                for (const prevPath of prev) {
                    // console.log(prevPath);
                    if (next !== "out" || (prevPath.includes("fft") && prevPath.includes("dac"))) {
                        nextPaths?.push(prevPath + " " + next);
                    }
                }
                reversePaths.set(next, nextPaths);
            }
            // console.log("delete");
            reversePaths.delete(node);
        }
    }
    console.log(noOfPaths.get(to));
    return reversePaths.get(to);
}

const exec = () => {
    const inputLines: string[] = readFileSync('./input', 'utf-8').trim().split('\n');

    const edges = new Map<string, string[]>();

    inputLines.forEach((line) => {
        const [ source, targets ] = line.split(": ");
        const edgeSet = edges.set(source, []);
        for (const target of targets.split(" ")) {
            edgeSet.get(source)?.push(target);
        }
    });

    const sortedEdges = new Map([...edges.entries()].sort((a, b) => a[1].length - b[1].length));

    // const start = edges.get("you");

    // console.log(edges);
    // console.log(start);
    // Ideas:
    // - Multiple "find shortest path"-algorithm, excluding previously found shortest paths
    // - Bread-first-search
    const paths = findPaths("svr", "out", sortedEdges);
    // const pathsToFFT = findPaths("svr", "fft", sortedEdges);
    // const pathsToDAC = findPaths("svr", "dac", sortedEdges);
    // const pathsFromFFT = findPaths("fft", "out", sortedEdges);
    // const pathsFromDAC = findPaths("dac", "out", sortedEdges);
    console.log(paths);
    // console.log("Paths to fft", pathsToFFT);
    // console.log("Paths to dac", pathsToDAC);
    // console.log("Paths from fft", pathsFromFFT);
    // console.log("Paths from dac", pathsFromDAC);
}

exec();