import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict


def part1(data):
    """ 2018 Day 22 Part 1

    >>> part1(['depth: 510', 'target: 10,10'])
    114
    """

    depth = [int(x) for x in re.findall(r'\d+', data[0])][0]
    target = tuple(int(x) for x in re.findall(r'\d+', data[1]))

    erosionLevels = {(0, 0): depth}

    erosionLevels[target] = calcErosion(target, erosionLevels, depth)
    erosionLevels[target] = depth

    return sum(e % 3 for e in erosionLevels.values())


def part2(data):
    """ 2018 Day 22 Part 2

    >>> part2(['depth: 510', 'target: 10,10'])
    45
    """

    depth = [int(x) for x in re.findall(r'\d+', data[0])][0]
    target = tuple(int(x) for x in re.findall(r'\d+', data[1]))

    erosionLevels = {(0, 0): depth}

    erosionLevels[target] = calcErosion(target, erosionLevels, depth)
    erosionLevels[target] = depth

    return bfs(startState = (0, 0, 1), end = tuple(list(target) + [1]), area = erosionLevels, d = depth, nextStateFunc = nextState, abortFunc = abort, trackFunc = track)


def bfs(startState, nextStateFunc, abortFunc, trackFunc, **kwargs):
    states = defaultdict(set)
    states[0].add(startState)

    visited = set()

    tracked = None
    while len(states) != 0:
        minT = min(states.keys())

        for state in states[minT]:
            if state in visited:
                continue

            visited.add(state)

            tracked = trackFunc(tracked = tracked, state = state, t = minT, visited = visited, **kwargs)

            if abortFunc(state = state, tracked = tracked, visited = visited, **kwargs):
                return minT if tracked is None else [minT, tracked]

            for newState, t in nextStateFunc(state = state, t = minT, tracked = tracked, visited = visited, **kwargs):
                states[t].add(newState)
                
        del(states[minT])

    return tracked
        

def calcErosion(pos, erosionLevels, depth):
    if pos[0] == 0:
        return (pos[1] * 48271 + depth) % 20183

    if pos[1] == 0:
        return (pos[0] * 16807 + depth) % 20183

    p1 = (pos[0] - 1, pos[1])
    p2 = (pos[0], pos[1] - 1)

    if p1 not in erosionLevels:
        erosionLevels[p1] = calcErosion(p1, erosionLevels, depth)

    if p2 not in erosionLevels:
        erosionLevels[p2] = calcErosion(p2, erosionLevels, depth)

    return (erosionLevels[p1] * erosionLevels[p2] + depth) % 20183


def nextState(state, area, d, t, **kwargs):
    *pos, tool = state
    pos = tuple(pos)

    newStates = []

    for offset in [[1, 0], [-1, 0], [0, 1], [0, -1]]:
        nPos = tuple(p + o for p, o in zip(pos, offset))
        if min(nPos) < 0:
            continue

        if nPos not in area:
            area[nPos] = calcErosion(nPos, area, d)

        if tool == area[nPos] % 3:
            newTool = [t for t in range(3) if t != tool and t != area[pos] % 3]
            newState = tuple(list(pos) + newTool)
            newStates.append([newState, t + 7])
        else:
            newState = tuple(list(nPos) + [tool])
            newStates.append([newState, t + 1])

    return newStates


def abort(state, end, **kwargs):
    return state == end


def track(**kwargs):
    return None


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
        print(f"\nPart 1:\nRisk Level: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nShortest time to reach target: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)