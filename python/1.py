import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2018 Day 1 Part 1

    >>> part1(['1', '-2', '+3', '+1'])
    3
    >>> part1(['+1', '+1', '+1'])
    3
    >>> part1(['+1', '+1', '-2'])
    0
    >>> part1(['-1', '-2', '-3'])
    -6
    """

    return sum(int(n) for n in data)


def part2(data):
    """ 2018 Day 1 Part 2

    >>> part2(['1', '-2', '+3', '+1'])
    2
    >>> part2(['+1', '-1'])
    0
    >>> part2(['+3', '+3', '+4', '-2', '-4'])
    10
    >>> part2(['-6', '+3', '+8', '+5', '-6'])
    5
    >>> part2(['+7', '+7', '-2', '-7', '-4'])
    14
    """

    nums = [int(n) for n in data]

    history = set()
    frequency = 0
    i = 0
    while True:
        if frequency in history:
            break

        history.add(frequency)
        frequency += nums[i]
        i += 1
        i %= len(nums)

    return frequency


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
        print(f"\nPart 1:\nFrequency: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFirst frequency reached twice: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)