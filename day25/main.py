import fileinput


def parse_blk(blk):
    heights = [0] * 5
    for l in blk.split("\n"):
        for i, c in enumerate(l):
            heights[i] += 1 if c == "#" else 0
    is_key = all(c == "#" for c in l)
    return is_key, tuple(heights)


def main():
    blks = ("".join(fileinput.input())).split("\n\n")
    locks = []
    keys = []
    for blk in blks:
        is_key, heights = parse_blk(blk)
        if is_key:
            keys.append(heights)
        else:
            locks.append(heights)

    combs = sum(1 for k in keys for l in locks if 
        all(k[i] + l[i] <= 7 for i in range(5))
    )
    print(combs)


main()
