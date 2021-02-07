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


def read_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return [group.splitlines() for group in f.read().split('\n\n')]


def parse_range(start_stop):
    res = start_stop.split("-")
    if len(res) != 2:
        raise ValueError("err")
    return [int(res[0]), int(res[1])]


def parse_fields(fields_to_parse):
    fields = {}
    for key, val in [field_raw.split(":") for field_raw in fields_to_parse]:
        ranges = val.strip().split(" or ")
        fields[key.strip()] = [parse_range(start_stop) for start_stop in ranges]
    return fields


def parse_single_ticket(ticket_to_parse):
    return [int(ticket) for ticket in ticket_to_parse.split(",")]


def parse_nearby_tickets(tickets_to_parse):
    return [parse_single_ticket(ticket) for ticket in tickets_to_parse]


def get_tickets(nearby_tickets, fields):
    for ticket in nearby_tickets:
        for no in ticket:
            match = False
            for field in fields.values():
                for start, stop in field:
                    match |= start <= no <= stop
            if not match:
                yield no


def filter_valid_tickets(nearby_tickets, fields):
    valid_tickets = []
    for ticket in nearby_tickets:
        valid = True
        for no in ticket:
            match = False
            for field in fields.values():
                for start, stop in field:
                    match |= start <= no <= stop
            valid &= match
        if valid:
            valid_tickets.append(ticket)
    return valid_tickets


def get_vertical_matches(valid_tickets, fields):
    valid_ticket_columns = {}
    for key, val in fields.items():
        valid_ticket_columns[key] = []
        for j in range(len(valid_tickets[0])):
            valid_column = True
            for i in range(len(valid_tickets)):
                match = False
                for start, stop in val:
                    match |= start <= valid_tickets[i][j] <= stop
                valid_column &= match
            if valid_column:
                valid_ticket_columns[key].append(j)
    return valid_ticket_columns
        

def determine_columns(column_matches):
    cols = {}
    keys_sorted = sorted(column_matches, key=lambda k: len(column_matches[k]))
    for k in keys_sorted:
        col_candidates = [col for col in column_matches[k] if col not in cols]
        if len(col_candidates) == 1:
            cols[col_candidates[0]] = k
        else:
            raise ValueError("durr")
    return cols


def determine_departure_positions(ticket, matching_cols):
    ticket_fields = []
    for k, v in matching_cols.items():
        if v.startswith("departure"):
            print(k)
            ticket_fields.append(ticket[k])
    res = 1
    for field in ticket_fields:
        res *= field
    print(res)


def solution_a():
    input_file = read_file()
    fields = parse_fields(input_file[0])
    ticket = parse_single_ticket(input_file[1][1])
    nearby_tickets = parse_nearby_tickets(input_file[2][1:])
    invalid_tickets = [invalid_ticket for invalid_ticket in get_tickets(nearby_tickets, fields)]
    print(sum(invalid_tickets))


def solution_b():
    input_file = read_file()
    fields = parse_fields(input_file[0])
    ticket = parse_single_ticket(input_file[1][1])
    nearby_tickets = parse_nearby_tickets(input_file[2][1:])
    valid_tickets = filter_valid_tickets(nearby_tickets, fields)
    vertical_matches = get_vertical_matches(valid_tickets, fields)
    res = determine_columns(vertical_matches)
    print(ticket)
    print(res)
    determine_departure_positions(ticket, res)


solution_b()