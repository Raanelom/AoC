from pathlib import Path
import os
import re


REQUIRED_KEYS = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'}
OPTIONAL_KEYS = {'cid'}
HEIGHT_PATTERN = r"^([0-9]+)([a-z]+)$"
HCL_PATTERN = r"^(#[0-9a-f]{6})$"
PID_PATTERN = r"^([0-9]{9})$"

def split_height(string_to_split):
    return re.match(HEIGHT_PATTERN, string_to_split)


def split_hcl(string_to_split):
    return re.match(HCL_PATTERN, string_to_split)

def check_pid(string_to_check):
    return re.match(PID_PATTERN, string_to_check)


def split_key_val(prop):
    return prop.split(':')


def passports_raw_to_dict(passports_raw):
    passports = []
    for passport_raw in passports_raw:
        passport = {}
        for sublist in passport_raw:
            for prop in sublist:
                k, v = split_key_val(prop)
                passport[k] = v
        passports.append(passport)
    return passports


def read_file():
    base_path = os.path.dirname(os.path.abspath(__file__))
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        return [[prop.split(' ') for prop in passport_obj.splitlines()] for passport_obj in f.read().split('\n\n')]

    
def check_password_validity(passport):
    is_valid = True
    is_valid &= int(passport['byr']) in range(1920, 2003)
    is_valid &= int(passport['iyr']) in range(2010, 2021)
    is_valid &= int(passport['eyr']) in range(2020, 2031)
    hgt = split_height(passport['hgt'])
    if not hgt:
        return False
    hgt_split = hgt.groups()
    hgt_val = int(hgt_split[0])
    hgt_metric = hgt_split[1]
    is_valid &= ((hgt_val in range(150, 194) if hgt_metric == 'cm' else False) or (hgt_val in range(59, 77) if hgt_metric == 'in' else False))
    hcl = split_hcl(passport['hcl'])
    if not hcl:
        return False
    is_valid &= passport['ecl'] in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
    if not check_pid(passport['pid']):
        return False
    
    return is_valid
    


def check_password_presence(passports):
    count_valid = 0
    for passport in passports:
        if all(key in passport for key in REQUIRED_KEYS):
            count_valid += check_password_validity(passport)
    return count_valid



passports_raw = read_file()
passports = passports_raw_to_dict(passports_raw)
valid_pp = check_password_presence(passports)
print(valid_pp)