#!/usr/bin/env python3

"""Wooden Board Usage Calculator

Works out the number of boards you need for a project, based on the pieces of wood you need to buy.

Modify the constants at the top to fit your project.
"""

# Types of board you will need.
SLAT = "slat"
POST = "post"

# Lengths of board you can buy.
LENGTHS = {
    SLAT: 2400,
    POST: 2400,
}

# Number of screws needed (on average) for each wood type.
SCREWS = {
    SLAT: 4,
    POST: 2,
}

# Which board lengths you need to build each component, and how many of each.
A = {
    # This means 12x300mm lengths, 12x350mm lengths etc.
    SLAT: [(12, 300), (12, 350), (4, 300)],
    POST: [(4, 350), (2, 350)],
}
B = {
    SLAT: [(8, 450), (8, 200), (5, 200)],
    POST: [(4, 300), (2, 450)],
}
C = {
    SLAT: [(8, 950), (8, 300), (8, 300)],
    POST: [(4, 1000), (2, 850)],
}

# The set of components you need to build (should be all the ones above).
COMPONENTS = [A, A, B, B, C]

# The wastage to allow per cut (Default: 10mm).
KERF = 10

# Constants.
TOTAL = "total"
SEGMENTS = "segments"


def main():
    total_lengths = collect_lengths()
    boards_needed = pack_boards(total_lengths)
    print_boards(boards_needed)


def collect_lengths():
    total_lengths = {
        # List of lengths, e.g.
        #   [100, 200, 100]
        SLAT: [],
        POST: [],
    }
    for component in COMPONENTS:
        for board, lengths in component.items():
            for count, length in lengths:
                for i in range(count):
                    total_lengths[board].append(length)
        for lengths in total_lengths.values():
            lengths.sort()
            lengths.reverse()
    return total_lengths


def pack_boards(total_lengths):
    # List of board usages, e.g.
    #   [{ total: 2000, segments: [400, 500, 600, 500] }, ...]
    usages = {
        SLAT: [blank_board()],
        POST: [blank_board()],
    }
    for board, lengths in total_lengths.items():
        for length in lengths:
            board_usages = usages[board]
            if board_usages[-1][TOTAL] != 0:
                board_usages.append(blank_board())
            most_full_usable_board_index = max(
                filter(
                    lambda enum_usage: LENGTHS[board] > enum_usage[1][TOTAL] +
                    length + ((len(enum_usage[1][SEGMENTS]) - 1) * KERF),
                    enumerate(board_usages),
                ),
                key=lambda enum_usage: enum_usage[1][TOTAL],
            )[0]
            board_usages[most_full_usable_board_index][TOTAL] += length
            board_usages[most_full_usable_board_index][SEGMENTS].append(length)

    for lengths in usages.values():
        if lengths[-1] == blank_board():
            removed = lengths.pop()
    return usages


def blank_board():
    return {TOTAL: 0, SEGMENTS: []}


def print_boards(boards_needed):
    """Show the boards used.

    Prints the total needed number of each board.
    Then shows the lenghts it will need to be cut into, and the leftover, which should be large
    enough to include the kerf width (wood wastage from the blade cuts).
    """
    for board, usages in boards_needed.items():
        total_segments = sum((len(usage[SEGMENTS]) for usage in usages))
        wastage = sum((LENGTHS[board] - usage[TOTAL] for usage in usages))
        screws = total_segments * SCREWS[board]
        print(
            f"{board}: {len(usages)} ({SEGMENTS}: {total_segments} | wastage: {wastage} | screws: {screws})")
        for usage in usages:
            print(
                f"  {usage[TOTAL]} => {usage[SEGMENTS]} - {LENGTHS[board] - usage[TOTAL]}"
            )


if __name__ == "__main__":
    main()
