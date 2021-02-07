from pathlib import Path
import os
import re
import math
import itertools
import more_itertools
import copy
import sys
import ast
import collections
from functools import reduce
import regex
from pprint import pprint


def read_input_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return [[c for c in line] for line in f.read().splitlines()]


def parse_input_file(directions_input):
    new_directions = []
    for direction in directions_input:
        new_directions.append([])
        i = 0
        while i < len(direction):
            new_directions[-1].append(direction[i])
            if direction[i] not in ["e", "w"]:
                new_directions[-1][-1] += direction[i+1]
                i += 1
            i += 1
    return new_directions


def discover_tile(from_tile, to_direction):
    to_tile = (0,0)
    if to_direction == "nw":
        to_tile = (from_tile[0]-.5, from_tile[1]+.5)
    elif to_direction == "ne":
        to_tile = (from_tile[0]+.5, from_tile[1]+.5)
    elif to_direction == "sw":
        to_tile = (from_tile[0]-.5, from_tile[1]-.5)
    elif to_direction == "se":
        to_tile = (from_tile[0]+.5, from_tile[1]-.5)
    elif to_direction == "w":
        to_tile = (from_tile[0]-1.0, from_tile[1])
    elif to_direction == "e":
        to_tile = (from_tile[0]+1.0, from_tile[1])
    else: 
        print("error")
    return to_tile


def flip_tile(color):
    return "w" if color == "b" else "b"


def get_neighbour_positions(pos):
    return [(pos[0]-.5, pos[1]+.5), (pos[0]+.5, pos[1]+.5), (pos[0]-.5, pos[1]-.5),
            (pos[0]+.5, pos[1]-.5), (pos[0]-1.0, pos[1]), (pos[0]+1.0, pos[1])]


def add_neighbours(tiles):
    new_tiles = tiles.copy()
    for pos, old_color in tiles.items():
        for neighbour in get_neighbour_positions(pos):
            new_tiles[neighbour] = new_tiles.get(neighbour, "w")
    return new_tiles


def flip_all_tiles(old_tiles):
    new_tiles = old_tiles.copy()
    for pos, old_color in old_tiles.items():
        neighbour_colors = [old_tiles.get(neighbour_pos, "w") for neighbour_pos in get_neighbour_positions(pos)]
        black_tiles = sum([1 for color in neighbour_colors if color == "b"])
        if old_color == "b" and (black_tiles == 0 or black_tiles > 2):
            new_tiles[pos] = "w"
        elif old_color == "w" and black_tiles == 2:
            new_tiles[pos] = "b"
    return new_tiles


def solution():
    directions_input_raw = read_input_file()
    directions = parse_input_file(directions_input_raw)
    next_tile = (0.0, 0.0)
    tiles = {}
    tiles[next_tile] = "w"
    final_colors = {}
    while len(directions) > 0:
        next_tile = (0.0, 0.0)
        next_path = directions.pop(0)
        while len(next_path) > 0:
            next_tile = discover_tile(next_tile, next_path.pop(0))
        tiles[next_tile] = flip_tile(tiles.get(next_tile, "w"))
    print(sum([1 for color in tiles.values() if color == "b"]))
    new_tiles = tiles.copy()
    for day in range(0, 100):
        new_tiles = add_neighbours(new_tiles)
        new_tiles = flip_all_tiles(new_tiles)
        print(sum([1 for color in new_tiles.values() if color == "b"]))


solution()