import { readFileSync } from 'fs';

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
}

const coordinates: Coordinate[] = readFileSync('./example_input', 'utf-8').trim().split('\n')
    .map((point: string) => {
        const coordinates = point.split(",");
        return new Coordinate(parseInt(coordinates[0]), parseInt(coordinates[1]))
    });

const areas: Set<Area> = new Set();

for (let i = 0; i < coordinates.length; i++) {
    for(let j = (i + 1); j < coordinates.length; j++) {
        areas.add(new Area(coordinates[i], coordinates[j]));
    }
}

const sortedAreas = [...areas]
    .sort((a, b) => b.area - a.area);

console.log(sortedAreas[0]);
// Largest area: 4776100539