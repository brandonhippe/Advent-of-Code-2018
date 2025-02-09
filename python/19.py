import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2018 Day 19 Part 1
    """

    program = [line.split(' ') for line in data]

    bound = int(program[0][1])
    program = program[1:]

    for line in program:
        for i, s in enumerate(line):
            try:
                line[i] = int(s)
            except ValueError:
                pass

    registers = [0] * 6
    while registers[bound] != 1:
        registers = OPERATIONS[program[registers[bound]][0]](registers, program[registers[bound]])
        registers[bound] += 1

    return sum(factor(registers[5]))


def part2(data):
    """ 2019 Day 19 Part 2
    """

    program = [line.split(' ') for line in data]

    bound = int(program[0][1])
    program = program[1:]

    for line in program:
        for i, s in enumerate(line):
            try:
                line[i] = int(s)
            except ValueError:
                pass

    registers = [0] * 6
    registers[0] = 1
    while registers[bound] != 1:
        registers = OPERATIONS[program[registers[bound]][0]](registers, program[registers[bound]])
        registers[bound] += 1

    return sum(factor(registers[5]))


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
    out[op[-1]] = 1 if op[1] > reg[op[2]] else 0
    return out


def gtri(reg, op):
    out = reg[:]
    out[op[-1]] = 1 if reg[op[1]] > op[2] else 0
    return out


def gtrr(reg, op):
    out = reg[:]
    out[op[-1]] = 1 if reg[op[1]] > reg[op[2]] else 0
    return out


def eqir(reg, op):
    out = reg[:]
    out[op[-1]] = 1 if op[1] == reg[op[2]] else 0
    return out


def eqri(reg, op):
    out = reg[:]
    out[op[-1]] = 1 if reg[op[1]] == op[2] else 0
    return out


def eqrr(reg, op):
    out = reg[:]
    out[op[-1]] = 1 if reg[op[1]] == reg[op[2]] else 0
    return out


OPERATIONS = {  
    'addr': addr, 
    'addi': addi,
    'mulr': mulr, 
    'muli': muli,
    'banr': banr,
    'bani': bani,
    'borr': borr,
    'bori': bori,
    'setr': setr,
    'seti': seti,
    'gtir': gtir,
    'gtri': gtri,
    'gtrr': gtrr,
    'eqir': eqir,
    'eqri': eqri,
    'eqrr': eqrr
}


def factor(num):
    factors = []

    largestFound = 1
    i = 1
    while i < num // largestFound:
        if num % i == 0:
            factors.append(i)
            factors.append(num // i)
            largestFound = i

        i += 1

    return factors


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
        print(f"\nPart 1:\nValue left in register 0 after process halts: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nValue left in register 0 after process halts: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)