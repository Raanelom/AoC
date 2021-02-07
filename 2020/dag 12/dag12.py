from pathlib import Path
import os
import re
import math
import itertools
import more_itertools
import copy
import sys

DIRECTION_ACTIONS_PATTERN = r"^([A-Z]{1})([0-9]*)$"
DIRECTIONS = ['E', 'S', 'W', 'N']
DIRECTIONS_REVERSE = ['E', 'N', 'W', 'S']

def split_direction_actions(string_to_split):
    return re.match(DIRECTION_ACTIONS_PATTERN, string_to_split).groups()


def read_input_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return [line for line in f.read().splitlines()]


def parse_direction_actions(input_direction_actions):
    return [split_direction_actions(direction_action) for direction_action in input_direction_actions]


def facing_next(current_direction, left_or_right, degrees):
    steps = int(degrees / 90)
    current_direction_idx = DIRECTIONS.index(current_direction) if left_or_right == 'R' else DIRECTIONS_REVERSE.index(current_direction)
    steps += current_direction_idx
    res = DIRECTIONS[steps % len(DIRECTIONS)] if left_or_right == 'R' else DIRECTIONS_REVERSE[steps % len(DIRECTIONS)]
    print(f"moving from {current_direction} with {left_or_right} and degrees {degrees} to {res}")
    return res


def rotate_waypoint(facing, left_or_right, degrees):
    res = [facing_next(direction, left_or_right, degrees) for direction in facing]
    return res


def determine_direction(action_directions):
    facing = "E"
    position = [0, 0]
    for direction, action in action_directions:
        action_nr = int(action)
        if direction == 'N' or (direction == 'F' and facing == 'N'):
            position[1] += action_nr
        elif direction == 'S' or (direction == 'F' and facing == 'S'):
            position[1] -= action_nr
        elif direction == 'E' or (direction == 'F' and facing == 'E'):
            position[0] += action_nr
        elif direction == 'W' or (direction == 'F' and facing == 'W'):
            position[0] -= action_nr
        elif direction in ['L', 'R']:
            facing = facing_next(facing, direction, action_nr)
        else:
            print(f"wtf, {direction} is not valid")
    print(position)


def solution_a():
    action_directions_file = read_input_file()
    action_directions = parse_direction_actions(action_directions_file)
    determine_direction(action_directions)


def determine_waypoint(action_directions):
    waypoint = [10, 1]
    position = [0, 0] # [0] = east/west, [1] = north/south
    facing = ['E', 'N']
    north_south_idx = 1
    east_west_idx = 0
    is_north = 1
    is_east = 1
    for direction, action in action_directions:
        action_nr = int(action)
        is_north = 1 if facing[north_south_idx] == 'N' else -1
        is_east = 1 if facing[east_west_idx] == 'E' else -1
        print(f"facing {facing}")
        if direction == 'F':
            position[0] = position[0] + (action_nr*waypoint[east_west_idx]*is_east)
            position[1] = position[1] + (action_nr*waypoint[north_south_idx]*is_north)
        elif direction == 'N':
            waypoint[north_south_idx] += (action_nr*is_north)
        elif direction == 'S':
            waypoint[north_south_idx] -= (action_nr*is_north)
        elif direction == 'E':
            waypoint[east_west_idx] += (action_nr*is_east)
        elif direction == 'W':
            waypoint[east_west_idx] -= (action_nr*is_east)
        elif direction in ['L', 'R']:
            facing = rotate_waypoint(facing, direction, action_nr)
            north_south_idx = 0 if facing[0] in ['S', 'N'] else 1
            east_west_idx = 0 if facing[0] in ['E', 'W'] else 1
            print(f"new indices north_south for {north_south_idx} and east_west {east_west_idx} for {facing}")
        else:
            print(f"wtf, {direction} is not valid")
    print(position)
    print(abs(position[0]) + abs(position[1]))


def solution_b():
    action_directions_file = read_input_file()
    action_directions = parse_direction_actions(action_directions_file)
    determine_waypoint(action_directions)

solution_b()

