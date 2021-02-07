from pathlib import Path
import os
import re
import math

def read_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return f.read().splitlines()


def split_word(word): 
    return [char for char in word]  


def split_rows(bsp_indicator, row_min, row_max):
    diff = (row_max - row_min) / 2
    return [row_min, math.floor(row_max - diff)] if bsp_indicator == "F" else [math.ceil(row_min + diff), row_max]


def split_cols(bsp_indicator, col_min, col_max):
    diff = (col_max - col_min) / 2
    return [col_min, math.floor(col_max - diff)] if bsp_indicator == "L" else [math.ceil(col_min + diff), col_max]


def get_row(rows):
    row_first = 0
    row_last = 127
    for row in rows:
        [row_first, row_last] = split_rows(row, row_first, row_last)
    return row_first if row_first == row_last else -1


def get_col(cols):
    col_first = 0
    col_last = 7
    for col in cols:
        [col_first, col_last] = split_cols(col, col_first, col_last)
    return col_first if col_first == col_last else -1


def calc_seat_id(row, col):
    return (int(row)*8) + int(col)


def determine_missing_seat_id(sorted_input_list):
    prev_seat_id = int(sorted_input_list[0])
    for seat_id in sorted_input_list[1:len(sorted_input_list)]:
        seat_id_int = int(seat_id)
        if seat_id_int != (prev_seat_id + 1):
            print(seat_id_int - 1)
        prev_seat_id = seat_id_int


def parse_bsp_lines(input_list):
    seat_id_list = []
    for line in input_list:
        rows = line[0:7]
        cols = line[7:10]
        row = get_row(rows)
        col = get_col(cols)
        seat_id = calc_seat_id(row, col)
        seat_id_list.append(seat_id)
    return sorted(seat_id_list)


input_list = read_file()
sorted_input = parse_bsp_lines(input_list)
determine_missing_seat_id(sorted_input)
