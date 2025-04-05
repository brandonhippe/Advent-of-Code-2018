import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2018 Day 20 Part 1

    >>> part1(['^WNE$'])
    3
    >>> part1(['^ENWWW(NEEE|SSE(EE|N))$'])
    10
    >>> part1(['^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$'])
    18
    >>> part1(['^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$'])
    23
    >>> part1(['^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$'])
    31
    """

    directions = Direction(data[0][1:-1])

    directions.genRooms()
    doors = directions.doors

    return farthestRoom(doors, (0, 0))[0]


def part2(data):
    """ 2018 Day 20 Part 2
    """

    directions = Direction(data[0][1:-1])

    directions.genRooms()
    doors = directions.doors

    return farthestRoom(doors, (0, 0))[1]


MOVES = {'N': (0, -1), 'E': (1, 0), 'S': (0, 1), 'W': (-1, 0)}


class Direction:
    def __init__(self, directionText):
        self.rooms = set()
        self.doors = set()
        self.directionText = directionText
        self.initialDirs = None
        self.dirChoices = []
        self.postDirs = None
        self.ends = {(0, 0)}

    def genRooms(self):
        if len(self.directionText) == 0:
            return

        parens = re.search(r'\(', self.directionText)

        if parens is None:
            for d in self.directionText:
                newEnds = set()
                for pos in self.ends:
                    self.rooms.add(pos)
                    self.doors.add(tuple(p + o for p, o in zip(pos, MOVES[d])))
                    newEnds.add(tuple(p + (2 * o) for p, o in zip(pos, MOVES[d])))

                self.ends = newEnds

            for pos in self.ends:
                self.rooms.add(pos)

            return

        start = parens.span()[1]
        self.initialDirs = Direction(self.directionText[:start - 1])
        self.initialDirs.genRooms()
        self.rooms, self.doors, self.ends = self.initialDirs.rooms, self.initialDirs.doors, self.initialDirs.ends

        end = start - 1
        opened = 1
        optionsText = [""]
        while True:
            end += 1
            d = self.directionText[end]
            if d == '(':
                opened += 1
            elif d == ')':
                opened -= 1

            if opened == 0:
                break

            if d == '|' and opened == 1:
                optionsText.append("")
            else:
                optionsText[-1] += d


        for option in optionsText:
            self.dirChoices.append(Direction(option))
            self.dirChoices[-1].genRooms()

        newEnds = set()
        for pos in self.ends:
            for choice in self.dirChoices:
                rooms, doors, ends = choice.rooms, choice.doors, choice.ends

                for r in rooms:
                    self.rooms.add(tuple(p + o for p, o in zip(pos, r)))

                for d in doors:
                    self.doors.add(tuple(p + o for p, o in zip(pos, d)))

                for e in ends:
                    newEnds.add(tuple(p + o for p, o in zip(pos, e)))

        self.ends = newEnds

        self.postDirs = Direction(self.directionText[end + 1:])
        self.postDirs.genRooms()
        
        newEnds = set()
        rooms, doors, ends = self.postDirs.rooms, self.postDirs.doors, self.postDirs.ends
        for pos in self.ends:
            for r in rooms:
                self.rooms.add(tuple(p + o for p, o in zip(pos, r)))

            for d in doors:
                self.doors.add(tuple(p + o for p, o in zip(pos, d)))

            for e in ends:
                newEnds.add(tuple(p + o for p, o in zip(pos, e)))

        self.ends = newEnds

    def posOffset(self, pos):
        roomsOffset = set()
        doorsOffset = set()

        for r in self.rooms:
            roomsOffset.add(tuple(p + o for p, o in zip(pos, r)))

        for d in self.doors:
            doorsOffset.add(tuple(p + o for p, o in zip(pos, d)))

        return [roomsOffset, doorsOffset]


def farthestRoom(doors, pos):
    visited = {}
    openList = [[pos, 0]]

    while len(openList) != 0:
        pos, d = openList.pop(0)

        for offset in MOVES.values():
            n = tuple(p + o for p, o in zip(pos, offset))
            n2 = tuple(p + (2 * o) for p, o in zip(pos, offset))
            if n in doors and n2 not in visited:
                openList.append([n2, d + 1])

        visited[pos] = d

    return [max(visited.values()), len([v for v in visited.values() if v >= 1000])]


def printRooms(rooms, doors, pos):
    minX = min(p[0] for p in rooms.union(doors))
    minY = min(p[1] for p in rooms.union(doors))
    maxX = max(p[0] for p in rooms.union(doors))
    maxY = max(p[1] for p in rooms.union(doors))

    string = ''
    for y in range(minY - 1, maxY + 2):
        string += '\n'
        for x in range(minX - 1, maxX + 2):
            c = '#'

            if (x, y) == pos:
                c = '@'
            elif (x, y) in rooms:
                c = ' '
            elif (x, y) in doors:
                c = '.'

            string += c

    return string[1:]


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
        print(f"\nPart 1:\nFarthest room from start: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of rooms over 1000 doors away: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)