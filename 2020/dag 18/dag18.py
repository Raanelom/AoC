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


def evaluate_expression(expression: list):
    no2 = int(expression.pop(2))
    operator = expression.pop(1)
    no1 = int(expression.pop(0))
    res = no1 + no2 if operator == '+' else no1 * no2
    expression.insert(0, str(res))
    if len(expression) == 1:
        return " ".join(expression)
    return evaluate_expression(expression)


def evaluate_addition_before_multiplication(expression: list):
    next_operator_idx = expression.index('+') if '+' in expression else 1
    no2 = int(expression.pop(next_operator_idx + 1))
    operator = expression.pop(next_operator_idx)
    no1 = int(expression.pop(next_operator_idx - 1))
    res = no1 + no2 if operator == '+' else no1 * no2
    expression.insert(next_operator_idx - 1, str(res))
    if len(expression) == 1:
        return " ".join(expression)
    return evaluate_addition_before_multiplication(expression)


def resolve_parentheses(expression):
    first_bracket_idx = expression.find('(') + 1
    last_bracket_idx = -1
    current_bracket_idx = first_bracket_idx
    haakje_open = 1
    while haakje_open != 0:
        next_open = expression.find('(', current_bracket_idx)
        next_closed = expression.find(')', current_bracket_idx)
        haakje_open += 1 if next_open < next_closed and next_open != -1 else -1
        last_bracket_idx = next_closed
        current_bracket_idx = min(next_open, next_closed) + 1 if next_open != -1 else next_closed + 1
    closed_world = expression[first_bracket_idx:last_bracket_idx]
    remainder = expression[expression.find('(', current_bracket_idx):]
    # Nested parentheses (children)
    if '(' in closed_world:
        # Substitute
        sub_expression = resolve_parentheses(closed_world)
        expression = expression.replace(f"({closed_world})", evaluate_addition_before_multiplication(sub_expression.split(" ")))
    else:
        # evaluate expressions already
        res = evaluate_addition_before_multiplication(closed_world.split(" "))
        expression = expression.replace(f"({closed_world})", res)
    # subsequent parentheses (sibblings)
    if '(' in remainder:
        sub_expression = resolve_parentheses(remainder)
        expression = expression.replace(remainder, sub_expression)
    return expression


def solve_sum(expression):
    expression_clean = resolve_parentheses(expression) if '(' in expression else expression
    expression_elements = expression_clean.split(" ")
    return evaluate_addition_before_multiplication(expression_elements)


def solution_a():
    homework = read_input_file()
    res = [int(solve_sum(expression)) for expression in homework]
    print(sum(res))


def solution_b():
    homework = read_input_file()
    res = [int(solve_sum(expression)) for expression in homework]
    print(sum(res))
    

solution_b()