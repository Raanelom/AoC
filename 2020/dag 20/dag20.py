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
from pprint import pprint


ORIENTATION_MATRIX = {
    0: [],
    1: ["flip-horizontal"],
    2: ["flip-vertical"],
    3: ["rotate-270"],
    4: ["rotate-90"],
    5: ["rotate-180"],
    6: ["rotate-90", "flip-horizontal"],
    7: ["rotate-180", "flip-horizontal"],
    8: ["rotate-270", "flip-horizontal"],
    9: ["rotate-90", "flip-vertical"],
    10: ["rotate-180", "flip-vertical"],
    11: ["rotate-270", "flip-vertical"],
    12: ["flip-horizontal", "flip-vertical"],
    13: ["rotate-90", "flip-horizontal", "flip-vertical"],
    14: ["rotate-180", "flip-horizontal", "flip-vertical"],
    15: ["rotate-270", "flip-horizontal", "flip-vertical"]
}
ORIENTATION_MULTIPLIER = 13


def read_input_tiles():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return [[line for line in group.splitlines()] for group in f.read().split("\n\n")]


def read_sea_monster():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('sea_monster.txt'), 'r') as f:
        return [line for line in f.read().splitlines()]


def tiles_to_dict(tiles):
    outer_tiles_dict = {}
    inner_tiles_dict = {}
    for tile in tiles:
        key = int(tile[0].split(" ")[1].split(":")[0])
        borders = []
        # only get the edges
        borders.append(tile[1]) # top
        borders.append("".join([line[-1] for line in tile[1:]])) # right
        borders.append(tile[-1]) # bottom
        borders.append("".join([line[0] for line in tile[1:]])) # left
        outer_tiles_dict[key] = borders
        # only get the inside
        inner_tiles_dict[key] = []
        for inner_tile in tile[2:-1]:
            inner_tiles_dict[key].append(inner_tile[1:-1])
    return outer_tiles_dict, inner_tiles_dict


def flip_horizontal(tile):
    return [tile[0][::-1], tile[3], tile[2][::-1], tile[1]]


def flip_vertical(tile):
    return [tile[2], tile[1][::-1], tile[0], tile[3][::-1]]


def shift(tile, n=0):
    a = n % len(tile)
    return tile[-a:] + tile[:-a]


def flip_borders(tile, border_indices):
    tile_new = tile.copy()
    for idx in border_indices:
        tile_new[idx] = tile_new[idx][::-1]
    return tile_new


def perform_operations(tile, orientation):
    tile_new = tile.copy()
    for operation in ORIENTATION_MATRIX[orientation]:
        if operation == "flip-horizontal":
            tile_new = flip_horizontal(tile_new)
        elif operation == "flip-vertical":
            tile_new = flip_vertical(tile_new)
        elif operation == "rotate-90":
            tile_new = shift(tile_new, 1)
            tile_new = flip_borders(tile_new, [0, 2])
        elif operation == "rotate-180":
            tile_new = shift(tile_new, 2)
            tile_new = flip_borders(tile_new, [0, 1, 2, 3])
        elif operation == "rotate-270":
            tile_new = shift(tile_new, 3)
            tile_new = flip_borders(tile_new, [1, 3])
    return tile_new


def get_current_tile(attempt, all_tiles):
    current_tile_idx = math.floor(attempt / ORIENTATION_MULTIPLIER)
    current_tile = all_tiles[current_tile_idx]
    current_orientation = attempt % ORIENTATION_MULTIPLIER
    return current_tile, current_orientation


