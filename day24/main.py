import fileinput
import itertools
from functools import cache
from frozendict import frozendict


@cache
def eval(out, vals, exprs):
    if out in vals:
        return vals[out]
    lhs, op, rhs = exprs[out]
    l = eval(lhs, vals, exprs)
    r = eval(rhs, vals, exprs)
    match op:
        case "AND":
            return l & r
        case "XOR":
            return l ^ r
        case "OR":
            return l | r
        case _:
            raise RuntimeError()


def collect_operands(out, exprs):
    if out.startswith("x") or out.startswith("y"):
        return out
    lhs, op, rhs = exprs[out]
    match op:
        case "AND":
            op = "&"
        case "XOR":
            op = "^"
        case "OR":
            op = "|"
    return f"({collect_operands(lhs, exprs)}) {op} ({collect_operands(rhs, exprs)})"


def find_output(a, op, b, exprs):
    try:
        [es] = [
            out
            for out, (ea, eop, eb) in exprs.items()
            if eop == op and ((ea == a and eb == b) or (ea == b and eb == a))
        ]
        return es
    except ValueError:
        print(f"Could not find ops {a} {op} {b}")


def find_twisted_pairs(exprs):
    # Found by inspection.
    carry = "gmk"
    for i in range(1, 45):
        # print(f"Trying {i=}, {carry=}")
        n = str(i).zfill(2)
        x, y, z = "x" + n, "y" + n, "z" + n
        xor1 = find_output(x, "XOR", y, exprs)
        and1 = find_output(x, "AND", y, exprs)
        and2 = find_output(carry, "AND", xor1, exprs)
        xor2 = find_output(carry, "XOR", xor1, exprs)
        or1 = find_output(and2, "OR", and1, exprs)
        assert xor2 == z, xor2
        carry = or1
        # print(f"Found {i=} {carry=} {xor1=} {and1=} {and2=} {xor2=} {or1=}")


def main():
    blk0, blk1 = "".join(fileinput.input()).split("\n\n")
    vals = {}
    for l in blk0.split("\n"):
        k, v = l.split(": ")
        vals[k] = int(v)
    vals = frozendict(vals)
    exprs = {}
    for l in blk1.split("\n"):
        lhs, op, rhs, _, out = l.split(" ")
        exprs[out] = (lhs, op, rhs)
    exprs = frozendict(exprs)

    zs = list(sorted(o for o in exprs if o.startswith("z")))
    pt1 = sum(eval(o, vals, exprs) << i for i, o in enumerate(zs))
    print(pt1)

    # My strategy: once I figured out we're dealing with a full adder circuit,
    # I set expectations on each internal signal. Once the code breaks, I manually
    # do the broken iteration by hand and figure out which outputs are swapped.
    # Luckily, outputs are swapped inside or a single iteration. Once I find the swap,
    # I correct them manually here and repeat.
    swaps = {
        "z07": "gmt",
        "gmt": "z07",
        "qjj": "cbj",
        "cbj": "qjj",
        "dmn": "z18",
        "z18": "dmn",
        "cfk": "z35",
        "z35": "cfk",
    }
    exprs = frozendict({swaps.get(o, o): expr for o, expr in exprs.items()})
    find_twisted_pairs(exprs)

    print(",".join(sorted(set(swaps.keys()))))


main()
