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
import regex


def read_input_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input_b.txt'), 'r') as f:
        return [[line for line in group.splitlines()] for group in f.read().split("\n\n")]


def rules_to_dict(rules):
    rules_dict = {}
    for rule in rules:
        key, val = rule.split(":")
        if "\"" in val:
            rules_dict[int(key)] = val.strip()[1]
        else:
            rules_dict[int(key)] = [[int(n) for n in v.strip().split(" ")] for v in val.split("|")]
    return rules_dict


def determine_rule(rule, rules_dict):
    next_rules_group = rules_dict.get(rule)
    if isinstance(next_rules_group, str):
        return next_rules_group
    rules = []
    group_name = f"group_{rule}"
    for next_rules in next_rules_group:
        pattern = ""
        for next_rule in next_rules:
            if next_rule == rule:
                pattern += f"(?&{group_name})"
            else:
                pattern += determine_rule(next_rule, rules_dict)
        rules.append(pattern)
    regex = "|".join(rules)
    return f"(?P<{group_name}>{regex})"


def check_rules(rules_regex, messages):
    no_of_matches = 0
    for message in messages:
        no_of_matches += 1 if regex.match(rules_regex, message) else 0
    print(no_of_matches)



def solution():
    rules, messages = read_input_file()
    rules_dict = rules_to_dict(rules)
    rules_regex = f"^{determine_rule(0, rules_dict)}$"
    check_rules(rules_regex, messages)
    

solution()