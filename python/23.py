import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2018 Day 23 Part 1

    >>> part1(['pos=<0,0,0>, r=4', 'pos=<1,0,0>, r=1', 'pos=<4,0,0>, r=3', 'pos=<0,2,0>, r=1', 'pos=<0,5,0>, r=3', 'pos=<0,0,3>, r=1', 'pos=<1,1,1>, r=1', 'pos=<1,1,2>, r=1', 'pos=<1,3,1>, r=1'])
    7
    """

    nanobots = [tuple([int(x) for x in re.findall(r'-?\d+', line)]) for line in data]

    largestRadius = nanobots[[n[-1] for n in nanobots].index(max([n[-1] for n in nanobots]))]
    inRadius = [n for n in nanobots if manhatDist(largestRadius[:-1], n[:-1]) <= largestRadius[-1]]

    return len(inRadius)


def part2(data):
    """ 2018 Day 23 Part 2

    >>> part2(['pos=<10,12,12>, r=2', 'pos=<12,14,12>, r=2', 'pos=<16,12,12>, r=4', 'pos=<14,14,14>, r=6', 'pos=<50,50,50>, r=200', 'pos=<10,10,10>, r=5'])
    36
    """

    nanobots = [tuple([int(x) for x in re.findall(r'-?\d+', line)]) for line in data]

    return bestLoc(nanobots)


def manhatDist(p1, p2):
    d = sum([abs(c2 - c1) for c1, c2 in zip(p1, p2)])
    return d


def iterate(point, nanobots, calculatedPoints):
    neighbors = [(x, y, z) for x in range(point[0] - 1, point[0] + 2) for y in range(point[1] - 1, point[1] + 2) for z in range(point[2] - 1, point[2] + 2)]
    
    if point not in calculatedPoints:
        calculatedPoints[point] = len([n for n in nanobots if manhatDist(n[:-1], point) <= n[-1]])

    maximum = calculatedPoints[point]
    maximumPoint = point

    for n in neighbors:
        if n not in calculatedPoints:
            calculatedPoints[n] = len([b for b in nanobots if manhatDist(b[:-1], n) <= b[-1]])

        if calculatedPoints[n] > maximum or calculatedPoints[n] == maximum and manhatDist(n, (0, 0, 0)) < manhatDist(maximumPoint, (0, 0, 0)):
            maximum = calculatedPoints[n]
            maximumPoint = n

    return maximumPoint


def find(visited, nanobots, cs, dist, offsets, forceCount):
    xs, ys, zs = cs
    ox, oy, oz = offsets
    boxes = []

    for x in range(min(xs), max(xs)+1, dist):
        for y in range(min(ys), max(ys)+1, dist):
            for z in range(min(zs), max(zs)+1, dist):
                count = 0
                for b in nanobots:
                    bdist = b[-1]
                    if dist == 1:
                        calc = manhatDist((x, y, z), b[:-1])
                        if calc <= bdist:
                            count += 1
                    else:
                        calc =  abs((ox+x) - (ox+b[0]))
                        calc += abs((oy+y) - (oy+b[1]))
                        calc += abs((oz+z) - (oz+b[2]))

                        if calc // dist - 3 <= (bdist) // dist:
                            count += 1

                if count >= forceCount:
                    boxes.append((x, y, z, count, abs(x) + abs(y) + abs(z)))

    while len(boxes) > 0:
        best = []
        bestIndex = None

        for i, b in enumerate(boxes):
            if bestIndex is None or b[4] < best[4]:
                best = b
                bestIndex = i

        if dist == 1:
            return best[4], best[3]
        else:
            xs = [best[0], best[0] + dist // 2]
            ys = [best[1], best[1] + dist // 2]
            zs = [best[2], best[2] + dist // 2]
            a, b = find(visited, nanobots, (xs, ys, zs), dist // 2, (ox, oy, oz), forceCount)
            if a is None:
                boxes.pop(bestIndex)
            else:
                return a, b

    return None, None


def bestLoc(nanobots):
    xs, ys, zs = [[n[i] for n in nanobots] + [0] for i in range(3)]

    dist = 1
    while dist < max(xs) - min(xs) or dist < max(ys) - min(ys) or dist < max(zs) - min(zs):
        dist *= 2

    ox, oy, oz = [-min(cs) for cs in [xs, ys, zs]]

    span = 1
    while span < len(nanobots):
        span *= 2

    forceCheck = 1
    visited = {}

    bestPos, bestCount = [None] * 2

    while True:
        if forceCheck not in visited:
            visited[forceCheck] = find(set(), nanobots, (xs, ys, zs), dist, (ox, oy, oz), forceCheck)

        pos, count = visited[forceCheck]

        if pos is None:
            if span > 1:
                span = span // 2
            forceCheck = max(1, forceCheck - span)
        else:
            if bestCount is None or count > bestCount:
                bestPos, bestCount = pos, count
            if span == 1:
                break

            forceCheck += span

    return bestPos


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
        print(f"\nPart 1:\nNanobots within largest radius: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nManhattan Distance to location in range of most nanobots: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)