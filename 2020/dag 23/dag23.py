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


CUPS = 685974213
TEST_CUPS = 389125467

MAX_CUP = 1000000
MIN_CUP = 1


def parse_cups():
    cups = [int(cup) for cup in str(CUPS)]
    max_cup = max(cups)
    cups_dict = {}
    # Use first part of cups array
    for i in range(len(cups)-1):
        cups_dict[cups[i]] = cups[i + 1]
    # after 9 comes 10
    cups_dict[cups[-1]] = max_cup + 1
    # Fill dict with values from 10..1000000
    for generated_val in range(max_cup + 1, MAX_CUP):
        cups_dict[generated_val] = generated_val + 1
    # And close the linked list
    cups_dict[MAX_CUP] = cups[0]
    return cups_dict, cups[0]


def get_three_cups_tail(current_cup, cups_dict):
    cups = []
    next_cup = cups_dict[current_cup]
    while len(cups) != 3:
        cups.append(next_cup)
        next_cup = cups_dict[next_cup]
    # update linked list
    cups_dict[current_cup] = next_cup
    return cups


def get_destination_cup(current_cup, three_cups_tail):
    destination_cup = current_cup - 1
    while destination_cup in three_cups_tail or destination_cup < MIN_CUP:
        destination_cup = destination_cup - 1 if destination_cup > MIN_CUP else MAX_CUP
    return destination_cup


def move_three_cups_tail(cups_dict, destination_cup, three_cups_tail):
    cups_dict[three_cups_tail[-1]] = cups_dict[destination_cup]
    cups_dict[destination_cup] = three_cups_tail[0]
    while len(three_cups_tail) > 1:
        insert_next = three_cups_tail.pop()
        cups_dict[three_cups_tail[-1]] = insert_next


def crab_move(cups_dict, current_cup):
    # get three cups tail
    three_cups_tail = get_three_cups_tail(current_cup, cups_dict)
    # get destination cup
    destination_cup = get_destination_cup(current_cup, three_cups_tail)
    move_three_cups_tail(cups_dict, destination_cup, three_cups_tail)
    current_cup = cups_dict[current_cup]
    return current_cup


def print_dict_vals(cups_dict):
    for cup in [6, 8, 5, 9, 7, 4, 2, 1, 3, 10, 10000000]:
        print(cups_dict.get(cup))
    print(len(cups_dict))


def solution():
    cups_dict, first_cup = parse_cups()
    print_dict_vals(cups_dict)
    current_cup = crab_move(cups_dict, first_cup)
    for i in range(10000000):
        current_cup = crab_move(cups_dict, current_cup)
    next_to_1 = cups_dict[1]
    next_to_next_to_1 = cups_dict[next_to_1]
    print(f"all I 1't is {next_to_1} and {next_to_next_to_1} multiplied, which 1s {next_to_1*next_to_next_to_1}")
    #idx_1 = cups_dict[1]
    #cups_after_1 = ""
    #for i in range(idx_1 + 1, idx_1 + len(cups)):
    #    cups_after_1 += str(cups[i%len(cups)])
    #print(cups)
    #print(cups_after_1)


solution()