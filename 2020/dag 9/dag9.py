from pathlib import Path
import os
import re
import math
import itertools
import more_itertools
import copy
import sys

sys.setrecursionlimit(10**6)

def read_input_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return [int(line) for line in f.read().splitlines()]


def verify_sum(idx_start, idx_end, cipher_list, number_to_validate):
    for i in range(idx_start, idx_end):
        for j in range(idx_start, idx_end):
            if cipher_list[i] == cipher_list[j] or i == j:
                continue
            if cipher_list[i] + cipher_list[j] == number_to_validate:
                return [i, j]
    return [-1, -1]



def verify_input(preamble_start, preamble_length, cipher_list):
    idx_current = preamble_start
    sliding_window = preamble_length
    for cipher in cipher_list[preamble_length:len(cipher_list)]:
        i, j = verify_sum(idx_current, idx_current + sliding_window, cipher_list, cipher)
        if i == -1:
            return [idx_current + sliding_window + 1, cipher]
        idx_current += 1
    return [-1, -1]


def find_contiguous_set(idx_start, idx_end, cipher_list, broken_cipher):
    for i in range(idx_start, idx_end):
        idx_current = i
        interim_sum = 0
        subset_idx = []
        subset = []
        while interim_sum < broken_cipher:
            interim_sum += cipher_list[idx_current]
            subset_idx.append(idx_current)
            subset.append(cipher_list[idx_current])
            if interim_sum == broken_cipher:
                return [subset_idx, sorted(subset)]
            idx_current += 1
    return [None, None]

  

cipher_list = read_input_file()
idx, broken_cipher = verify_input(0, 25, cipher_list)
print(idx, broken_cipher)
subset_idx, subset = find_contiguous_set(0, idx, cipher_list, broken_cipher)
print(subset_idx)
print(subset)
print(sum(subset))
print(subset[0] + subset[-1])