import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import copy


def part1(data):
    """ 2018 Day 15 Part 1

    >>> part1(['#######', '#G..#E#', '#E#E.E#', '#G.##.#', '#...#E#', '#...E.#', '#######'])
    36334
    >>> part1(['#########', '#G......#', '#.E.#...#', '#..##..G#', '#...##..#', '#...#...#', '#.G...G.#', '#.....G.#', '#########'])
    18740
    """

    walls = []
    elves = []
    goblins = []
    maxs = [0, 0]
    for y, line in enumerate(data):
        if y > maxs[1]:
            maxs[1] = y
        for x, l in enumerate(line):
            if x > maxs[0]:
                maxs[0] = x

            if l == '#':
                walls.append([x, y])
            elif l == 'E':
                elves.append(Unit([x, y]))
            elif l == 'G':
                goblins.append(Unit([x, y]))
    
    rounds = 0
    while len(elves) > 0 and len(goblins) > 0:
        if iterate(walls, elves, goblins):
            rounds += 1
    
    return rounds * sum(w.hp for w in (elves if len(goblins) == 0 else goblins))



def part2(data):
    """ 2018 Day 15 Part 2

    >>> part2(['#########', '#G......#', '#.E.#...#', '#..##..G#', '#...##..#', '#...#...#', '#.G...G.#', '#.....G.#', '#########'])
    1140
    """

    walls = []
    elves = []
    goblins = []
    maxs = [0, 0]
    for y, line in enumerate(data):
        if y > maxs[1]:
            maxs[1] = y
        for x, l in enumerate(line):
            if x > maxs[0]:
                maxs[0] = x

            if l == '#':
                walls.append([x, y])
            elif l == 'E':
                elves.append(Unit([x, y]))
            elif l == 'G':
                goblins.append(Unit([x, y]))
    
    iteratingElves = []
    totalElves = len(elves)
    elfAttack = 3
    while len(iteratingElves) < totalElves:
        elfAttack += 1
        iteratingElves = copy.deepcopy(elves)
        iteratingGoblins = copy.deepcopy(goblins)
        
        for e in iteratingElves:
            e.attack = elfAttack

        rounds = 0
        while len(iteratingElves) == totalElves and len(iteratingGoblins) > 0:
            if iterate(walls, iteratingElves, iteratingGoblins):
                rounds += 1
    
    return rounds * sum(w.hp for w in (iteratingElves if len(iteratingGoblins) == 0 else iteratingGoblins))


READING_ORDER = [[0, -1], [-1, 0], [1, 0], [0, 1]]


class Unit:
    def __init__(self, pos):
        self.hp = 200
        self.attack = 3
        self.pos = pos

    def __lt__(self, other):
        return self.pos[1] < other.pos[1] or (self.pos[1] == other.pos[1] and self.pos[0] < other.pos[0])


class SolvePath:
    def __init__(self, path):
        self.path = path[:]
        self.pos = self.path[-1]
        self.d = len(self.path) - 1

    def __lt__(self, other):
        if self.d < other.d:
            return True

        if self.d > other.d:
            return False

        return self.path[1][1] < other.path[1][1] or (self.path[1][1] == other.path[1][1] and self.path[1][0] < other.path[1][0])


def manhatDist(p1, p2):
    return sum(abs(c1 - c2) for c1, c2 in zip(p1, p2))


def findNearest(unit, allies, enemies, walls):
    visited = []
    inQueue = [SolvePath([unit.pos])]

    while len(inQueue) != 0:
        currPath = inQueue.pop(0)
        
        if currPath.pos in [e.pos for e in enemies]:
            return [enemies[[e.pos for e in enemies].index(currPath.pos)], currPath.path[1]]

        for n in [[p + o for p, o in zip(currPath.pos, offset)] for offset in READING_ORDER]:
            if n in visited or n in walls or n in [q.pos for q in inQueue] or n in [a.pos for a in allies]:
                continue

            inQueue.append(SolvePath(currPath.path + [n]))

        visited.append(currPath.pos)

    return [None] * 2


def iterate(walls, elves, goblins):
    complete = True
    for x, y in [u.pos for u in sorted(elves + goblins)]:            
        if [x, y] in [e.pos for e in elves]:
            unit = elves[[e.pos for e in elves].index([x, y])]
            allies = elves
            enemies = goblins
        elif [x, y] in [g.pos for g in goblins]:
            unit = goblins[[g.pos for g in goblins].index([x, y])]
            allies = goblins
            enemies = elves
        else:
            continue

        if len(enemies) == 0:
            complete = False

        nearestEnemy, nextPos = findNearest(unit, allies, enemies, walls)

        if nearestEnemy is None:
            continue

        if nearestEnemy.pos != nextPos:
            # Moving
            unit.pos = nextPos[:]
        
        if manhatDist(unit.pos, nearestEnemy.pos) == 1:
            # Attacking
            nearestEnemy = None
            for n in [[p + o for p, o in zip(unit.pos, offset)] for offset in READING_ORDER]:
                try:
                    enemy = enemies[[e.pos for e in enemies].index(n)]
                    if nearestEnemy is None or enemy.hp < nearestEnemy.hp:
                        nearestEnemy = enemy
                except ValueError:
                    pass

            nearestEnemy.hp -= unit.attack
            if nearestEnemy.hp <= 0:
                enemies.pop(enemies.index(nearestEnemy))

    return complete

            
def printBattle(maxs, walls, elves, goblins):
    for y in range(0, maxs[1] + 1):
        postStr = '   '
        for x in range(0, maxs[0] + 1):
            c = '.'
            if [x, y] in walls:
                c = '#'
            elif [x, y] in [e.pos for e in elves]:
                c = 'E'
                postStr += f'E({elves[[e.pos for e in elves].index([x, y])].hp}), '
            elif [x, y] in [g.pos for g in goblins]:
                c = 'G'
                postStr += f'G({goblins[[g.pos for g in goblins].index([x, y])].hp}), '

            print(c, end='')
        print(f'{postStr[:-2]}')


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
        print(f"\nPart 1:\nCombat Outcome: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nCombat Outcome: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)