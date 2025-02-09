import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2018 Day 3 Part 1

    >>> part1(['#1 @ 1,3: 4x4', '#2 @ 3,1: 4x4', '#3 @ 5,5: 2x2'])
    4
    """

    claims = []
    for line in data:
        # Creates format [minX, minY, maxX, maxY]
        claims.append([int(x) for x in re.findall('\d+', line)[1:]])
        for i in range(2):
            claims[-1][i + 2] += claims[-1][i] - 1

    return findOverlaps(claims)


def part2(data):
    """ 2018 Day 3 Part 2

    >>> part2(['#1 @ 1,3: 4x4', '#2 @ 3,1: 4x4', '#3 @ 5,5: 2x2'])
    3
    """

    claims = []
    for line in data:
        # Creates format [minX, minY, maxX, maxY]
        claims.append([int(x) for x in re.findall('\d+', line)[1:]])
        for i in range(2):
            claims[-1][i + 2] += claims[-1][i] - 1

    return nonOverlap(claims)


def findIntersection(box1, box2):
    maxX, maxY = [max(box1[i], box2[i]) for i in (0, 1)]
    minX, minY = [min(box1[i], box2[i]) for i in (2, 3)]

    if minX >= maxX and minY >= maxY:
        return [maxX, maxY, minX, minY]
    else:
        return 0
    

def nonOverlap(data):
    finished = []
    intersections = []

    for box in data:
        for otherBox in finished:
            intersection = findIntersection(box, otherBox)

            if intersection != 0:
                intersections.append(intersection)

        finished.append(box)

    for i, box in enumerate(data):
        for foundIntersection in intersections:
            intersection = findIntersection(box, foundIntersection)
            if intersection != 0:
                break

        if intersection == 0:
            return i + 1

    return -1


def findOverlaps(claims):
    claimed = {}
    overlaps = {}

    for minX, minY, maxX, maxY in claims:
        for x in range(minX, maxX + 1):
            for y in range(minY, maxY + 1):
                locStr = f"{x},{y}"
                if locStr in claimed:
                    if locStr not in overlaps:
                        overlaps[locStr] = 1
                else:
                    claimed[locStr] = 1

    return len(overlaps.keys())


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
        print(f"\nPart 1:\nInches of fabric within 2+ claims: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nOnly claim with no overlaps: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)