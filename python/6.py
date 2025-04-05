import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2018 Day 6 Part 1

    >>> part1(['1, 1', '1, 6', '8, 3', '3, 4', '5, 5', '8, 9'])
    17
    """

    points = [[int(x) for x in re.findall(r'\d+', line.strip('\n'))] for line in data]

    hull = convexHull(points)
    regions = {','.join([str(c) for c in point]): [] for point in points if point not in hull}
    
    mins = [min([p[i] for p in hull]) for i in range(len(hull[0]))]
    maxs = [max([p[i] for p in hull]) for i in range(len(hull[0]))]

    for y in range(mins[1], maxs[1] + 1):
        for x in range(mins[0], maxs[0] + 1):
            best = [float('inf'), []]
            for p in points:
                d = manhatDist([x, y], p)
                if d == best[0]:
                    best[1].append(p)
                elif d < best[0]:
                    best = [d, [p]]
            
            if len(best[1]) == 1 and ','.join([str(p) for p in best[1][0]]) in regions:
                regions[','.join([str(p) for p in best[1][0]])].append([x, y])
    
    return max(len(s) for s in regions.values())


def part2(data, totDist = 10000):
    """ 2018 Day 6 Part 2

    >>> part2(['1, 1', '1, 6', '8, 3', '3, 4', '5, 5', '8, 9'], 32)
    16
    """

    points = [[int(x) for x in re.findall(r'\d+', line.strip('\n'))] for line in data]

    hull = convexHull(points)
    
    mins = [min([p[i] for p in hull]) for i in range(len(hull[0]))]
    maxs = [max([p[i] for p in hull]) for i in range(len(hull[0]))]

    closeCount = 0
    for y in range(mins[1], maxs[1] + 1):
        for x in range(mins[0], maxs[0] + 1):
            if sum([manhatDist([x, y], p) for p in points]) < totDist:
                closeCount += 1

    return closeCount


def manhatDist(p1, p2):
    return sum([abs(c1 - c2) for c1, c2 in zip(p1, p2)])


def convexHull(points):    
    hull = []
    for point in points:
        x1, y1 = point
        for x2, y2 in points:
            if x1 == x2 and y1 == y2:
                continue

            a = y2 - y1
            b = x1 - x2
            c = x1 * y2 - y1 * x2

            left, right = 0, 0

            for x, y in points:
                if (x == x1 and y == y1) or (x == x2 and y == y2):
                    continue

                if a * x + b * y < c:
                    left += 1
                elif a * x + b * y > c:
                    right += 1

            if left == 0 or right == 0:
                hull.append(point)
                break

    return hull


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
        print(f"\nPart 1:\nSize of the largest finite area: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSize of close region: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)