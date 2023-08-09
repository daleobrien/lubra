#!/usr/bin/env python3
from itertools import combinations
from itertools import chain
#],
LINES = [[],
         #[1,2], [2,3], [1,3], [1,2,3],
         [2,5], [3,4],  [1,6],
         ]
CURVES = [[], [4,5], [5,6], [4,6], [4,5,6]]
POINTS = [[], [1], [2], [3], [4], [5], [6]]

def intersect_lists(lists):
    for i in range(len(lists)):
        for j in range(i+1, len(lists)):
            if any(item in lists[j] for item in lists[i]):
                return True
    return False

def flatten(matrix):
     return sorted(tuple(chain.from_iterable(matrix)))

def gen():
    all_lines = list(combinations(LINES,1)) + list(combinations(LINES, 2)) + list(combinations(LINES, 3))
    all_curves = list(combinations(CURVES,1)) + list(combinations(CURVES, 2)) + list(combinations(CURVES, 3))
    all_points = list(combinations(POINTS,1)) + list(combinations(POINTS, 2)) + list(combinations(POINTS, 3)) + list(combinations(POINTS, 4)) + list(combinations(POINTS, 5)) + list(combinations(POINTS, 6))

    tiles = set([tuple([])])

    for lines in all_lines:
        # Line nets can't share a vertex
        if intersect_lists(lines):
            continue

        for curves in all_curves:
            if intersect_lists(curves):
                continue

            if ([1,2] in lines or [2,3] in lines or [1,3] in lines or [1,2,3] in lines) and flatten(curves):
                continue

            for points in all_points:
                nets = tuple([tuple(l) for l in lines if l] + [tuple(c) for c in curves if c] + [tuple(p) for p in points if p])

                if intersect_lists(nets):
                    continue

                tiles.add(nets)
    return tiles

lookups = {
    1: "①",
    2: "②",
    3: "③",
    4: "④",
    5: "⑤",
    6: "⑥",
}

def pretty(net):
    if not net:
        return ""
    return "-".join([str(n) for n in net])


def to_bits(nets):
    nodes = flatten(nets)
    bits = 0
    for n in nodes:
        bits |= 1 << (n-1)
    return bits


def to_mask(nets):
    bits = to_bits(nets)
    return f'0b{bits:08b}'


def as_vec(nets, pad=False):
    if pad:
        return f"vec![ {', '.join([str(n) for n in sorted(nets)])} ]"
    else:
        return f"vec![{', '.join([str(n) for n in sorted(nets)])}]"


def dont_care_to_trit(bits):
     # 0b101 -> 0b10_00_10
    trits = 0
    for j in range(6):
        b = 2 << (2 * j)
        if bits & (1 << j) == (1 << j):
            trits |= b
    return trits

def mask_to_trit(bits):
    # 0b101 -> 0b01_00_01
    trits = 0
    for j in range(6):
        b = 1 << (2 * j)
        if bits & (1 << j) == (1 << j):
            trits |= b
    return trits


def binary_to_base4(binary_str):
    decimal_num = int(binary_str, 2)  # Convert binary string to decimal
    base4_num = ""  # Initialize the base 3 representation

    while decimal_num > 0:
        remainder = decimal_num % 4
        base4_num = str(remainder) + base4_num
        decimal_num //= 4

    base4_num = base4_num.rjust(6, "0")
    return base4_num if base4_num else "000000"  # Handle the case when input is "0"


def create_lookups(nets):
    lookups = {}
    for n in range(64):
        dont_care = dont_care_to_trit(n)
        for bits in sorted(nets.keys()):
            values = nets[bits]
            index = mask_to_trit(bits & ~n & 0b111_111) | dont_care

            # sorted_tiles = sorted(nets)

            # 0, clear, 1 set, 2 don't care, 3- unused
            # I think using base 4 will make it more readable
            s = binary_to_base4(f"{index:012b}")

            k = f"{bits:012b}"
            # print(f"{bits:012b}", s)
            if s not in lookups:
                lookups[s] = []
            lookups[s].extend(values)

    return lookups


if __name__ == "__main__":

    # binary_string = "101010"
    #
    # base3_result = binary_to_base4(binary_string)
    # print(f"Binary {binary_string} in base 3 is: {base3_result}")
    #
    # exit(0)
    nets = gen()
    # print(len(nets))
    mask_to_nets = {}
    # TODO: Create templates and generate RUST files
    for nets_for_one_tile in sorted(list(nets)):
        # print(", ".join([pretty(n) for n in net]))

        s = to_bits(nets_for_one_tile)
        if s not in mask_to_nets:
            mask_to_nets[s] = []
        # mask_to_nets[s].append(as_vec([as_vec(n) for n in nets_for_one_tile], pad=True))
        mask_to_nets[s].append(sorted([sorted(n) for n in nets_for_one_tile]))

        # print(s)
    print()
    keys = sorted(mask_to_nets.keys())
    # for k in keys:
    #     # print(f"{k} =>  vec![{', '.join(mask_to_nets[k])}],")
    #     print(k, len(mask_to_nets[k]) )

    # base 4 lookups
    lookups = create_lookups(mask_to_nets)
    for i, k in enumerate(sorted(lookups.keys())):
        print(f"0b{k} =>  {lookups[k]}")
