import { readFileSync } from 'fs';

const closestAfter = (set: number[], axis: number) => {
    for (const no of set) {
        // Return first value
        if (no - axis >= 0) {
            return no;
        }
    }
    return Infinity;
}

const closestBefore = (set: number[], axis: number) => {
    let previous = Infinity;
    for (const no of set) {
        if (no - axis >= 0) {
            return previous;
        }
        previous = no;
    }
    return Infinity;
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
        return `x: ${this.x}, y: ${this.y}`;
    }

    equals(other: Coordinate) {
        return this.x === other.x && this.y === other.y;
    }
}

class Area {
    a: Coordinate;
    b: Coordinate;
    area: number;

    constructor(a: Coordinate, b: Coordinate) {
        this.a = a;
        this.b = b;
        this.area = a.area(b);
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

    getCorners = () => {
        if (this.isVerticalLine() || this.isHorizontalLine()) {
            return [this.a, this.b];
        }
        return [this.a, this.b, new Coordinate(this.a.x, this.b.y), new Coordinate(this.b.x, this.a.y)]
    }
}

const coordinates: Coordinate[] = readFileSync('./example_input', 'utf-8').trim().split('\n')
    .map((point: string) => {
        const coordinates = point.split(",");
        return new Coordinate(parseInt(coordinates[0]), parseInt(coordinates[1]))
    });

const areas: Area[] = [];

for (let i = 0; i < coordinates.length; i++) {
    for(let j = (i + 1); j < coordinates.length; j++) {
        const cornerOne = coordinates[i];
        const cornerTwo = coordinates[j];
        areas.push(new Area(cornerOne, cornerTwo));
    }
}

const verticalLines = areas.filter((a) => a.isVerticalLine());
const horizontalLines = areas.filter((a) => a.isHorizontalLine());

const validAreas: Area[] = [];
const knownCorners = new Map();

for (const area of areas) {
    const corners = area.getCorners();
    let validCorners = true;
    for (const corner of corners) {
        const cornerKey = JSON.stringify(corner);
        if (knownCorners.has(cornerKey)) {
            validCorners = knownCorners.get(cornerKey)!;
            continue;
        }
        const verticalAxes = verticalLines
            .filter((a) => a.isInVerticalRange(corner))
            .map((area) => area.a.x)
            .sort((a, b) => b - a);
        const horizontalAxes = horizontalLines
            .filter((a) => a.isInHorizontalRange(corner))
            .map((area) => area.a.y)
            .sort((a, b) => b - a);

        const alwaysValid = horizontalAxes.includes(corner.y) || verticalAxes.includes(corner.x);
        const validHline = alwaysValid 
            || !!horizontalAxes.find((val, index) => val < corner.y && corner.y > (horizontalAxes?.[index + 1] || -1) && index % 2 === 1);
        const validVline = alwaysValid
            || !!verticalAxes.find((val, index) => val < corner.x && corner.x > (verticalAxes?.[index + 1] || -1) && index % 2 === 1);
        // console.log("looking for vline for corner", corner, "in axes", verticalAxesSelect, verticalAxesSelect.find((val) => val < corner.x));
        // console.log("\nvalid vline", corner,  validVline);
        // console.log("valid hline", corner, validHline);
        knownCorners.set(cornerKey, validVline && validHline);
        if (validVline && validHline) {
            knownCorners.set(cornerKey, true);
            // console.log("Valid corner", corner);
        }
        else {
            console.log("Invalid corner", corner, "vline", validVline, "hline", validHline);
            console.log(verticalAxes);
            console.log(verticalAxes.find((val, index) => val < corner.y && corner.x > (verticalAxes?.[index + 1] || -1)));
            validCorners = false;
            knownCorners.set(cornerKey, validCorners);
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