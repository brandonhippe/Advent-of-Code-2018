import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def part1(data):
    """ 2018 Day 12 Part 1

    >>> part1(['initial state: #..#.#..##......###...###', '', '...## => #', '..#.. => #', '.#... => #', '.#.#. => #', '.#.## => #', '.##.. => #', '.#### => #', '#.#.# => #', '#.### => #', '##.#. => #', '##.## => #', '###.. => #', '###.# => #', '####. => #'])
    325
    """

    plantState = defaultdict(lambda: '.')
    for i, c in enumerate(data[0].split(": ")[1]):
        plantState[i] = c

    rules = {line.split(" => ")[0] for line in data[2:] if line.split(" => ")[1] == '#'}

    genData = [sum(plantState.keys())]
    deltas = []
    for gen in range(20):
        plantState = iterate(plantState, rules)
        totalPlants = sum(plantState.keys())
        genData.append(totalPlants)
        deltas.append(genData[gen] - genData[gen - 1])
    
    return totalPlants


def part2(data):
    """ 2018 Day 12 Part 2
    """

    plantState = defaultdict(lambda: '.')
    for i, c in enumerate(data[0].split(": ")[1]):
        plantState[i] = c

    rules = {line.split(" => ")[0] for line in data[2:] if line.split(" => ")[1] == '#'}

    genData = [sum(plantState.keys())]
    deltas = []
    for gen in range(50000000000):
        if gen >= 20 and len(set(deltas[-10:])) == 1:
            break

        plantState = iterate(plantState, rules)
        totalPlants = sum(plantState.keys())
        genData.append(totalPlants)
        deltas.append(genData[gen] - genData[gen - 1])
    
    return totalPlants + ((50000000000 - gen) * deltas[-1])


def iterate(plantState, rules):
    newPlantState = defaultdict(lambda: '.')

    for i in range(min(plantState.keys()) - 2, max(plantState.keys()) + 3):
        plantString = ''.join(plantState[i + c] for c in range(-2, 3))
        if plantString in rules:
            newPlantState[i] = '#'

    return newPlantState


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
        print(f"\nPart 1:\nSum of all numbers of pots that contain plants after 20 generations: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of all numbers of pots that contain plants after 50,000,000,000 generations: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)