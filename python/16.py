import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2018 Day 16 Part 1

    >>> part1(['Before: [3, 2, 1, 1]', '9 2 1 2', 'After:  [3, 2, 2, 1]', '', ''])
    1
    """

    observations = []
    i = 0
    while len(data[i]) > 0:
        before, op, after, _ = data[i:i + 4]
        observations.append(Observation([int(n) for n in re.findall(r'\d+', before)], [int(n) for n in re.findall(r'\d+', op)], [int(n) for n in re.findall(r'\d+', after)]))
        i += 4

    opCodes = {o: OPERATIONS[:] for o in set(ob.op[0] for ob in observations)}

    for o in observations:
        for fun in OPERATIONS:
            if fun(o.prevReg, o.op) == o.postReg:
                o.behavesLike += 1
            elif fun in opCodes[o.op[0]]:
                opCodes[o.op[0]].pop(opCodes[o.op[0]].index(fun))

    return len([o for o in observations if o.behavesLike >= 3])


def part2(data):
    """ 2018 Day 16 Part 2
    """

    observations = []
    i = 0
    while len(data[i]) > 0:
        before, op, after, _ = data[i:i + 4]
        observations.append(Observation([int(n) for n in re.findall(r'\d+', before)], [int(n) for n in re.findall(r'\d+', op)], [int(n) for n in re.findall(r'\d+', after)]))
        i += 4

    program = [[int(n) for n in re.findall(r'\d+', line)] for line in data[i + 2:]]

    opCodes = {o: OPERATIONS[:] for o in set(ob.op[0] for ob in observations)}

    for o in observations:
        for fun in OPERATIONS:
            if fun(o.prevReg, o.op) == o.postReg:
                o.behavesLike += 1
            elif fun in opCodes[o.op[0]]:
                opCodes[o.op[0]].pop(opCodes[o.op[0]].index(fun))

    reduceOps(opCodes)
    reg = [0] * 4
    for op in program:
        reg = opCodes[op[0]][0](reg, op)

    return reg[0]


class Observation:
    def __init__(self, before, op, after) -> None:
        self.prevReg = before
        self.op = op
        self.postReg = after
        self.behavesLike = 0


def addr(reg, op):
    out = reg[:]
    out[op[-1]] = reg[op[1]] + reg[op[2]]
    return out


def addi(reg, op):
    out = reg[:]
    out[op[-1]] = reg[op[1]] + op[2]
    return out


def mulr(reg, op):
    out = reg[:]
    out[op[-1]] = reg[op[1]] * reg[op[2]]
    return out


def muli(reg, op):
    out = reg[:]
    out[op[-1]] = reg[op[1]] * op[2]
    return out


def banr(reg, op):
    out = reg[:]
    out[op[-1]] = reg[op[1]] & reg[op[2]]
    return out


def bani(reg, op):
    out = reg[:]
    out[op[-1]] = reg[op[1]] & op[2]
    return out


def borr(reg, op):
    out = reg[:]
    out[op[-1]] = reg[op[1]] | reg[op[2]]
    return out


def bori(reg, op):
    out = reg[:]
    out[op[-1]] = reg[op[1]] | op[2]
    return out


def setr(reg, op):
    out = reg[:]
    out[op[-1]] = reg[op[1]]
    return out


def seti(reg, op):
    out = reg[:]
    out[op[-1]] = op[1]
    return out


def gtir(reg, op):
    out = reg[:]
    out[op[-1]] = int(op[1] > reg[op[2]])
    return out


def gtri(reg, op):
    out = reg[:]
    out[op[-1]] = int(reg[op[1]] > op[2])
    return out


def gtrr(reg, op):
    out = reg[:]
    out[op[-1]] = int(reg[op[1]] >= reg[op[2]])
    return out


def eqir(reg, op):
    out = reg[:]
    out[op[-1]] = int(op[1] == reg[op[2]])
    return out


def eqri(reg, op):
    out = reg[:]
    out[op[-1]] = int(reg[op[1]] == op[2])
    return out


def eqrr(reg, op):
    out = reg[:]
    out[op[-1]] = int(reg[op[1]] == reg[op[2]])
    return out


def reduceOps(opCodes):
    while max(len(o) for o in opCodes.values()) > 1:
        funPos = {fun: [o for o in opCodes.keys() if fun in opCodes[o]] for fun in OPERATIONS}

        for fun, pos in zip(funPos.keys(), funPos.values()):
            if len(pos) == 1:
                opCodes[pos[0]] = [fun]

        for op, funs in zip(opCodes.keys(), opCodes.values()):
            if len(funs) == 1:
                for op2, funs2 in zip(opCodes.keys(), opCodes.values()):
                    if op == op2:
                        continue

                    if funs[0] in funs2:
                        opCodes[op2].pop(opCodes[op2].index(funs[0]))


OPERATIONS = [addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr]


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
        print(f"\nPart 1:\nNumber of observations that behave like 3 or more opcodes: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nValue in register 0 after program executes: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)