import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import numpy as np


def part1(data):
    """ 2018 Day 18 Part 1

    >>> part1(['.#.#...|#.', '.....#|##|', '.|..|...#.', '..|#.....#', '#.#|||#|#|', '...#.||...', '.|....|...', '||...#|.#|', '|.||||..|.', '...#.|..|.'])
    1147
    """

    acres = np.array([np.array([l for l in line.strip('\n')]) for line in data])

    for _ in range(10):
        acres = iterate(acres)

    trees, lumber = counts(acres)

    return trees * lumber


def part2(data):
    """ 2018 Day 18 Part 2
    """

    acres = np.array([np.array([l for l in line.strip('\n')]) for line in data])

    gridStates = {}
    minutes = 0
    while minutes < 1000000000:
        minutes += 1
        acres = iterate(acres)

        gridStr = printAcres(acres)
        if gridStr in gridStates:
            minutes += ((1000000000 - minutes) // (minutes - gridStates[gridStr])) * (minutes - gridStates[gridStr]) 
        else:
            gridStates[gridStr] = minutes

    trees, lumber = counts(acres)

    return trees * lumber


def iterate(acres):
    newAcres = np.empty(acres.shape, dtype=acres.dtype)

    for y in range(len(acres)):
        for x in range(len(acres[y])):
            curr = acres[y, x]
            neighbors = np.array([acres[m, n] for n in range(x-1, x+2) for m in range(y-1, y+2) if 0 <= n < len(acres[y]) and 0 <= m < len(acres) and not (n == x and m == y)])
            adjTrees = len([t for t in neighbors if t == '|'])
            adjLumb = len([l for l in neighbors if l == '#'])

            if curr == '.':
                newAcres[y, x] = '|' if adjTrees >= 3 else '.'
            elif curr == '|':
                newAcres[y, x] = '#' if adjLumb >= 3 else '|'
            elif curr == '#':
                newAcres[y, x] = '#' if adjTrees >= 1 and adjLumb >= 1 else '.'

    return newAcres


def counts(acres):
    trees = 0
    lumber = 0
    for y in range(len(acres)):
        for x in range(len(acres[y])):
            if acres[y, x] == '|':
                trees += 1
            elif acres[y, x] == '#':
                lumber += 1

    return [trees, lumber]


def printAcres(acres):
    string = ''
    for y in range(len(acres)):
        for x in range(len(acres[y])):
            string += acres[y, x]

        string += '\n'

    return string


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = [line.strip('\n') for line in f.readlines()]

    with Timer() as p1_time:
        p1 = part1(data)

    if verbose:
        print(f"\nPart 1:\nResource collection value after 10 min: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nResource collection value after 1000000000 min: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)