from pathlib import Path
import os
import re
import math
import itertools
import more_itertools
import copy
import sys
from functools import reduce

def read_input_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return [line for line in f.read().splitlines()]


def get_departure_time(input_file):
    return int(input_file[0])


def get_bus_schedule(input_file):
    return sorted([int(bus_id) for bus_id in input_file[1].split(",") if bus_id is not 'x'])


def get_bus_schedule_with_index(input_file):
    bus_schedule_raw = input_file[1].split(",")
    return [[i, int(bus_schedule_raw[i])] for i in range(0, len(bus_schedule_raw)) if bus_schedule_raw[i] is not 'x']


def get_next_bus_departure_id(departure_time, bus_schedule_entry):
    i = 0
    while i < departure_time:
        i += bus_schedule_entry
    return i


def determine_earliest_bus(departure_time, bus_schedule):
    #departure_time_minutes = departure_time % 60
    earliest_departure_time_bus = sys.maxsize
    bus_id_selected = -1
    for bus_id in bus_schedule:
        departure_time_bus = get_next_bus_departure_id(departure_time, bus_id)
        if departure_time_bus < earliest_departure_time_bus:
            bus_id_selected = bus_id
            earliest_departure_time_bus = departure_time_bus
    time_diff = earliest_departure_time_bus - departure_time
    answer = time_diff * bus_id_selected
    print(f"time-diff = {time_diff}, where bus-ID = {bus_id_selected} and departure time = {departure_time} with earliest departure time = {earliest_departure_time_bus}")
    print(answer)


def chinese_remainder(n, a):
    sum = 0
    prod = reduce(lambda a, b: a*b, n)
    for n_i, a_i in zip(n, a):
        p = prod // n_i
        sum += a_i * mul_inv(p, n_i) * p
    return sum % prod


def mul_inv(a, b):
    b0 = b
    x0, x1 = 0, 1
    if b == 1: return 1
    while a > 1:
        q = a // b
        a, b = b, a%b
        x0, x1 = x1 - q * x0, x0
    if x1 < 0: x1 += b0
    return x1


def check_schedule(t, offset,  bus_departure_time):
    return (t % bus_departure_time) == ((bus_departure_time - offset) % bus_departure_time)


def print_schedule(t, offset,  bus_departure_time):
    res = (t + offset) % bus_departure_time == 0
    print(f"(t + offset) % bus_departure_time = ({t} + {offset}) % {bus_departure_time} = {res}")


def win_bus_contest(bus_schedule):
    first_entry = bus_schedule[0]
    #max_entry = max(bus_schedule, key=lambda x: x[1])
    t = first_entry[0]
    #t_step = max_entry[1]
    eureka = False
    while (not eureka):
        experiment = True
        for offset, bus_departure_time in bus_schedule:
            experiment &= check_schedule(t, offset, bus_departure_time)
        if experiment:
            print(t)
            for offset, bus_departure_time in bus_schedule:
                print_schedule(t, offset, bus_departure_time)
            return t
        t += 1


def win_bus_contest_efficiently(bus_schedule):
    n = [bus_id for i, bus_id in bus_schedule]
    a = [(bus_id - i) % bus_id for i, bus_id in bus_schedule]
    print(chinese_remainder(n,a))


def solution_a():
    input_file = read_input_file()
    departure_time = get_departure_time(input_file)
    bus_schedule = get_bus_schedule(input_file)
    determine_earliest_bus(departure_time, bus_schedule)


def solution_b():
    input_file = read_input_file()
    departure_time = get_departure_time(input_file)
    bus_schedule = get_bus_schedule_with_index(input_file)
    win_bus_contest_efficiently(bus_schedule)
    #win_bus_contest(bus_schedule)

solution_b()