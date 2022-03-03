from collections import defaultdict
import json
import os
import re


curr = os.path.dirname(os.path.abspath(__file__))

def parse_range(number):
    number = list(map(int, number.split('-')))
    assert len(number) in (1, 2)
    if len(number) == 2:
        start, end = number
    else:
        start = end = number[0]
    return start, end

def coalesce_into_range_list(range_list, start, end):
    # is simple, in that it assumes ranges are added in ascending order, so it only check the last entry.

    # If we have existing entries, we check to see if they can be extended to include this range.
    if range_list:
        last_entry = range_list[-1]
        if last_entry[-1] + 1 == start:
            range_list[-1] = (last_entry[0], end)
            return
    # otherwise just add it in
    range_list.append((start, end))

with open(curr+'/ajackson-strat.txt') as f:
    current_gate = None
    capture = False
    monster_to_loc = defaultdict(lambda: defaultdict(list))

    for l in f:
        l = l.strip()
        if l.startswith('Beginner'):
            capture = True
        if l.startswith('N P C  M O N S T E R  M A S T E R S'):
            break

        if capture and l:
            if current_gate and '?' in current_gate:
                continue
            elif '(' in l:
                current_gate = l.split('(')[0].strip()
            else:
                number, monsters = l.split(':')
                if '?' in number:
                    continue
                number = number.strip()
                start, end = parse_range(number)

                monsters = monsters.split()
                for m in monsters:
                    if m[0] == '*':
                        m = m[1:]
                    # TreeSlime needs this, non-contiguous
                    coalesce_into_range_list(monster_to_loc[m][current_gate], start, end)


    capture = False
    current_levels = None
    level_rx = re.compile(r'Team Levels (\d+-\d+)')
    for l in f:
        l = l.strip()
        if level_rx.match(l):
            capture = True
        if l.strip().startswith('M O N S T E R  D A T A'):
            break
        if l and capture:
            m = level_rx.match(l)
            if m:
                current_levels = parse_range(m.group(1))
            else:
                monsters = l.split()
                for m in monsters:
                    # Phoenix needs this, non-contiguous
                    coalesce_into_range_list(monster_to_loc[m]['Foreign Masters'], current_levels[0], current_levels[1])


with open(curr+'/monster-locations.json', 'w') as f:
    json.dump({
        m: {
            name: ', '.join(f'{n[0]}-{n[1]}' if n[0] != n[1] else str(n[0]) for n in numbers)
            for name, numbers in loc.items()
        }
        for m, loc in monster_to_loc.items()
    }, f, indent=4)
