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

const coordinates: Coordinate[] = readFileSync('./input', 'utf-8').trim().split('\n\n')[0].split('\n')
    .map((point: string) => {
        const coordinates = point.split(",");
        return new Coordinate(parseInt(coordinates[0]!), parseInt(coordinates[1]!))
    });

const areas: Area[] = [];
const edges: Edge[] = [];

for (let i = 0; i < coordinates.length; i++) {
    for(let j = (i + 1); j < coordinates.length; j++) {
        const cornerOne = coordinates[i];
        const cornerTwo = coordinates[j];
        areas.push(new Area(cornerOne!, cornerTwo!));
    }
    edges.push(new Edge(coordinates[i]!, coordinates[(i + 1) % coordinates.length]!));
}

const verticalLines = edges.filter((a) => a.isVerticalLine());
const horizontalLines = edges.filter((a) => a.isHorizontalLine());

const validAreas: Area[] = [];
const knownCoordinates = new Map<string, boolean>();
const knownEdges = new Map<string, { source: Edge, intersectAt: Coordinate }[]>();

for (const area of areas) {
    const intersections = area.getEdges().map((edge) => {
        const edgeKey = JSON.stringify(edge);
        if (knownEdges.has(edgeKey)) {
            return knownEdges.get(edgeKey)!;
        }
        const intersections = edges
            .filter((otherEdge) => !!otherEdge.intersect(edge))
            .map((otherEdge) => ({ source: edge, intersectAt: otherEdge.intersect(edge)! }));
        knownEdges.set(edgeKey, intersections);
        return intersections;
    }).flat();
    
    const coordsToCheck = area.getCorners();
    let validCoordinates = true;
    for (const intersection of intersections) {
        const { source, intersectAt } = intersection;
        const maxY = Math.max(source.a.y, source.b.y);
        const minY = Math.min(source.a.y, source.b.y);
        const maxX = Math.max(source.a.x, source.b.x);
        const minX = Math.min(source.a.x, source.b.x);
        if (source.isVerticalLine()) {
            if (intersectAt.y < maxY) {
                coordsToCheck.push(new Coordinate(intersectAt.x, intersectAt.y + 1));
            }
            if (intersectAt.y > minY) {
                coordsToCheck.push(new Coordinate(intersectAt.x, intersectAt.y - 1));
            }
        }
        else {
            if (intersectAt.x < maxX) {
                coordsToCheck.push(new Coordinate(intersectAt.x + 1, intersectAt.y));
            }
            if (intersectAt.x > minX) {
                coordsToCheck.push(new Coordinate(intersectAt.x - 1, intersectAt.y));
            }
        }
    }
    for (const coordinate of coordsToCheck) {
        const coordKey = JSON.stringify(coordinate);
        if (knownCoordinates.has(coordKey)) {
            validCoordinates = knownCoordinates.get(coordKey)!;
            if (!validCoordinates) {
                break;
            }
            continue;
        }
        const verticalAxes = verticalLines
            .filter((a) => a.isInVerticalRange(coordinate))
            .filter((current, index, list) => {
            // check if there are subsequent horizontal axes with identical directions and overlapping x coordinates / are connected by a vertical line
            // Preserve only the closest
            const next = list[index + 1 % list.length];
            const prev = list.slice(index - 1)[0];
            return !(current.directionX === next?.directionX
                // Only remove this edge if it's further than the next one
                && Math.abs(current.a.x - coordinate.x) > Math.abs(next.a.x - coordinate.x)
                && (current.b.y === next?.a.y && current.b.y === coordinate.y) 
                && horizontalLines.some((vEdge) => vEdge.a.equals(current.b) && vEdge.b.equals(next.a)))
            && !(current.directionX === prev?.directionX
                // Only remove this edge if it's further than the previous one
                && Math.abs(current.a.x - coordinate.x) > Math.abs(prev.a.x - coordinate.x)
                // ... and is on the same y-coordinate (start + end)
                && (current.a.y === prev?.b.y && current.a.y === coordinate.y) 
                && horizontalLines.some((vEdge) => vEdge.a.equals(prev?.b) && vEdge.b.equals(current.a)));
        });
        const horizontalAxes = horizontalLines
            .filter((a) => a.isInHorizontalRange(coordinate))
            .filter((current, index, list) => {
            // check if there are subsequent horizontal axes with identical directions and overlapping x coordinates / are connected by a vertical line
            // Preserve only the closest
            const next = list[index + 1 % list.length];
            const prev = list.slice(index - 1)[0];
            return !(current.directionX === next?.directionX
                // Only remove this edge if it's further than the next one
                && Math.abs(current.a.y - coordinate.y) > Math.abs(next.a.y - coordinate.y)
                && (current.b.x === next?.a.x && current.b.x === coordinate.x) 
                && verticalLines.some((vEdge) => vEdge.a.equals(current.b) && vEdge.b.equals(next.a)))
            && !(current.directionX === prev?.directionX
                // Only remove this edge if it's further than the previous one
                && Math.abs(current.a.y - coordinate.y) > Math.abs(prev.a.y - coordinate.y)
                && (current.a.x === prev?.b.x && current.a.x === coordinate.x) 
                && verticalLines.some((vEdge) => vEdge.a.equals(prev?.b) && vEdge.b.equals(current.a)));
        });

        const verticalSelect = verticalAxes
            .map((area) => area.a.x)
            .sort((a, b) => b - a);
        const horizontalSelect = horizontalAxes
            .map((area) => area.a.y)
            .sort((a, b) => b - a);

        const alwaysValid = horizontalSelect.includes(coordinate.y) || verticalSelect.includes(coordinate.x);

        const validVertical = alwaysValid
            || (coordinate.x < (verticalSelect[0] || -1)
            && !!verticalSelect.find((val, index, items) => val < coordinate.x && coordinate.x > (items?.[index + 1] || -1) && index % 2 === 1));
        const validHorizontal = alwaysValid 
            || (coordinate.y < (horizontalSelect[0] || -1)
            || !!horizontalSelect.find((val, index, items) => val < coordinate.y && coordinate.y > (items?.[index + 1] || -1) && index % 2 === 1));
        
        validCoordinates = validCoordinates && validVertical && validHorizontal;
        knownCoordinates.set(coordKey, validCoordinates);
        if (!validCoordinates) {
            break;
        }
    }
    if (validCoordinates) {
        validAreas.push(area);
    }
}

const sortedAreas = validAreas
    .sort((a, b) => b.area - a.area);


console.log(sortedAreas[0]);
// Largest area: 4776100539
// Largest area pt 2: 4650063000 => invalid, too high
// Largest area pt 2: 2997770932 => invalid, too high
// Largest area pt 3: 1476550548 => Yay