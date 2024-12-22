import fileinput
import itertools
from collections import defaultdict


def gen(n):
    while True:
        # yield mp(2048 * mp())
        n = ((n * 64) ^ n) % 16777216
        n = ((n // 32) ^ n) % 16777216
        n = ((n * 2048) ^ n) % 16777216
        yield n


def gen_changes(n, times):
    prev = n
    for i, n in enumerate(gen(n)):
        yield (n % 10, (n % 10) - (prev % 10))
        prev = n
        if i == times - 1:
            break


"""
1: 8685429
10: 4700978
100: 15273692
2024: 8667524
"""


def run_for(n, times):
    for i, n in enumerate(gen(n)):
        if i == times - 1:
            return n


def gen_4changes_price(n, times):
    changes = tuple()
    for n, change in gen_changes(n, times):
        if len(changes) == 4:
            changes = changes[1:] + (change,)
        else:
            changes = changes + (change,)

        if len(changes) == 4:
            yield changes, n


def main():
    n = 1
    ns = [int(n) for n in fileinput.input()]

    print(sum(run_for(n, 2000) for n in ns))

    all_prices_by_seq = defaultdict(int)
    for n in ns:
        prices_by_seq = {}
        for changes, price in gen_4changes_price(n, 2000):
            if changes not in prices_by_seq:
                prices_by_seq[changes] = price
        for seq, price in prices_by_seq.items():
            all_prices_by_seq[seq] += price

    print(max(all_prices_by_seq.items(), key=lambda kv: kv[1])[1])


main()
