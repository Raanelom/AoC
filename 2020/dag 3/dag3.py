from pathlib import Path
import os

def read_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return f.read().splitlines()

def traverse(right, down, start_col, start_row):
    input_matrix = read_file()
    output_symbols = []
    col_idx = start_col
    for i in range(start_row, len(input_matrix), down):
        line = input_matrix[i]
        col_idx = (col_idx + right) % len(line)
        print(f"{i} {col_idx}")
        output_symbols.append(line[col_idx])
    return output_symbols

def count_trees(input_symbols):
    return input_symbols.count('#')


no1 = count_trees(traverse(1, 1, 0, 1))
no2 = count_trees(traverse(3, 1, 0, 1))
no3 = count_trees(traverse(5, 1, 0, 1))
no4 = count_trees(traverse(7, 1, 0, 1))
no5 = count_trees(traverse(1, 2, 0, 2))

print(f"{no1} {no2} {no3} {no4} {no5}")

print(f"{no1*no2*no3*no4*no5}")