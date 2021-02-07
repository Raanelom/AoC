from pathlib import Path
import os
import re
import math

def read_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return [[[answer for answer in answers] for answers in group.splitlines()] for group in f.read().split('\n\n')]


def duplicate_answers(input_file):
    answers_per_group = []
    for group_answers in input_file:
        everyone_agrees = group_answers[0]
        for answers in group_answers[1:len(group_answers)]:
            everyone_agrees = [answer for answer in answers if answer in everyone_agrees]
        answers_per_group.append(sorted(set(everyone_agrees)))
    return answers_per_group


def unique_answers(input_file):
    answers_per_group = []
    for group_answers in input_file:
        yes_questions = []
        for answers in group_answers:
            yes_questions = yes_questions + answers
        answers_per_group.append(sorted(set(yes_questions)))
    return answers_per_group


def count_answers_per_group(answer_sets):
    return [sum(1 for a in answer_set) for answer_set in answer_sets]

input_file = read_file()
answer_sets_anyone = unique_answers(input_file)
answer_count_per_group = count_answers_per_group(answer_sets_anyone)

answer_sets_everyone = duplicate_answers(input_file)
answer_count_per_group_everyone = count_answers_per_group(answer_sets_everyone)
print(sum(answer_count_per_group_everyone))