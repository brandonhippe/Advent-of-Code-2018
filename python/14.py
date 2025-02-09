import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2018 Day 14 Part 1

    If the Elves think their skill will improve after making 9 recipes, the scores of the ten recipes after the first nine on the scoreboard would be 5158916779 (highlighted in the last line of the diagram).
    After 5 recipes, the scores of the next ten would be 0124515891.
    After 18 recipes, the scores of the next ten would be 9251071085.
    After 2018 recipes, the scores of the next ten would be 5941429882

    >>> part1(['9'])
    '5158916779'
    >>> part1(['5'])
    '0124515891'
    >>> part1(['18'])
    '9251071085'
    >>> part1(['2018'])
    '5941429882'
    """

    amt = int(data[0]) + 10

    scores = [3, 7]
    a, b = 0, 1
    while len(scores) < amt:
        s = scores[a] + scores[b]
        scores.extend(divmod(s, 10) if s >= 10 else (s,))

        a += scores[a] + 1
        b += scores[b] + 1
        a %= len(scores)
        b %= len(scores)

    return ''.join(str(n) for n in scores[amt - 10: amt])


def part2(data):
    """ 2018 Day 14 Part 2

    51589 first appears after 9 recipes.
    01245 first appears after 5 recipes.
    92510 first appears after 18 recipes.
    59414 first appears after 2018 recipes.

    >>> part2(['51589'])
    9
    >>> part2(['01245'])
    5
    >>> part2(['92510'])
    18
    >>> part2(['59414'])
    2018
    """

    searchFor = list(map(int, data[0]))
    l = len(searchFor)

    scores = [3, 7]
    a, b = 0, 1
    while not (scores[-1 - l: -1] == searchFor or scores[-l:] == searchFor):
        s = scores[a] + scores[b]
        scores.extend(divmod(s, 10) if s >= 10 else (s,))

        a += scores[a] + 1
        b += scores[b] + 1
        a %= len(scores)
        b %= len(scores)

    return len(scores) - len(data[0]) - (1 if scores[-len(data[0]):] != searchFor else 0)


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
        print(f"\nPart 1:\nNext 10 scores: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFirst occurance of input: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)