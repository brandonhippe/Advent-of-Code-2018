import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2018 Day 17 Part 1

    >>> part1(['x=495, y=2..7', 'y=7, x=495..501', 'x=501, y=3..7', 'x=498, y=2..4', 'x=506, y=1..2', 'x=498, y=10..13', 'x=504, y=10..13', 'y=13, x=498..504'])
    57
    """

    clay = set()
    for line in data:
        xText = re.findall('x=\d+\.*\d*', line)[0][2:]
        yText = re.findall('y=\d+\.*\d*', line)[0][2:]
        
        x = [int(n) for n in re.split('\.\.', xText)]
        y = [int(n) for n in re.split('\.\.', yText)]

        if len(x) == 1:
            for i in range(y[0], y[1] + 1):
                clay.add((x[0], i))
        elif len(y) == 1:
            for i in range(x[0], x[1] + 1):
                clay.add((i, y[0]))

    water = {}
    return flood(clay, water, (500, 0))


def part2(data):
    """ 2018 Day 17 Part 2

    >>> part2(['x=495, y=2..7', 'y=7, x=495..501', 'x=501, y=3..7', 'x=498, y=2..4', 'x=506, y=1..2', 'x=498, y=10..13', 'x=504, y=10..13', 'y=13, x=498..504'])
    29
    """

    clay = set()
    for line in data:
        xText = re.findall('x=\d+\.*\d*', line)[0][2:]
        yText = re.findall('y=\d+\.*\d*', line)[0][2:]
        
        x = [int(n) for n in re.split('\.\.', xText)]
        y = [int(n) for n in re.split('\.\.', yText)]

        if len(x) == 1:
            for i in range(y[0], y[1] + 1):
                clay.add((x[0], i))
        elif len(y) == 1:
            for i in range(x[0], x[1] + 1):
                clay.add((i, y[0]))

    water = {}
    flood(clay, water, (500, 0))
    return len([w for w in water.keys() if water[w]])


def expand(clay, water, start, dir):
    total = 0
    x, y = start
    settled = []
    while (x, y) not in clay:
        total += 1
        if not ((x, y + 1) in clay or ((x, y + 1) in water and water[(x, y + 1)])):
            total += flood(clay, water, (x, y))
            if not water[(x, y + 1)]:
                return [False, settled, total]
        
        settled.append((x, y))
        water[(x, y)] = True

        x += dir * 1

    return [True, settled, total]


def flood(clay, water, spring):
    total = 0
    highest = min(c[1] for c in clay)
    lowest = max(c[1] for c in clay)
    
    y = spring[1]
    while (spring[0], y) not in clay and not (spring[0], y) in water and y <= lowest:
        water[(spring[0], y)] = False
        y += 1
        total += highest <= y

    y -= 1
    total -= 1

    if y == lowest or ((spring[0], y + 1) in water and not water[(spring[0], y + 1)]):
        return total

    while y > spring[1]:
        breaking = False
        layer = []
        for dir in [-1, 1]:
            settled, layer_side, visited = expand(clay, water, (spring[0] + (dir * 1), y), dir)

            total += visited
            layer += layer_side
            if not settled:
                breaking = True

        if breaking:
            for p in layer:
                water[p] = False

            break

        water[spring[0], y] = True
        y -= 1

    return total


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
        print(f"\nPart 1:\nTiles reached by water: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nWater tiles left after spring dries: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)