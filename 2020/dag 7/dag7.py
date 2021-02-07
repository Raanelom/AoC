from pathlib import Path
import os
import re
import math
import itertools
import more_itertools


def read_input_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return f.read().splitlines()


def map_empty(nr):
    if nr == 'no':
        return '0'
    return nr


def clean_bag(bag):
    return bag.rsplit(" ", 1)[0].strip(' ')


def parse_line_of_bags(line_of_bags):
    parent, children = line_of_bags.split(" contain ")
    children = children.strip(".").split(", ")
    children_counted = [child.split(" ", 1) for child in children]
    children_counted_dict = [{ clean_bag(child_counted[1]): int(map_empty(child_counted[0])) } 
        for child_counted in children_counted]
    return [clean_bag(parent), children_counted_dict]


def process_input_file(input_file_splitted):
    bag_overview = {}
    for line_of_bags in input_file_splitted:
        parent, children_counted = parse_line_of_bags(line_of_bags)
        bag_overview[parent] = children_counted
    return bag_overview


def traverse_up_and_count(luggage_belt, begging_bag, colors):
    for parent, children in luggage_belt.items():
        for bag in children:
            for bag_k, bag_v in bag.items():
                if begging_bag == bag_k:
                    colors.append(parent)
                    #print(f"{parent}: {begging_bag}:{bag.keys()}")
                    traverse_up_and_count(luggage_belt, parent, colors)
    return set(colors)


def traverse_down_and_count(luggage_belt, begging_bag):
    count = 1
    for children in luggage_belt[begging_bag]:
        for k, v in children.items():
            if k != 'other':
                count += (v * traverse_down_and_count(luggage_belt, k))
    return count


input_file_splitted = read_input_file()
bags_dict = process_input_file(input_file_splitted)
bag_count = traverse_up_and_count(bags_dict, "shiny gold", [])
#print(len(bag_count))

res1 = traverse_down_and_count(bags_dict, "shiny gold")
print(res1 - 1)