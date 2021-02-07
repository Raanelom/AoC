from pathlib import Path
import os
import re
import math
import itertools
import more_itertools
import copy
import sys
import ast
from functools import reduce


def read_input_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return [line for line in f.read().splitlines()]


def is_active(coordinate):
    return coordinate == "#"


def init_coords(initial_map):
    coordinate_map = {}
    for y in range(len(initial_map)):
        for x in range(len(initial_map[y])):
            coordinate_map[(x, y, 0, 0)] = is_active(initial_map[x][y])
    return coordinate_map


def discover_neighbours(coordinates_map, coords):
    x, y, z, w = coords
    for x2 in range(x-1, x+2):
        for y2 in range(y-1, y+2):
            for z2 in range(z-1, z+2):
                for w2 in range(w-1, w+2):
                    coordinates_map[(x2, y2, z2, w2)] = coordinates_map.get((x2, y2, z2, w2), False)


def find_neighbours(coordinates_map, coords):
    x, y, z, w = coords
    neighbour_tuples = []
    for x2 in range(x-1, x+2):
        for y2 in range(y-1, y+2):
            for z2 in range(z-1, z+2):
                for w2 in range(w-1, w+2):
                    if (x2, y2, z2, w2) != coords:
                        neighbour_tuples.append((x2, y2, z2, w2))
    return neighbour_tuples


def new_state(neighbours, coordinates_map, updated_map, is_active):
    active_count = 0
    for neighbour in neighbours:
        active_count += coordinates_map.get(neighbour, False)
    if is_active:
        return active_count == 2 or active_count == 3
    else:
        return active_count == 3


def perform_cycle(coordinates_map):
    updated_map = copy.deepcopy(coordinates_map)
    # First discover neighbours and add them to the dictionary
    for key, val in copy.deepcopy(coordinates_map).items():
        discover_neighbours(coordinates_map, key)
    # Then perform the actual update
    for key, val in coordinates_map.items():
        neighbour_tuples = find_neighbours(coordinates_map, key)
        updated_map[key] = new_state(neighbour_tuples, coordinates_map, updated_map, val)
    return updated_map


def solution_a():
    initial_map = read_input_file()
    coordinates_map = init_coords(initial_map)
    for i in range(0, 6):
        coordinates_map = perform_cycle(coordinates_map)
    print(sum(coordinates_map.values()))
    
    
def solution_b():
    initial_map = read_input_file()
    coordinates_map = init_coords(initial_map)
    for i in range(0, 6):
        coordinates_map = perform_cycle(coordinates_map)
    print(sum(coordinates_map.values()))

solution_b()