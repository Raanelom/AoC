import { readFileSync } from 'fs';

// edges [(9, 7), (9, 3)],[(9, 7), (7, 7)],[(7, 3), (9, 3)],[(7, 3), (7, 7)] intersections [ [], [], [], [ Coordinate { x: 7, y: 5 } ] ]
// . . . . . . . . . .
// . . . . . . . . . .
// . . . . . . . . . .
// . . . . . . . x . x
// . . . . . . . . . .
// . . . . . . . I . .
// . . . . . . . . . .
// . . . . . . . x . x
// . . . . . . . . . .

// Thanks https://paulbourke.net/geometry/pointlineplane/javascript.txt
function intersect(a: Coordinate, b: Coordinate, c: Coordinate, d: Coordinate) {

	if ((a.x === b.x && a.y === b.y) || (c.x === d.x && c.y === d.y)) {
		return null
	}

	let denominator = ((d.y - c.y) * (b.x - a.x) - (d.x - c.x) * (b.y - a.y))
	if (denominator === 0) {
		return null
	}

	let ua = ((d.x - c.x) * (a.y - c.y) - (d.y - c.y) * (a.x - c.x)) / denominator
	let ub = ((b.x - a.x) * (a.y - c.y) - (b.y - a.y) * (a.x - c.x)) / denominator

	if (ua < 0 || ua > 1 || ub < 0 || ub > 1) {
		return null
	}

	let x = a.x + ua * (b.x - a.x)
	let y = a.y + ua * (b.y - a.y)

    const newCoord = new Coordinate(x, y);

    if (a.equals(newCoord) || b.equals(newCoord) || c.equals(newCoord) || d.equals(newCoord)) {
        // We only want a 100% intersection (no edges of edges)
        return null;
    }

	return new Coordinate(x, y);
}


class Coordinate {
    x: number;
    y: number;

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }
    
    area(other: Coordinate) {
        return (Math.abs(this.x - other.x) + 1) * (Math.abs(this.y - other.y) + 1);
    }

    toString() {
        return `(${this.x}, ${this.y})`;
    }

    equals(other: Coordinate) {
        return this.x === other.x && this.y === other.y;
    }
}

enum VerticalDirection {
    left,
    right,
    up,
    down,
    neutral
}

enum HorizontalDirection {
    left,
    right,
    neutral
}

class Edge {
    a: Coordinate;
    b: Coordinate;

    directionX: HorizontalDirection;
    directionY: VerticalDirection;

    constructor(a: Coordinate, b: Coordinate) {
        this.a = a;
        this.b = b;
        const xDiff = b.x - a.x;
        const yDiff = b.y - a.y;
        this.directionX = xDiff < 0 ? HorizontalDirection.left : xDiff > 0 ? HorizontalDirection.right : HorizontalDirection.neutral;
        this.directionY = yDiff < 0 ? VerticalDirection.up : yDiff > 0 ? VerticalDirection.down : VerticalDirection.neutral;
    }

    isVerticalLine = () => this.a.x === this.b.x;
    isHorizontalLine = () => this.a.y === this.b.y;

    isInVerticalRange = (other: Coordinate) => {
        const minY = Math.min(this.a.y, this.b.y);
        const maxY = Math.max(this.a.y, this.b.y);

        return other.y <= maxY && other.y >= minY;
    }

    isInHorizontalRange(other: Coordinate) {
        const minX = Math.min(this.a.x, this.b.x);
        const maxX = Math.max(this.a.x, this.b.x);
        
        return other.x <= maxX && other.x >= minX;
    }

    intersect = (other: Edge) => {
        if(this.directionX === HorizontalDirection.neutral && other.directionX === HorizontalDirection.neutral
            || this.directionY === VerticalDirection.neutral && other.directionY === VerticalDirection.neutral
        ) {
            return null;
        }
        return intersect(this.a, this.b, other.a, other.b);
        
    }

    toString = () => {
        return `[${this.a}, ${this.b}]`
    }
}

class Area extends Edge {
    area: number;

    constructor(a: Coordinate, b: Coordinate) {
        super(a, b);
        this.area = a.area(b);
    }

    getCorners = () => {
        if (this.isVerticalLine() || this.isHorizontalLine()) {
            return [this.a, this.b];
        }
        return [this.a, this.b, new Coordinate(this.a.x, this.b.y), new Coordinate(this.b.x, this.a.y)]
    }

    getEdges = () => {
        if (this.isVerticalLine() || this.isHorizontalLine()) {
            return [new Edge(this.a, this.b)];
        }
        return [new Edge(this.a, new Coordinate(this.a.x, this.b.y)),
            new Edge(this.a, new Coordinate(this.b.x, this.a.y)),
            new Edge(this.b, new Coordinate(this.a.x, this.b.y)),
            new Edge(this.b, new Coordinate(this.b.x, this.a.y))];
    }
}

const coordinates: Coordinate[] = readFileSync('./example_input', 'utf-8').trim().split('\n')
    .map((point: string) => {
        const coordinates = point.split(",");
        return new Coordinate(parseInt(coordinates[0]), parseInt(coordinates[1]))
    });

const areas: Area[] = [];
const edges: Edge[] = [];

