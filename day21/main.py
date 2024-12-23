import fileinput
from frozendict import frozendict
from collections import deque
from functools import cache
import itertools

KEYPAD = {
    (0, 0): "7",
    (0, 1): "8",
    (0, 2): "9",
    (1, 0): "4",
    (1, 1): "5",
    (1, 2): "6",
    (2, 0): "1",
    (2, 1): "2",
    (2, 2): "3",
    (3, 1): "0",
    (3, 2): "A",
}

DIRPAD = {
    (0, 1): "^",
    (0, 2): "A",
    (1, 0): "<",
    (1, 1): "v",
    (1, 2): ">",
}


def neighbors(pos, grid):
    y, x = pos
    for (dy, dx), move in (
        ((-1, 0), "^"),
        ((0, 1), ">"),
        ((1, 0), "v"),
        ((0, -1), "<"),
    ):
        n = (y + dy, x + dx)
        if n in grid:
            yield n, move


def all_shortest_paths(_from, to, grid):
    paths = []
    q = deque([(_from, [], {_from})])
    while q:
        _from, path, visited = q.popleft()
        if _from == to:
            if len(paths) == 0 or len(path) == len(paths[0]):
                paths.append(path)
            elif len(paths) > len(paths[0]):
                break
        for neighbor, move in neighbors(_from, grid):
            if neighbor in visited:
                continue
            q.append((neighbor, path + [move], visited | {neighbor}))

    return paths


def all_moves(_from, rest, paths):
    @cache
    def cacheable(_from, rest):
        if not rest:
            return [[]]

        head, *tail = rest
        subpaths = cacheable(head, tuple(tail))
        res = []
        for path in paths[(_from, head)]:
            for subpath in subpaths:
                res.append(path + ["A"] + subpath)
        return res

    return cacheable(_from, rest)


def find_shortest(_from, to, level, paths):
    @cache
    def find_dp(_from, to, level):
        if level == 0:
            return 1

        shortest = float("Inf")
        for moves in all_moves(_from, (to,), paths):
            total = 0
            for f1, t1 in itertools.pairwise(["A"] + moves):
                total += find_dp(f1, t1, level - 1)
            shortest = min(total, shortest)
        return shortest

    return find_dp(_from, to, level)


def main():
    KEYPAD_PATHS = frozendict(
        {
            (KEYPAD[p1], KEYPAD[p2]): all_shortest_paths(p1, p2, KEYPAD)
            for p1 in KEYPAD
            for p2 in KEYPAD
        }
    )

    DIRPAD_PATHS = frozendict(
        {
            (DIRPAD[p1], DIRPAD[p2]): all_shortest_paths(p1, p2, DIRPAD)
            for p1 in DIRPAD
            for p2 in DIRPAD
        }
    )

    def run_times(times):
        res = 0
        for digits in fileinput.input():
            moves = all_moves("A", digits.strip(), KEYPAD_PATHS)
            shortest = float("Inf")
            for move in moves:
                total = 0
                for _from, to in itertools.pairwise(["A"] + move):
                    total += find_shortest(_from, to, times, DIRPAD_PATHS)
                shortest = min(total, shortest)
            res += shortest * int(digits.strip()[:-1])
        return res

    print(run_times(2))
    print(run_times(25))


main()
