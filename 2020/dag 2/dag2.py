from pathlib import Path
import os

def count_chars_in_password(policy_min, policy_max, policy_char, password):
    print(policy_min)
    print(policy_max)
    print(policy_char)
    print(password)
    min_sanitized = int(policy_min)
    max_sanitized = int(policy_max)
    policy_range = range(min_sanitized, max_sanitized+1)
    occurence_count = password.count(policy_char)
    return occurence_count in policy_range


def count_position_in_password(policy_idx_min, policy_idx_max, policy_char, password):
    idx_min = int(policy_idx_min)
    idx_max = int(policy_idx_max)
    return (password[idx_min - 1] == policy_char) ^ (password[idx_max - 1] == policy_char)


def determine_password_validity():
    base_path = os.path.dirname(os.path.abspath(__file__))
    valid_passwords = 0
    with open(Path(base_path) / Path('input.txt'), 'r') as f:
        input_lines = f.read().splitlines()
        for line in input_lines:
            entry = line.split(' ')
            if len(entry) == 3:
                policy_amount = entry[0].split('-')
                policy_char = entry[1].strip(':')
                password = entry[2]
                #is_valid = count_chars_in_password(policy_amount[0], policy_amount[1], policy_char, password)
                is_valid = count_position_in_password(policy_amount[0], policy_amount[1], policy_char, password)
                valid_passwords += is_valid
                print(f"{line} {is_valid}")
    return valid_passwords



print(determine_password_validity())