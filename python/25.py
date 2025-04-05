import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2018 Day 25 Part 1

    >>> part1([' 0,0,0,0', ' 3,0,0,0', ' 0,3,0,0', ' 0,0,3,0', ' 0,0,0,3', ' 0,0,0,6', ' 9,0,0,0', '12,0,0,0'])
    2
    >>> part1(['-1,2,2,0', '0,0,2,-2', '0,0,0,-2', '-1,2,0,0', '-2,-2,-2,2', '3,0,2,-1', '-1,3,2,2', '-1,0,-1,0', '0,2,1,-2', '3,0,0,0'])
    4
    >>> part1(['1,-1,0,1', '2,0,-1,0', '3,2,-1,0', '0,0,3,1', '0,0,-1,-1', '2,3,-2,0', '-2,2,0,0', '2,-2,0,-1', '1,-1,0,-1', '3,2,0,2'])
    3
    >>> part1(['1,-1,-1,-2', '-2,-2,0,1', '0,2,1,3', '-2,3,-2,1', '0,2,3,-2', '-1,-1,1,-2', '0,-2,-1,0', '-2,2,3,-1', '1,2,2,0', '-1,-2,0,-2'])
    8
    """

    points = [tuple(int(x) for x in re.findall(r'-?\d+', line)) for line in data]

    neighbors = {p: [p1 for p1 in points if p != p1 and manhatDist(p, p1) <= 3] for p in points}
    return len(scc(points, neighbors))


def part2(data):
    """ 2018 Day 25 Part 2
    """

    return "Christmas has been saved!"


def manhatDist(p1, p2):
    d = sum([abs(c2 - c1) for c1, c2 in zip(p1, p2)])
    return d


def scc(points, neighbors):
    visited = set()
    components = []

    for p in points:
        if p in visited:
            continue
        
        components.append([])
        openList = [p]

        while len(openList) != 0:
            currP = openList.pop(0)
            components[-1].append(currP)
            
            for n in neighbors[currP]:
                if n not in visited:
                    openList.append(n)

            visited.add(currP)

    return components


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
        print(f"\nPart 1:\nNumber of constellations: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\n{p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)