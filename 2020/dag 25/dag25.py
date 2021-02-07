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
        return [int(line) for line in f.read().splitlines()]


def loop(base, subject_number):
    return (base * subject_number) % 20201227


def determine_loop_size(pk):
    sn = 7
    base = 1
    loop_size = 0
    while(base != pk):
        base = loop(base, sn)
        loop_size += 1
    return loop_size


def calculate_encryption_key(sn, loop_size):
    base = 1
    while(loop_size > 0):
        base = loop(base, sn)
        loop_size -= 1
    return base


def solution():
    card_pk, door_pk = read_input_file()
    # card_pk, door_pk = [5764801, 17807724]
    # loop sizes
    card_loopsize = determine_loop_size(card_pk)
    door_loopsize = determine_loop_size(door_pk)
    print(f"card loop size: {card_loopsize}")
    print(f"door loop size: {door_loopsize}")
    # encryption keys
    encryption_key_1 = calculate_encryption_key(door_pk, card_loopsize)
    encryption_key_2 = calculate_encryption_key(card_pk, door_loopsize)
    print(f"encryption keys are {encryption_key_1} {encryption_key_2}")


solution()