import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2018 Day 13 Part 1

    >>> part1(['/->-\        ', '|   |  /----\ ', '| /-+--+-\  |', '| | |  | v  |', '\-+-/  \-+--/', '  \------/   '])
    '7,3'
    """

    carts = []
    tracks = {}
    POIs = {}
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l in '<>^v':
                carts.append(Cart(complex(x, y), l))
                tracks[complex(x, y)] = '-' if l in '<>' else '|'
            elif l in '/\+':
                POIs[complex(x, y)] = POI(complex(x, y), l)
            elif l in '-|':
                tracks[complex(x, y)] = l

    while True:
        carts.sort()
        for cart in carts:
            if not cart.alive:
                continue

            cart.pos += cart.facing

            for cart2 in carts:
                if cart != cart2 and cart2.alive and cart.pos == cart2.pos:
                    cart.alive = False
                    cart2.alive = False
                    break

            if cart.alive and cart.pos in POIs:
                cart.rotate(POIs[cart.pos])

        if False in [c.alive for c in carts]:
            for c in carts:
                if not c.alive:
                    collision = c.pos
                    break

            return ','.join(str(c) for c in (int(collision.real), int(collision.imag)))


def part2(data):
    """ 2018 Day 13 Part 2
    """

    carts = []
    tracks = {}
    POIs = {}
    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l in '<>^v':
                carts.append(Cart(complex(x, y), l))
                tracks[complex(x, y)] = '-' if l in '<>' else '|'
            elif l in '/\+':
                POIs[complex(x, y)] = POI(complex(x, y), l)
            elif l in '-|':
                tracks[complex(x, y)] = l

    firstFound = False
    while len(carts) > 1:
        carts.sort()
        for cart in carts:
            if not cart.alive:
                continue

            cart.pos += cart.facing

            for cart2 in carts:
                if cart != cart2 and cart2.alive and cart.pos == cart2.pos:
                    cart.alive = False
                    cart2.alive = False
                    break

            if cart.alive and cart.pos in POIs:
                cart.rotate(POIs[cart.pos])

        if False in [c.alive for c in carts]:
            carts = [c for c in carts if c.alive]

    return ','.join(str(c) for c in (int(carts[0].pos.real), int(carts[0].pos.imag)))


class Cart:
    def __init__(self, pos, char):
        self.pos = pos
        self.facing = {'>': complex(1), '<': complex(-1), 'v': complex(0, 1), '^': complex(0, -1)}[char]
        self.nextTurn = 0 # 0 for left, 1 for straight, 2 for right
        self.alive = True

    def __lt__(self, other):
        return (self.pos.imag < other.pos.imag) ^ (self.pos.real < other.pos.real)

    def rotate(self, POI):
        # Multiplying by j turns clockwise, multiplying by -j turns counterclockwise
        if POI.char == '+':
            if self.nextTurn % 2 == 0:
                self.facing *= complex(0, -1 if self.nextTurn == 0 else 1)

            self.nextTurn += 1
            self.nextTurn %= 3
        else:
            if (self.facing.imag == 0 and POI.char == '\\') or (self.facing.real == 0 and POI.char == '/'):
                self.facing *= complex(0, 1)
            else:
                self.facing *= complex(0, -1)


class POI:
    def __init__(self, pos, char):
        self.pos = pos
        self.char = char


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
        print(f"\nPart 1:\nFirst collision occurs at position: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nPosition of final cart: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)