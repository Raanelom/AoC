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

PUZZLE_INPUT = [14, 8, 16, 0, 1, 17]
PUZZLE_INPUT_TEST = [0, 3, 6]


def init_last_index(puzzle_input):
    last_idx = {}
    for i in range(0,len(puzzle_input)-1):
        last_idx[puzzle_input[i]] = (i + 1)
    return last_idx


def memory_game():
    turns = PUZZLE_INPUT.copy()
    last_indices = init_last_index(PUZZLE_INPUT)
    next_spoken_number = -1
    last_spoken_number = turns[-1]
    for i in range(len(turns), 30000000):
        last_idx = last_indices.get(last_spoken_number, -1)
        if last_idx == -1:
            next_spoken_number = 0
        elif last_idx < i:
            diff = i - last_idx
            next_spoken_number = diff
        last_indices[last_spoken_number] = i
        last_spoken_number = next_spoken_number
    print(last_spoken_number)



def solution_a():
    memory_game()


solution_a()