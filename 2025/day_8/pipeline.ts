import { readFileSync } from 'fs';

class Vector {
    x: number;
    y: number;
    z: number;

    constructor(x: number, y: number, z: number) {
        this.x = x;
        this.y = y;
        this.z = z;
    }
    
    diff(other: Vector) {
        return Math.sqrt(Math.pow(this.x - other.x, 2) + Math.pow(this.y - other.y, 2) + Math.pow(this.z - other.z, 2));
    }

    toString() {
        return `x: ${this.x}, y: ${this.y}, z: ${this.z}`
    }

    equals(other: Vector) {
        return this.x === other.x && this.y === other.y && this.z == other.z;
    }
}

class Edge {
    vectors: Vector[] = [];
    distance: number;

    constructor(to: Vector, from: Vector) {
        this.vectors = [to, from].sort();
        this.distance = to.diff(from);
    }

    toKey() {
        return JSON.stringify(this);
    }
    static toValue(key: string): Edge {
        const res: Edge = JSON.parse(key);
        return res;
    }
}

const nodes: Vector[] = readFileSync('./example_input', 'utf-8')
    .trim()
    .split('\n')
    .map((vector: string) => vector.split(",").map((no) => parseInt(no)))
    .map((vector: number[]) => new Vector(vector[0], vector[1], vector[2]));
// const graph = new Map<Vector, { to: Vector, distance: number }[]>();
const edges: Set<string> = new Set();

for (const thisVector of nodes) {
    for (const otherVector of nodes.filter((v) => v !== thisVector)) {
        // const thisDistances = graph.get(thisVector) || [];
        // thisDistances.push({ to: otherVector, distance: thisVector.diff(otherVector)})
        // graph.set(thisVector, thisDistances.sort((a, b) => a.distance - b.distance));

        const edge = new Edge(thisVector, otherVector);
        edges.add(edge.toKey());
    }
}

// const shortedDistance = [...graph].map((dist) => {
//     return { from: dist[0], to: dist[1][0].to, dist: dist[1][0].distance };
// }).sort((a, b) => a.dist - b.dist);

const shortestEdges = [...edges].map((edge) => Edge.toValue(edge)).sort((a, b) => a.distance - b.distance);

const circuits = [...nodes.map((vector) => [vector])];

const NO_OF_CIRCUITS = 10;

for(let i = 0; i < NO_OF_CIRCUITS; i++) {
    const shortestEdge = shortestEdges.shift();
    if (!shortestEdge) {
        throw new Error("No more shortest edges left");
    }
    // console.log("\n\nSTART with edge", i);
    // console.log(shortestEdge);
    const vectors = shortestEdge.vectors;
    const from = circuits.find((c) => c.find((v) => v.equals(vectors[0])));
    if (!from) {
        throw new Error("Missing From; Expected to find at least one circuit", from);
    }
    const indexFrom = circuits.indexOf(from);

    const to = circuits.find((c) => c.find((v) => v.equals(vectors[1])));
    if (!to) {
        throw new Error("Missing To; Expected to find at least one circuit", to);
    }
    const indexTo = circuits.indexOf(to);
    if (indexTo === indexFrom) {
        // Do nothing
        console.log("Same index");
        continue;
    }
    const toVectors = circuits.splice(indexTo, 1)[0];
    const fromVectors = circuits.splice(circuits.indexOf(from), 1)[0];
    circuits.push([...toVectors, ...fromVectors]);
    circuits.sort((a, b) => b.length - a.length);
    // console.log(`Circuits after step ${i}:`, circuits);
    // console.log("Circuits length", circuits.length);
}

// console.log(shortestEdges);
// console.log(input[0].diff(input[1]))
console.log(circuits.slice(0, 3).map((v) => v.length).reduce((acc, v) => acc *= v));