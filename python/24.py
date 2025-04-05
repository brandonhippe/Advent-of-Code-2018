import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
import copy


def part1(data):
    """ 2018 Day 24 Part 1

    >>> part1(['Immune System:', '17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2', '989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3', '', 'Infection:', '801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1', '4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4'])
    5216
    """

    groups = []
    for line in data:
        if len(line) == 0:
            continue

        if line[-1] == ':':
            army = line[:-1]
        else:
            groups.append(Group(army, line))

    return sum([g.units for g in fight(groups)])


def part2(data):
    """ 2018 Day 24 Part 2

    >>> part2(['Immune System:', '17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2', '989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3', '', 'Infection:', '801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1', '4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4'])
    51
    """

    groups = []
    for line in data:
        if len(line) == 0:
            continue

        if line[-1] == ':':
            army = line[:-1]
        else:
            groups.append(Group(army, line))

    immune = None
    infection = 0
    boost = 1
    while immune is None or immune - infection > 1:
        gs = copy.deepcopy(groups)
        for g in gs:
            if g.army == "Immune System":
                g.damage += boost

        gs = fight(gs)

        if len(set(g.army for g in gs)) == 1 and gs[0].army == 'Immune System':
            if immune is None or boost < immune:
                immune = boost
        elif boost > infection:
            infection = boost

        if immune is None:
            boost *= 2
        else:
            boost = (immune + infection) // 2

    gs = copy.deepcopy(groups)
    for g in gs:
        if g.army == "Immune System":
            g.damage += immune

    return sum([g.units for g in fight(gs)])


class Group:
    def __init__(self, army, groupText):
        self.army = army
        self.units, self.hp, self.damage, self.initiative = (int(x) for x in re.findall(r'\d+', groupText))
        self.damageType = groupText.split(f'{self.damage} ')[-1].split(' damage')[0]

        self.weak = []
        self.immune = []

        if '(' in groupText:
            specials = re.split(' to |; ', re.findall(r'\(.*\)', groupText)[0][1:-1])
            for i, s in enumerate(specials):
                if s not in ['weak', 'immune']:
                    if specials[i - 1] == 'weak':
                        self.weak = s.split(', ')
                    else:
                        self.immune = s.split(', ')

        self.attacking = None
        self.defending = None

    def __lt__(self, other):
        return self.units * self.damage < other.units * other.damage or (self.units * self.damage == other.units * other.damage and self.initiative < other.initiative)

    def damageDealt(self, other):
        return self.units * self.damage * (0 if self.damageType in other.immune else 1) * (2 if self.damageType in other.weak else 1)

    def takeDamage(self):
        if self.defending.units > 0:
            dam = self.defending.damageDealt(self)
            self.units -= dam // self.hp


def targetPhase(groups):
    groups = sorted(groups, reverse=True)
    
    for g in groups:
        most = 0
        chosen = None
        for gr in groups:
            if gr.defending is None and gr.army != g.army:
                dam = g.damageDealt(gr)
                if dam > most or (dam == most and (chosen is None or (gr.units * gr.damage > chosen.units * chosen.damage or (gr.units * gr.damage == chosen.units * chosen.damage and gr.initiative > chosen.initiative)))):
                    most = g.damageDealt(gr)
                    chosen = gr

        if most > 0:
            g.attacking = chosen
            chosen.defending = g


def attackingPhase(groups):
    groups = sorted(groups, reverse=True, key=lambda g: g.initiative)

    newGroups = []
    for g in groups:
        if g.attacking is not None:
            g.attacking.takeDamage()
            if g.attacking.units > 0:
                newGroups.append(g.attacking)

        if g.defending is None:
            newGroups.append(g)

    return newGroups


def fight(groups):
    remainingUnits = set()
    while len({g.army for g in groups}) != 1 and tuple(g.units for g in groups) not in remainingUnits:
        remainingUnits.add(tuple(g.units for g in groups))
        targetPhase(groups)
        groups = attackingPhase(groups)
        for g in groups:
            g.attacking = None
            g.defending = None

    return groups


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
        print(f"\nPart 1:\nTotal remaining units of winning army: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTotal remaining immune system units with smallest boost: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)