def match_position(tile_idx, tiles_dict, current_tile, current_orientation, image, image_orientation, dim, tile_rotation_cache):
    # Case 1: first position
    if tile_idx == 0:
        return True
    # get current rotation from cache (based on key/orientation)
    tile_rotation_cache[(current_tile, current_orientation)] = tile_rotation_cache.get((current_tile, current_orientation), 
        perform_operations(tiles_dict.get(current_tile), current_orientation))
    tile_candidate = tile_rotation_cache[(current_tile, current_orientation)]
    # Left tile
    left_idx = image[tile_idx-1]
    left_orientation = image_orientation.get(left_idx)
    tile_rotation_cache[(left_idx, left_orientation)] = tile_rotation_cache.get((left_idx, left_orientation), 
        perform_operations(tiles_dict.get(left_idx), left_orientation))
    tile_to_match_left = tile_rotation_cache[(left_idx, left_orientation)]
    # Case 2: first row -> only match right/left
    if tile_idx < dim:
        return tile_to_match_left[1] == tile_candidate[3]
    # Top tile
    top_idx = image[tile_idx-dim]
    top_orientation = image_orientation.get(top_idx)
    tile_rotation_cache[(top_idx, top_orientation)] = tile_rotation_cache.get((top_idx, top_orientation), 
        perform_operations(tiles_dict.get(top_idx), top_orientation))
    tile_to_match_top = tile_rotation_cache[(top_idx, top_orientation)]
    # Case 3: start on new row -> only match top/bottom
    if tile_idx % dim == 0:
        return tile_to_match_top[2] == tile_candidate[0]
    # Case 4: all other cases -> match right/left and top/bottom
    return tile_to_match_left[1] == tile_candidate[3] and tile_to_match_top[2] == tile_candidate[0]


def step_back(image, image_orientation, tile_idx, key):
    image.remove(key)
    del image_orientation[key]
    return tile_idx - 1


def step_forward(image, image_orientation, tile_idx, key, orientation):
    image.append(key)
    image_orientation[key] = orientation
    return tile_idx + 1


def selfie_doen(tiles_dict, dim):
    image = []
    tile_cache = {}
    image_orientation = {}
    tile_rotation_cache = {}
    all_tiles = list(tiles_dict.keys())
    attempts = {}
    tile_idx = 0
    step = 0
    while len(image) < len(tiles_dict):
        step += 1
        if tile_idx > 0:
            if tile_idx < dim and tile_cache.get((tile_idx, image_orientation[image[tile_idx - 1]], image[tile_idx - 1]), False):
                tile_idx = step_back(image, image_orientation, tile_idx, image[-1])
            elif tile_idx % dim == 0 and tile_cache.get((tile_idx, image_orientation[image[tile_idx - dim]], image[tile_idx - dim]), False):
                tile_idx = step_back(image, image_orientation, tile_idx, image[-1])
            elif tile_idx > dim and tile_cache.get((tile_idx, image_orientation[image[tile_idx - 1]], image_orientation[image[tile_idx - dim]], 
                image[tile_idx - 1], image[tile_idx - dim]), False):
                tile_idx = step_back(image, image_orientation, tile_idx, image[-1])
        if attempts.get(tile_idx, -1) == len(tiles_dict) * ORIENTATION_MULTIPLIER - 1:
            # exhausted, thus go back one (or) more tile indices
            # store in cache
            if tile_idx < dim:
                left_img = image[tile_idx - 1]
                tile_cache[(tile_idx, image_orientation[left_img], left_img)] = True
            elif tile_idx % dim == 0:
                top_img = image[tile_idx - dim]
                tile_cache[(tile_idx, image_orientation[top_img], top_img)] = True
            else:
                left_img = image[tile_idx - 1]
                top_img = image[tile_idx - dim]
                tile_cache[(tile_idx, image_orientation[left_img], image_orientation[top_img], left_img, top_img)] = True
            attempts[tile_idx] = 0
            tile_idx = step_back(image, image_orientation, tile_idx, image[-1])
            continue
        attempts[tile_idx] = attempts.get(tile_idx, -1) + 1
        # Try to match all possible rotations
        current_tile, current_orientation = get_current_tile(attempts[tile_idx], all_tiles)
        if current_tile in image:
            continue
        is_match = match_position(tile_idx, tiles_dict, current_tile, current_orientation, image, image_orientation, dim, tile_rotation_cache)
        if is_match:
            tile_idx = step_forward(image, image_orientation, tile_idx, current_tile, current_orientation)
    return image, image_orientation


def flip_horizontal_full(tile):
    return [line[::-1] for line in tile]


def flip_vertical_full(tile):
    return tile[::-1]


