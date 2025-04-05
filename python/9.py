import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import deque


def part1(data):
    """ 2018 Day 9 Part 1

    10 players; last marble is worth 1618 points: high score is 8317
    13 players; last marble is worth 7999 points: high score is 146373
    17 players; last marble is worth 1104 points: high score is 2764
    21 players; last marble is worth 6111 points: high score is 54718
    30 players; last marble is worth 5807 points: high score is 37305

    >>> part1(['9 players; last marble is worth 25 points'])
    32
    >>> part1(['10 players; last marble is worth 1618 points'])
    8317
    >>> part1(['13 players; last marble is worth 7999 points'])
    146373
    >>> part1(['17 players; last marble is worth 1104 points'])
    2764
    >>> part1(['21 players; last marble is worth 6111 points'])
    54718
    >>> part1(['30 players; last marble is worth 5807 points'])
    37305
    """

    return marbleGame(*[int(x) for x in re.findall(r'\d+', data[0])])


def part2(data):
    """ 2018 Day 9 Part 2
    """

    players, marbles = [int(x) for x in re.findall(r'\d+', data[0])]
    return marbleGame(players, marbles * 100)


def marbleGame(players, marbles):
    scores = [0] * players
    circle = deque([0])

    for marble in range(1, marbles + 1):
        if marble % 23:
            circle.rotate(-1)
            circle.append(marble)
        else:
            circle.rotate(7)
            scores[marble % players] += marble + circle.pop()
            circle.rotate(-1)

    return max(scores)


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
        print(f"\nPart 1:\nHigh score: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nHigh score: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)