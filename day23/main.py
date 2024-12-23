import fileinput
import itertools
from collections import defaultdict, deque, Counter


def find_groups(children_by_parent):
    groups = set()
    q = deque([(n, frozenset()) for n in children_by_parent])
    while q:
        n, visited = q.popleft()
        if n in visited or len(visited) > 2:
            continue
        if len(visited) == 2:
            n1, n2 = visited
            if n in children_by_parent[n1] and n in children_by_parent[n2]:
                groups.add(visited | {n})
            continue

        for child in children_by_parent[n]:
            q.append((child, visited | {n}))
    return groups


def find_largest_group(children_by_parent):
    largest = set()
    for n in children_by_parent:
        clique = set([n])
        # Try to grow.
        for nn in children_by_parent:
            if all(nn in children_by_parent[c] for c in clique):
                clique.add(nn)

        if len(clique) > len(largest):
            largest = clique
    return largest


def main():
    edges = [tuple(e.strip().split("-")) for e in fileinput.input()]
    children_by_parent = defaultdict(set)
    for a, b in edges:
        children_by_parent[a].add(b)
        children_by_parent[b].add(a)

    groups = find_groups(children_by_parent)
    print(sum(1 for g in groups if any(n.startswith("t") for n in g)))

    group = find_largest_group(children_by_parent)
    print(",".join(sorted(group)))


main()
