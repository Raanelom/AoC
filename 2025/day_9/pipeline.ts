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
        const otherCorners = [
            new Coordinate(cornerOne.x, cornerTwo.y),
            new Coordinate(cornerTwo.x, cornerOne.y)
        ];
        // let isValid = true;
        // for (const corner of otherCorners) {
        //     const verticalBefore = closestBefore(verticalLines, corner.x);
        //     const verticalAfter = closestAfter(verticalLines, corner.x);
        //     // const isEvenX = verticalLines.indexOf(verticalBefore) - startX % 2 === 0;
        //     // TODO: determine if the line before and after are odd/even compared to start point to determine if it's between two valid lines
        //     console.log("Vertical before", corner.x, verticalBefore);
        //     console.log("Vertical after", corner.x, verticalAfter);
        // }
        areas.push(new Area(cornerOne, cornerTwo));
        // console.log(areas);
        // console.log("\nNew corners:", cornerOne, cornerTwo, cornerThree, cornerFour);
    }
}

const verticalLines = areas.filter((a) => a.isVerticalLine());
const horizontalLines = areas.filter((a) => a.isHorizontalLine());

const baseVertical = coordinates[0].x;
const baseHorizontal = coordinates[0].y;

// console.log(verticalLines);

let validAreas: Area[] = [];

for (const area of areas) {
    const corners = area.getCorners();
    console.log("\nProcess area", area.a, area.b);
    for (const corner of corners) {
        const verticalAxes = verticalLines
            .filter((a) => a.isInVerticalRange(corner))
            .map((area) => area.a.x);
        const horizontalAxes = horizontalLines
            .filter((a) => a.isInHorizontalRange(corner))
            .map((area) => area.a.y);
        
        const verticalAxesSelect = baseVertical >= corner.x ? verticalAxes
            .filter((val) => val <= baseVertical)
            .sort((a, b) => b - a) 
            : verticalAxes
                .filter((val) => val >= baseVertical)
                .sort((a, b) => a - b);
        // console.log("Vertical axes select:", corner, verticalAxesSelect);
        const horizontalAxesSelect = baseHorizontal >= corner.y ? horizontalAxes
            .filter((val) => val <= baseHorizontal) 
            .sort((a, b) => b - a)
            : horizontalAxes
                .filter((val) => val >= baseHorizontal)
                .sort((a, b) => a - b);
        // console.log("Horizontal axes select:", corner, horizontalAxesSelect);
        // const hlineIndex = horizontalAxesSelect.indexOf(horizontalAxesSelect.find((val, index) => val <= corner.y && val > horizontalAxesSelect[index + 1]) || Infinity);
        // const vlineIndex = verticalAxesSelect.indexOf(verticalAxesSelect.find((val, index) => val <= corner.x && val > horizontalAxesSelect[index + 1]) || Infinity);
        const hlines = horizontalAxesSelect.find((val, index) => val <= corner.y && val > horizontalAxesSelect[index + 1]);
        const vlines = verticalAxesSelect.find((val, index) => val <= corner.x && val > horizontalAxesSelect[index + 1]);
        console.log("looking for vline for corner", corner, "in axes", verticalAxesSelect);
        console.log("vlines", vlines, "hlines", hlines);
        // if (hlineIndex > -1 && vlineIndex > -1) {
        // const validCorner = hlineIndex % 2 === 1 && vlineIndex % 2 === 1;
        // if (validCorner) {
        //     console.log("Got valid corner", corner);
        // }
        // }
    }
    // Let's check if:
    // The area's corner are between two horizontal lines and two vertical lines
    // And whether
}

// const sortedAreas = [...areas]
//     .sort((a, b) => b.area - a.area);

// console.log(sortedAreas);

// console.log(sortedAreas[0]);
// Largest area: 4776100539