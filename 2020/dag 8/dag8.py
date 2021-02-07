from pathlib import Path
import os
import re
import math
import itertools
import more_itertools
import copy


def read_input_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return f.read().splitlines()


def parse_instruction(instruction_raw):
    return [instruction_raw[0], int(instruction_raw[1])]


def parse_boot_code(boot_code_raw):
    return [parse_instruction(instruction_raw.split(' ')) for instruction_raw in boot_code_raw]


def perform_boot_code_mutations(arg_boot_code):
    is_terminated = False
    jmp_nop_idx = 0
    accumulator = -1
    while not is_terminated:
        boot_code = copy.deepcopy(arg_boot_code)
        while boot_code[jmp_nop_idx][0] not in ["nop", "jmp"]:
            jmp_nop_idx += 1
        boot_code[jmp_nop_idx][0] = "nop" if boot_code[jmp_nop_idx][0] == "jmp" else "jmp"
        jmp_nop_idx += 1
        accumulator, is_terminated = execute_boot_code(boot_code)
    return accumulator


def execute_boot_code(boot_code):
    executed_boot_code = {}
    accumulator = 0
    instruction_idx = 0
    while (not executed_boot_code.get(instruction_idx, False)) and instruction_idx != len(boot_code):
        instruction = boot_code[instruction_idx]
        executed_boot_code[instruction_idx] = True
        if instruction[0] == "nop":
            instruction_idx += 1
        elif instruction[0] == "acc":
            accumulator += instruction[1]
            instruction_idx += 1
        elif instruction[0] == "jmp":
            instruction_idx += instruction[1]
        else:
            raise Exception("Err")
    return accumulator, instruction_idx == len(boot_code)
        


boot_code_raw = read_input_file()
boot_code = parse_boot_code(boot_code_raw)
accumulator = execute_boot_code(boot_code)
print(accumulator)
accumulator_updated_boot_code = perform_boot_code_mutations(boot_code)
print(accumulator_updated_boot_code)