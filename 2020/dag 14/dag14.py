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


def get_mask_val(mask):
    return mask.split(" = ")[1]


def get_memory_idx_and_val(mem):
    var, val = mem.split(" = ")
    list_idx = int(var.split("[")[1].split("]")[0])
    return list_idx, int(val)


def clear_bits(val_to_clear, mask_0_positions):
    val = val_to_clear
    for position in mask_0_positions:
        val = val & ~(1<<position)
    return val


def parse_mask(mask):
    mask_1_positions = [pos for pos, char in enumerate(reversed(mask)) if char == '1']
    mask_1 = 0
    for pos in mask_1_positions:
        mask_1 = mask_1 | (1<<pos)
    mask_0_positions = [pos for pos, char in enumerate(reversed(mask)) if char == '0']
    return mask_1, mask_0_positions


def perform_memory_update(mem_val, mask_1, mask_0_positions):
    updated_val = clear_bits(mem_val, mask_0_positions)
    updated_val = updated_val | mask_1
    return updated_val


def load_operations_in_memory(input_file):
    mask = ""
    mask_1 = 0
    mask_0_positions = []
    mem = {}
    for line in input_file:
        if len(line) == 43:
            mask = get_mask_val(line)
            mask_1, mask_0_positions = parse_mask(mask)
        else:
            mem_idx, mem_val = get_memory_idx_and_val(line)
            mem[mem_idx] = perform_memory_update(mem_val, mask_1, mask_0_positions)
    return mem
        

def solution_a():
    input_file = read_input_file()
    init_program = load_operations_in_memory(input_file)
    print(sum(init_program.values()))


def parse_mask_v2(mask):
    mask_1_positions = [pos for pos, char in enumerate(reversed(mask)) if char == '1']
    mask_1 = 0
    for pos in mask_1_positions:
        mask_1 = mask_1 | (1<<pos)
    mask_X_positions = [pos for pos, char in enumerate(reversed(mask)) if char == 'X']
    return mask_1, mask_X_positions


def set_bit(bit, val):
    return val | (1<<bit)

def clear_bit(bit, val):
    return val & ~(1<<bit)


def apply_floating_position(val, positions, res):
    floating_pos = positions.pop()
    pos_val = set_bit(floating_pos, val)
    neg_val = clear_bit(floating_pos, val)
    res.append(pos_val)
    res.append(neg_val)
    if len(positions) > 0:
        apply_floating_position(pos_val, positions.copy(), res)
        apply_floating_position(neg_val, positions.copy(), res)
    return res


def apply_mask(mem, mem_val, mem_idx, mask_1, mask_X_positions):
    updated_val = mem_idx | mask_1
    idx_values = []
    apply_floating_position(updated_val, mask_X_positions, idx_values)
    for idx in idx_values:
        mem[idx] = mem_val


def load_program_2(input_file):
    mask = ""
    mask_1 = 0
    mask_X = []
    mem = {}
    for line in input_file:
        if len(line) == 43:
            mask = get_mask_val(line)
            mask_1, mask_X_positions = parse_mask_v2(mask)
        else:
            mem_idx, mem_val = get_memory_idx_and_val(line)
            apply_mask(mem, mem_val, mem_idx, mask_1, mask_X_positions.copy())
    return mem


def solution_b():
     input_file = read_input_file()
     init_program = load_program_2(input_file)
     print(init_program)
     print(sum(init_program.values()))

solution_b()