#!/usr/bin/env python3

import os

def readInput(filename):
    with open(filename) as f:
        return f.readlines()

def part1(input):
    for i in input:
        if 2020 - i in input:
            return i * (2020 - i)

def part2(input):
    for i in range(len(input) - 2):
        for j in range(i + 1, len(input) - 1):
            if 2020 - input[i] - input[j] in input:
                return input[i] * input[j] * (2020 - input[i] - input[j])

def main():
    # inputfile = os.path.join(os.path.dirname(__file__), 'test.txt')
    inputfile = os.path.join(os.path.dirname(__file__), 'input.txt')

    input = readInput(inputfile)

    input = [int(v.strip()) for v in input]

    print(f"Part 1: {part1(input)}")
    print(f"Part 2: {part2(input)}")

if __name__ == "__main__":
    main()
