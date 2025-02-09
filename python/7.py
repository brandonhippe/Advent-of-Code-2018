import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
import heapq


def part1(data):
    """ 2018 Day 7 Part 1

    >>> part1(['Step C must be finished before step A can begin.', 'Step C must be finished before step F can begin.', 'Step A must be finished before step B can begin.', 'Step A must be finished before step D can begin.', 'Step B must be finished before step E can begin.', 'Step D must be finished before step E can begin.', 'Step F must be finished before step E can begin.'])
    'CABDFE'
    """

    pattern = f"( {' | '.join(chr(c) for c in range(ord('A'), ord('Z') + 1)) } )"
    nodes = {}
    for line in data:
        step, substep = re.findall(pattern, line)
        if step not in nodes:
            nodes[step] = Node(step)

        if substep not in nodes:
            nodes[substep] = Node(substep)

        nodes[substep].addNeed(nodes[step])

    return order(nodes)


def part2(data, workers = 5, baseTime = 60):
    """ 2018 Day 7 Part 2

    >>> part2(['Step C must be finished before step A can begin.', 'Step C must be finished before step F can begin.', 'Step A must be finished before step B can begin.', 'Step A must be finished before step D can begin.', 'Step B must be finished before step E can begin.', 'Step D must be finished before step E can begin.', 'Step F must be finished before step E can begin.'], 2, 0)
    15
    """

    pattern = f"( {' | '.join(chr(c) for c in range(ord('A'), ord('Z') + 1)) } )"
    nodes = {}
    for line in data:
        step, substep = re.findall(pattern, line)
        if step not in nodes:
            nodes[step] = Node(step)

        if substep not in nodes:
            nodes[substep] = Node(substep)

        nodes[substep].addNeed(nodes[step])

    return orderTimed(nodes, workers, baseTime)



class Node:
    def __init__(self, name):
        self.name = name
        self.needs = []
        self.visited = False
        self.inProgress = False
        self.timeLeft = ord(self.name[1]) - ord('A') + 1

    def addNeed(self, other):
        self.needs.append(other)

    def available(self):
        for needed in self.needs:
            if not needed.visited:
                return False
        
        return True

    def __lt__(self, other):
        return ord(self.name[1]) < ord(other.name[1])
    

def order(nodes):
    free = []
    orderString = ''

    while len([n for n in nodes.values() if n.visited]) < len(nodes):
        for n in nodes.values():
            if n not in free and not n.visited and n.available():
                heapq.heappush(free, n)

        n = heapq.heappop(free)
        orderString += n.name[1:-1]
        n.visited = True

    return orderString


def orderTimed(nodes, numWorkers, baseTime):
    free = []
    freeWorkers = numWorkers
    for n in nodes.values():
        n.timeLeft += baseTime

    timeTaken = 0
    while len([n for n in nodes.values() if n.visited]) < len(nodes):
        for n in nodes.values():
            if n not in free and not n.visited and not n.inProgress and n.available():
                heapq.heappush(free, n)

        while len(free) > 0 and freeWorkers > 0:
            n = heapq.heappop(free)
            n.inProgress = True
            freeWorkers -= 1

        for n in nodes.values():
            if n.inProgress:
                n.timeLeft -= 1

                if n.timeLeft == 0:
                    n.visited = True
                    n.inProgress = False
                    freeWorkers += 1

        timeTaken += 1

    return timeTaken


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
        print(f"\nPart 1:\nCorrect Order: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTime Taken: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)