for (let i = 0; i < coordinates.length; i++) {
    for(let j = (i + 1); j < coordinates.length; j++) {
        const cornerOne = coordinates[i];
        const cornerTwo = coordinates[j];
        areas.push(new Area(cornerOne, cornerTwo));
    }
    edges.push(new Edge(coordinates[i], coordinates[(i + 1) % coordinates.length]));
}

const verticalLines = edges.filter((a) => a.isVerticalLine());
const horizontalLines = edges.filter((a) => a.isHorizontalLine());

const validAreas: Area[] = [];
const knownCorners = new Map();

for (const area of areas) {
    const edges = area.getEdges();
    const interSections = edges.map((edge) => {
        const compareWith = edge.isVerticalLine() ? horizontalLines : verticalLines;
        const intersections = compareWith
            .filter((otherEdge) => !!otherEdge.intersect(edge))
            .map((otherEdge) => otherEdge.intersect(edge)!);
        return intersections;
    });
    console.log("edges", edges.toString(), "intersections", interSections)
    const corners = area.getCorners();
    let validCorners = true;
    /// TODO: do not only check corners, but every coordinate on the line => or check if there's any additional intersection with another line
    for (const corner of corners) {
        const cornerKey = JSON.stringify(corner);
        if (knownCorners.has(cornerKey)) {
            validCorners = knownCorners.get(cornerKey)!;
            if (!validCorners) {
                break;
            }
            continue;
        }
        const verticalAxes = verticalLines
            .filter((a) => a.isInVerticalRange(corner))
            .filter((current, index, list) => {
            // check if there are subsequent horizontal axes with identical directions and overlapping x coordinates / are connected by a vertical line
            // Preserve only the closest
            const next = list[index + 1 % list.length];
            const prev = list.slice(index - 1)[0];
            return !(current.directionX === next?.directionX
                // Only remove this edge if it's further than the next one
                && Math.abs(current.a.x - corner.x) > Math.abs(next.a.x - corner.x)
                && (current.b.y === next?.a.y && current.b.y === corner.y) 
                && horizontalLines.some((vEdge) => vEdge.a.equals(current.b) && vEdge.b.equals(next.a)))
            && !(current.directionX === prev?.directionX
                // Only remove this edge if it's further than the previous one
                && Math.abs(current.a.x - corner.x) > Math.abs(prev.a.x - corner.x)
                // ... and is on the same y-coordinate (start + end)
                && (current.a.y === prev?.b.y && current.a.y === corner.y) 
                && horizontalLines.some((vEdge) => vEdge.a.equals(prev?.b) && vEdge.b.equals(current.a)));
        });
        const horizontalAxes = horizontalLines
            .filter((a) => a.isInHorizontalRange(corner))
            .filter((current, index, list) => {
            // check if there are subsequent horizontal axes with identical directions and overlapping x coordinates / are connected by a vertical line
            // Preserve only the closest
            const next = list[index + 1 % list.length];
            const prev = list.slice(index - 1)[0];
            return !(current.directionX === next?.directionX
                // Only remove this edge if it's further than the next one
                && Math.abs(current.a.y - corner.y) > Math.abs(next.a.y - corner.y)
                && (current.b.x === next?.a.x && current.b.x === corner.x) 
                && verticalLines.some((vEdge) => vEdge.a.equals(current.b) && vEdge.b.equals(next.a)))
            && !(current.directionX === prev?.directionX
                // Only remove this edge if it's further than the previous one
                && Math.abs(current.a.y - corner.y) > Math.abs(prev.a.y - corner.y)
                && (current.a.x === prev?.b.x && current.a.x === corner.x) 
                && verticalLines.some((vEdge) => vEdge.a.equals(prev?.b) && vEdge.b.equals(current.a)));
        });

        const verticalSelect = verticalAxes
            .map((area) => area.a.x)
            .sort((a, b) => b - a);
        const horizontalSelect = horizontalAxes
            .map((area) => area.a.y)
            .sort((a, b) => b - a);

        const alwaysValid = horizontalSelect.includes(corner.y) || verticalSelect.includes(corner.x);

        const validVertical = alwaysValid
            || !!verticalSelect.find((val, index) => val < corner.x && corner.x > (verticalSelect?.[index + 1] || -1) && index % 2 === 1);
        const validHorizontal = alwaysValid 
            || !!horizontalSelect.find((val, index) => val < corner.y && corner.y > (horizontalSelect?.[index + 1] || -1) && index % 2 === 1);
        
        // console.log("looking for vline for corner", corner, "in axes", verticalAxesSelect, verticalAxesSelect.find((val) => val < corner.x));
        // console.log("\nvalid vline", corner,  validVline);
        // console.log("valid hline", corner, validHline);
        validCorners = validCorners && validVertical && validHorizontal;
        knownCorners.set(cornerKey, validCorners);
        if (!validCorners) {
            console.log("\nInvalid corner", corner);
            break;
        }
    }
    if (validCorners) {
        validAreas.push(area);
    }
}

const sortedAreas = validAreas
    .sort((a, b) => b.area - a.area);

// console.log(sortedAreas);

console.log(sortedAreas[0]);
// Largest area: 4776100539
// Largest area pt 2: 4650063000 => invalid