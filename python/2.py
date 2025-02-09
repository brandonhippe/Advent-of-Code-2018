import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import Counter


def part1(data):
    """ 2018 Day 2 Part 1

    abcdef contains no letters that appear exactly two or three times.
    bababc contains two a and three b, so it counts for both.
    abbcde contains two b, but no letter appears exactly three times.
    abcccd contains three c, but no letter appears exactly two times.
    aabcdd contains two a and two d, but it only counts once.
    abcdee contains two e.
    ababab contains three a and three b, but it only counts once.

    >>> part1(['abcdef', 'bababc', 'abbcde', 'abcccd', 'aabcdd', 'abcdee', 'ababab'])
    12
    """

    twos = 0
    threes = 0
    for line in data:
        letters = Counter(line)
        twos += 1 if 2 in letters.values() else 0
        threes += 1 if 3 in letters.values() else 0

    return twos * threes


def part2(data):
    """ 2018 Day 2 Part 2

    >>> part2(['abcde', 'fghij', 'klmno', 'pqrst', 'fguij', 'axcye', 'wvxyz'])
    'fgij'
    """

    common = ''
    for i, l1 in enumerate(data[:-1]):
        for _, l2 in enumerate(data[i + 1:]):
            diffCount = 0
            for c1, c2 in zip(l1, l2):
                if c1 != c2:
                    diffCount += 1

            if diffCount == 1:
                for c1, c2 in zip(l1, l2):
                    common += c1 if c1 == c2 else ''
                break

        if len(common) != 0:
            break

    return common


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
        print(f"\nPart 1:\nChecksum: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nCommon letters between box IDs: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)