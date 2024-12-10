#!/usr/bin/env python3

# pylint: disable=missing-function-docstring

"""Question 2"""

# TODO: get input


def main():
    """Run code"""
    r = 9
    b = 3
    s = 3
    f = 1

    (red, blue) = play_game_of_drones(r, b, s, f)
    print(red)
    print(blue)


def play_game_of_drones(r: int, b: int, s: int, f: int) -> tuple[int, int]:
    red_hexagons_jumped_to = r
    blue_hexagons_jumped_to = b

    skirmishes = s  # Number of skirmishes to do
    feuds = f  # Number of feuds to do

    # Output
    red_hexagons_controlled = 0
    blue_hexagons_controlled = 0

    # TODO: all the code

    # skirmishes

    # feuds

    return (red_hexagons_controlled, blue_hexagons_controlled)


if __name__ == "__main__":
    main()
