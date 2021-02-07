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


def read_input_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return [line for line in f.read().splitlines()]


def foods_to_dict(foods):
    ingredients_dict = {}
    allergens_dict = {}
    for food in foods:
        ingredients_raw, allergens_raw = food[0:-1].split(" (contains ")
        ingredients = ingredients_raw.split(" ")
        allergens = allergens_raw.split(", ")
        for ingredient in ingredients:
            ingredients_list = ingredients_dict.get(ingredient, [])
            ingredients_list.append(allergens.copy())
            ingredients_dict[ingredient] = ingredients_list
        for allergen in allergens:
            allergens_list = allergens_dict.get(allergen, [])
            allergens_list.append(ingredients.copy())
            allergens_dict[allergen] = allergens_list
    return ingredients_dict, allergens_dict


def matchmaking(ingredients_dict, allergens_dict):
    max_allergens = len(allergens_dict)
    known_allergens = {}
    remaining_ingredients = list(ingredients_dict.keys())
    allergens_dict_reduced = {}
    for allergen, ingredients in allergens_dict.items():
        allergens_dict_reduced[allergen] = list(set.intersection(*map(set, ingredients)))
    known_allergen_list = [allergen for allergen, ingredients in allergens_dict_reduced.items() if len(ingredients) == 1]
    while len(known_allergen_list) > 0:
        for known_allergen in known_allergen_list:
            known_ingredient = allergens_dict_reduced[known_allergen][0]
            remaining_ingredients.remove(known_ingredient)
            known_allergens[known_allergen] = known_ingredient
            del allergens_dict_reduced[known_allergen]
            for ingredients in allergens_dict_reduced.values():
                if known_ingredient in ingredients:
                    ingredients.remove(known_ingredient)
        known_allergen_list = [allergen for allergen, ingredients in allergens_dict_reduced.items() if len(ingredients) == 1]

    if len(known_allergens) == max_allergens:
        print(sum([len(ingredients_dict[ingredient]) for ingredient in remaining_ingredients]))
        known_allergens_ordered = collections.OrderedDict(sorted(known_allergens.items()))
        print(",".join(known_allergens_ordered.values()))
    else:
        print("something went wrong")
        return False


def solution():
    foods = read_input_file()
    ingredients_dict, allergens_dict = foods_to_dict(foods)
    allergen_free_min = len(ingredients_dict) - len(allergens_dict)
    print(f"There are {len(ingredients_dict)} ingredients and {len(allergens_dict)} allergens, thus at least {allergen_free_min} allergen free")
    matchmaking(ingredients_dict, allergens_dict)

solution()