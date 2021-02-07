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


sys.setrecursionlimit(10000)


def read_input_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return [[line for line in group.splitlines()] for group in f.read().split("\n\n")]


def parse_decks(player_decks):
    return [[int(card) for card in deck[1:]] for deck in player_decks]


def decks_key(deck_1, deck_2):
    return "|".join([",".join(str(card) for card in deck_1), ",".join(str(card) for card in deck_2)])


def play_combat(player_1_deck, player_2_deck, deck_memory, game, rounds):
    if len(player_2_deck) == 0 or deck_memory.get(decks_key(player_1_deck, player_2_deck), False):
        return "p1", player_1_deck
    if len(player_1_deck) == 0:
        return "p2", player_2_deck
    deck_memory[decks_key(player_1_deck, player_2_deck)] = True
    p1_top_card = player_1_deck.pop(0)
    p2_top_card = player_2_deck.pop(0)
    if p1_top_card <= len(player_1_deck) and p2_top_card <= len(player_2_deck):
        deck_memory_subgame = {}
        player_1_subdeck = player_1_deck.copy()[0:p1_top_card]
        player_2_subdeck = player_2_deck.copy()[0:p2_top_card]
        winner, winner_deck = play_combat(player_1_subdeck, player_2_subdeck, deck_memory_subgame, game + 1, 1)
        if winner == "p1":
            player_1_deck += [p1_top_card, p2_top_card]
        elif winner == "p2":
            player_2_deck += [p2_top_card, p1_top_card]
        else:
            print("something went wrong")
    elif p1_top_card > p2_top_card:
        player_1_deck += [p1_top_card, p2_top_card]
    elif p2_top_card > p1_top_card:
        player_2_deck += [p2_top_card, p1_top_card]
    else:
        print("they're equal...")
    return play_combat(player_1_deck, player_2_deck, deck_memory, game, rounds+1)


def calculate_score(winner_deck):
    score = 0
    for i in range(1, len(winner_deck) + 1):
        score += i*winner_deck[i-1]
    return score


def solution():
    player_decks = read_input_file()
    player_1_deck, player_2_deck = parse_decks(player_decks)
    deck_memory = {}
    winner, winner_deck = play_combat(player_1_deck, player_2_deck, deck_memory, 1, 1)
    print(winner)
    print(winner_deck)
    print(calculate_score(winner_deck[::-1]))

solution()