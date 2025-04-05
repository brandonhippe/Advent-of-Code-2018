import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
import copy


def part1(data):
    """ 2018 Day 10 Part 1
    """

    return printPoints(iteratePoints(tuple(Point(line) for line in data))[0])


def part2(data):
    """ 2018 Day 10 Part 2

    >>> part2(['position=< 9,  1> velocity=< 0,  2>', 'position=< 7,  0> velocity=<-1,  0>', 'position=< 3, -2> velocity=<-1,  1>', 'position=< 6, 10> velocity=<-2, -1>', 'position=< 2, -4> velocity=< 2,  2>', 'position=<-6, 10> velocity=< 2, -2>', 'position=< 1,  8> velocity=< 1, -1>', 'position=< 1,  7> velocity=< 1,  0>', 'position=<-3, 11> velocity=< 1, -2>', 'position=< 7,  6> velocity=<-1, -1>', 'position=<-2,  3> velocity=< 1,  0>', 'position=<-4,  3> velocity=< 2,  0>', 'position=<10, -3> velocity=<-1,  1>', 'position=< 5, 11> velocity=< 1, -2>', 'position=< 4,  7> velocity=< 0, -1>', 'position=< 8, -2> velocity=< 0,  1>', 'position=<15,  0> velocity=<-2,  0>', 'position=< 1,  6> velocity=< 1,  0>', 'position=< 8,  9> velocity=< 0, -1>', 'position=< 3,  3> velocity=<-1,  1>', 'position=< 0,  5> velocity=< 0, -1>', 'position=<-2,  2> velocity=< 2,  0>', 'position=< 5, -2> velocity=< 1,  2>', 'position=< 1,  4> velocity=< 2,  1>', 'position=<-2,  7> velocity=< 2, -2>', 'position=< 3,  6> velocity=<-1, -1>', 'position=< 5,  0> velocity=< 1,  0>', 'position=<-6,  0> velocity=< 2,  0>', 'position=< 5,  9> velocity=< 1, -2>', 'position=<14,  7> velocity=<-2,  0>', 'position=<-3,  6> velocity=< 2, -1>'])
    3
    """

    return iteratePoints(tuple(Point(line) for line in data))[1]


class Point:
    def __init__(self, infoLine):
        data = [int(x) for x in re.findall(r'[- ]?\d+', infoLine)]
        self.pos = data[:len(data)//2]
        self.vel = data[len(data)//2:]

    def applyVel(self, iterations=1):
        for i, (p, v) in enumerate(zip(self.pos, self.vel)):
            self.pos[i] = p + (v * iterations)

    def __str__(self) -> str:
        return ",".join(str(x) for x in self.pos)
    

def cross(v1, v2):
    return v1[0] * v2[1] - v1[1] * v2[0]


def coordRange(points):
    return [max(p.pos[i] for p in points) - min(p.pos[i] for p in points) + 1 for i in range(2)]


def iteratePoints(points):
    best = float('-inf')
    for p1 in points:
        for p2 in points:
            if p1 == p2:
                continue
            
            c = abs(cross(p1.vel, p2.vel))
            if c > best:
                perpVel = [p1, p2]
                best = c

    p1, p2 = perpVel
    for i in range(2):
        if abs(p2.vel[i] - p1.vel[i]) != 0:
            timeSteps = abs(p2.pos[i] - p1.pos[i]) // abs(p2.vel[i] - p1.vel[i])
            break

    timeSteps -= 10
    if timeSteps < 0:
        timeSteps = 0

    for p in points:
        p.applyVel(iterations=timeSteps)

    while True:
        pPoints = copy.deepcopy(points)

        for p in points:
            p.applyVel()

        if coordRange(pPoints) < coordRange(points):
            break

        timeSteps += 1

    return [pPoints, timeSteps]


def printPoints(points):
    points = {str(p): p for p in points}
    mins = [min(p.pos[i] for p in points.values()) for i in range(2)]
    maxs = [max(p.pos[i] for p in points.values()) for i in range(2)]
    
    s = ''
    for y in range(mins[1], maxs[1] + 1):
        line = ''
        for x in range(mins[0], maxs[0] + 1):
            line += '█' if ",".join([str(x), str(y)]) in points else ' '

        s = '\n'.join([s, line])

    return s


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
        print(f"\nPart 1:\nMessage: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSeconds passed before message appeared: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)