from pathlib import Path
import os
import re
import math
import itertools
import more_itertools
import copy
import sys

def read_input_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return sorted([int(line) for line in f.read().splitlines()])


def calc_joltage_differences(joltages_list):
    return [joltages_list[i] - joltages_list[i - 1] for i in range(1, len(joltages_list))]


def count_joltages(joltages_diff_list, specific_joltage):
    return joltages_diff_list.count(specific_joltage)


def diff_clusters(joltages_diff_list):
    clusters = []
    count = 0
    for i in range(len(joltages_diff_list)):
        if joltages_diff_list[i] == 1:
            count += 1
        elif count != 0:
            count += 1  # append 1 extra, because the first element is not included in the diff list
            clusters.append(count)
            count = 0
    if count != 0:
        count += 1  # append 1 extra, because the first element is not included in the diff list
        clusters.append(count)
    return clusters


def calculate_cluster_impact(cluster_size):
    impact = 2
    for i in range(2, cluster_size - 1):
        impact += i
    return impact


def calculate_arrangements(diff_clusters):
    arrangements = 1
    for cluster in diff_clusters:
        if cluster > 2:
            cluster_impact = calculate_cluster_impact(cluster)
            arrangements *= cluster_impact
    return arrangements

joltages_list = read_input_file()
device = joltages_list[-1] + 3 # Add 3 to last value in list
charging_outlet = 0 # use 0 as value for charging outlet

joltages_list.insert(0, charging_outlet)
joltages_list.append(device)
joltages_diff_list = calc_joltage_differences(joltages_list)

print(joltages_list)
#joltage_count_1 = count_joltages(joltages_diff_list, 1)
#joltage_count_3 = count_joltages(joltages_diff_list, 3)
#print(f"joltage count 1 = {joltage_count_1}")
#print(f"joltage count 3 = {joltage_count_3}")
#print(f"sum is {joltage_count_1 * joltage_count_3}")

clusters = diff_clusters(joltages_diff_list)
print(clusters)

arrangements = calculate_arrangements(clusters)
print(arrangements)
# 1 4 5 6 9
# 1 4 6 9
# = 2

# 1 4 5 6 7 10
# 1 4 5 7 10
# 1 4 6 7 10
# 1 4 7 10
# = 4

# 1 4 5 6 7 8 11
# 1 4 5 6 8 11
# 1 4 5 7 8 11
# 1 4 6 7 8 11
# 1 4 5 8 11
# 1 4 6 8 11
# 1 4 7 8 11
# = 7

# 1 4 5 6 7 8 9 12
# 1 4 5 6 7 9 12
# 1 4 5 6 8 9 12
# 1 4 5 7 8 9 12
# 1 4 6 7 8 9 12
# 1 4 5 6 9 12
# 1 4 5 7 9 12
# 1 4 6 7 9 12
# 1 4 6 8 9 12
# 1 4 6 9 12
# 1 4 7 9 12
# = 11