def rotate_full(tile, n=0):
    if n == 0:
        return tile
    new_tile = []
    for j in range(len(tile[0])): # loop over columns
        new_tile.append("")
        for i in range(len(tile)): # loop over rows
            new_tile[j] = tile[i][j] + new_tile[j]
    return rotate_full(new_tile, n-1)


def perform_operations_full(tile, orientation):
    tile_new = tile.copy()
    for operation in ORIENTATION_MATRIX[orientation]:
        if operation == "flip-horizontal":
            tile_new = flip_horizontal_full(tile_new)
        elif operation == "flip-vertical":
            tile_new = flip_vertical_full(tile_new)
        elif operation == "rotate-90":
            tile_new = rotate_full(tile_new, 1)
        elif operation == "rotate-180":
            tile_new = rotate_full(tile_new, 2)
        elif operation == "rotate-270":
            tile_new = rotate_full(tile_new, 3)
    return tile_new



def compose_image(resulting_image, image_orientation, inner_tiles_dict, dim):
    gappie_image = []
    for key in resulting_image:
        new_tile = perform_operations_full(inner_tiles_dict[key], image_orientation[key])
        # perform rotation...
        gappie_image.append(new_tile)
    gapless_image = []
    for i in range(dim*8): # loop over tile rows
        gapless_image.append("")
        tile_row = math.floor(i / 8)
        for tile in gappie_image[(tile_row*dim):((tile_row+1)*dim)]: # get the relevant tiles
            gapless_image[i] += tile[i%8]
    return gapless_image


def is_a_in_b(line_a, line_b):
    for i in range(len(line_a)):
        if line_a[i] == "#" and line_b[i] != "#":
            return False
    return True


def match_sea_monster(gapless_image, sea_monster):
    sea_monster_match = 0
    sea_monster_height = len(sea_monster)
    sea_monster_length = len(sea_monster[1])
    for i in range(len(gapless_image) - sea_monster_height + 1):
        for j in range(len(gapless_image) - sea_monster_length + 1):
            sea_monster_match += (is_a_in_b(sea_monster[0], gapless_image[i][j:(j+sea_monster_length)]) and
                is_a_in_b(sea_monster[1], gapless_image[i+1][j:(j+sea_monster_length)]) and
                is_a_in_b(sea_monster[2], gapless_image[i+2][j:(j+sea_monster_length)]))
    return sea_monster_match


def get_sea_monster_area(sea_monster, sea_monster_count):
    sea_monster_area = [line.count("#") for line in sea_monster]
    return sum(sea_monster_area) * sea_monster_count


def get_water_roughness(gapless_image, sea_monster_area):
    water_roughness_raw = [line.count("#") for line in gapless_image]
    return sum(water_roughness_raw) - sea_monster_area


def find_sea_monster(gapless_image):
    sea_monster = read_sea_monster()
    sea_monster_count = 0
    best_orientation = 0
    best_image = []
    for orientation in ORIENTATION_MATRIX.keys():
        gapless_image_toggled = perform_operations_full(gapless_image.copy(), orientation)
        sea_monsters_found = match_sea_monster(gapless_image_toggled, sea_monster)
        if sea_monsters_found > sea_monster_count:
            sea_monster_count = sea_monsters_found
            best_orientation = orientation
            best_image = gapless_image_toggled
    pprint(best_image)
    sea_monster_area = get_sea_monster_area(sea_monster, sea_monster_count)
    water_roughness = get_water_roughness(best_image, sea_monster_area)
    print(water_roughness)
    print(f"sea monsters: {sea_monster_count}")
    print(f"best orientation: {best_orientation}")


def solution():
    tiles = read_input_tiles()
    dim = int(math.sqrt(len(tiles)))
    outer_tiles_dict, inner_tiles_dict = tiles_to_dict(tiles)
    resulting_image, image_orientation = selfie_doen(outer_tiles_dict, dim)
    corners = [resulting_image[0], resulting_image[dim-1], resulting_image[len(resulting_image) - dim], resulting_image[-1]]
    res = 1
    for corner in corners:
        res *= corner
    print(res)
    gapless_image = compose_image(resulting_image, image_orientation, inner_tiles_dict, dim)
    find_sea_monster(gapless_image)


solution()