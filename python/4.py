import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from datetime import datetime
from collections import defaultdict


def part1(data):
    """ 2018 Day 4 Part 1

    >>> part1(['[1518-11-01 00:00] Guard #10 begins shift', '[1518-11-01 00:05] falls asleep', '[1518-11-01 00:25] wakes up', '[1518-11-01 00:30] falls asleep', '[1518-11-01 00:55] wakes up', '[1518-11-01 23:58] Guard #99 begins shift', '[1518-11-02 00:40] falls asleep', '[1518-11-02 00:50] wakes up', '[1518-11-03 00:05] Guard #10 begins shift', '[1518-11-03 00:24] falls asleep', '[1518-11-03 00:29] wakes up', '[1518-11-04 00:02] Guard #99 begins shift', '[1518-11-04 00:36] falls asleep', '[1518-11-04 00:46] wakes up', '[1518-11-05 00:03] Guard #99 begins shift', '[1518-11-05 00:45] falls asleep', '[1518-11-05 00:55] wakes up'])
    240
    """

    events = [event(line) for line in data]
    events.sort()
    
    guards = defaultdict(lambda: [defaultdict(lambda: 0), 0])
    currGuard = None
    asleepTime = 0
    for e in events:
        if 'shift' in e.eventStr:
            currGuard = int(re.findall(r'\d+', e.eventStr)[0])
        elif 'falls asleep' == e.eventStr:
            asleepTime = e.time.minute
        elif 'wakes up' == e.eventStr:
            for m in range(asleepTime, e.time.minute):
                guards[currGuard][0][m] += 1
                guards[currGuard][1] += 1
                
    sleepingGuard = [None, [float('-inf')]]
    for guardNum, guard in zip(guards.keys(), guards.values()):
        if guard[1] > sleepingGuard[-1][-1]:
            sleepingGuard = [guardNum, guard]

    minuteAsleep = 0
    for m, amt in zip(sleepingGuard[1][0].keys(), sleepingGuard[1][0].values()):
        if minuteAsleep not in sleepingGuard[1][0] or amt > sleepingGuard[1][0][minuteAsleep]:
            minuteAsleep = m

    return sleepingGuard[0] * minuteAsleep


def part2(data):
    """ 2018 Day 4 Part 2

    >>> part2(['[1518-11-01 00:00] Guard #10 begins shift', '[1518-11-01 00:05] falls asleep', '[1518-11-01 00:25] wakes up', '[1518-11-01 00:30] falls asleep', '[1518-11-01 00:55] wakes up', '[1518-11-01 23:58] Guard #99 begins shift', '[1518-11-02 00:40] falls asleep', '[1518-11-02 00:50] wakes up', '[1518-11-03 00:05] Guard #10 begins shift', '[1518-11-03 00:24] falls asleep', '[1518-11-03 00:29] wakes up', '[1518-11-04 00:02] Guard #99 begins shift', '[1518-11-04 00:36] falls asleep', '[1518-11-04 00:46] wakes up', '[1518-11-05 00:03] Guard #99 begins shift', '[1518-11-05 00:45] falls asleep', '[1518-11-05 00:55] wakes up'])
    4455
    """

    events = [event(line) for line in data]
    events.sort()
    
    guards = defaultdict(lambda: [defaultdict(lambda: 0), 0])
    currGuard = None
    asleepTime = 0
    for e in events:
        if 'shift' in e.eventStr:
            currGuard = int(re.findall(r'\d+', e.eventStr)[0])
        elif 'falls asleep' == e.eventStr:
            asleepTime = e.time.minute
        elif 'wakes up' == e.eventStr:
            for m in range(asleepTime, e.time.minute):
                guards[currGuard][0][m] += 1
                guards[currGuard][1] += 1

    minuteAsleep = [0, 0, 0] # Guard, minute, number of times asleep
    for guardNum, guard in zip(guards.keys(), guards.values()):
        for minute, times in zip(guard[0].keys(), guard[0].values()):
            if minuteAsleep[0] not in guards or minuteAsleep[1] not in guards[minuteAsleep[0]][0] or times > minuteAsleep[2]:
                minuteAsleep = [guardNum, minute, times]

    return minuteAsleep[0] * minuteAsleep[1]


class event:
    def __init__(self, eventLine):
        self.time = datetime.strptime(eventLine.split(']')[0][1:], '%Y-%m-%d %H:%M')
        self.eventStr = eventLine.split(']')[1][1:]

    def __lt__(self, other):
        return self.time < other.time


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
        print(f"\nPart 1:\nGuard ID * minute: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nGuard ID * minute: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)