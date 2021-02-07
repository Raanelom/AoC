from pathlib import Path
import os
import re
import math
import itertools
import more_itertools
import copy
import sys

def read_input_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return [list(line) for line in f.read().splitlines()]


def pad_seats_map(seats_map):
    for i in seats_map:
        i.insert(0, ".")
        i.append(".")
    seats_map.insert(0, ["." for seat in seats_map[0]])
    seats_map.append(["." for seat in seats_map[0]])
    return seats_map


def toggle_seat(seat_type, number_of_occupied):
    if seat_type == ".":
        return seat_type
    if seat_type == "#":
        return "L" if number_of_occupied >= 5 else seat_type
    if seat_type == "L":
        return "#" if number_of_occupied == 0 else seat_type


def get_surrounding_chairs(seats_map, row, col):
    adjacent_chairs = [seats_map[i][j] for i in range(row - 1, row + 2) for j in range(col - 1, col + 2) if not (i == row and col == j)]
    number_of_occupied = adjacent_chairs.count("#")
    return number_of_occupied


def get_visible_chairs(seats_map, row, col):
    indices = [-1, 0, 1] # determine direction on x- and y-axes
    directions = [[i, j] for i in indices for j in indices if not (i == 0 and j == 0)]
    visible_chairs = []
    for direction in directions:
        visible_chair = "."
        row_ext = row + direction[0]
        col_ext = col + direction[1]
        while(visible_chair == "." and 0 <= row_ext < len(seats_map) and 0 <= col_ext < len(seats_map[0])):
            visible_chair = seats_map[row_ext][col_ext]
            row_ext += direction[0]
            col_ext += direction[1]
        visible_chairs.append(visible_chair)
    number_of_occupied = visible_chairs.count("#")
    return number_of_occupied


def musical_chairs(seats_map):
    # Handle all changes simultaneously
    # Thus copy the initial array
    seats_map_new = copy.deepcopy(seats_map)
    for i in range(1, len(seats_map) - 1):
        for j in range(1, len(seats_map[i]) - 1):
            number_of_occupied = get_visible_chairs(seats_map, i, j)
            seats_map_new[i][j] = toggle_seat(seats_map[i][j], number_of_occupied)
    return seats_map_new


def count_occupied(seats_map):
    number_of_occupied = 0
    for seats in seats_map:
        number_of_occupied += seats.count("#")
    return number_of_occupied


seats_map = read_input_file()
seats_map = pad_seats_map(seats_map)
seats_map_prev = []
iteration = 0
while(seats_map_prev != seats_map):
    seats_map_prev = seats_map
    seats_map = musical_chairs(seats_map)
    iteration += 1

print(seats_map)
print(iteration)
print(count_occupied(seats_map))