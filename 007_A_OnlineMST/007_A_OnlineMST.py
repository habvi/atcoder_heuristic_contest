from collections import defaultdict
from decimal import Decimal, ROUND_HALF_UP
from heapq import heappop, heappush

N = 400
M = 1995

v_xy = []
for _ in range(N):
    x, y = map(int, input().split())
    v_xy.append((x, y))

G = defaultdict(list)
edge_dist = []
edge_endpt = []
edge_dist_idx = []
cnt = 1

for i in range(M):
    u, v = map(int, input().split())
    G[u].append(v)
    G[v].append(u)
    x1, y1 = v_xy[u]
    x2, y2 = v_xy[v]

    edge_endpt.append((u, v))

    d = ((x1 - x2)**2 + (y1 - y2)**2) ** Decimal('0.5')
    round_d = int(Decimal(str(d)).quantize(Decimal('0'), rounding=ROUND_HALF_UP))
    round_3d = round_d * 3
    edge_dist.append((round_d, round_3d))

    if cnt % 8 == 0:
        heappush(edge_dist_idx, (round_d + (round_3d - round_d) // 2, u, v, i))
    else:
        heappush(edge_dist_idx, (round_d, u, v, i))
    cnt += 1


def root(x):
    if rank[x] < 0:
        return x
    rank[x] = root(rank[x])
    return rank[x]
def unite(x, y):
    x, y = root(x), root(y)
    if x == y:
        return False
    if rank[x] > rank[y]:
        x, y = y, x
    rank[x] += rank[y]
    rank[y] = x
    return True
def is_same(x, y):
    return root(x) == root(y)
def size(x):
    return -rank[root(x)]


rank = [-1] * N
used_edge_idx = set()
while edge_dist_idx:
    dist, u, v, idx = heappop(edge_dist_idx)
    if not is_same(u, v):
        unite(u, v)
        used_edge_idx.add(idx)

    if size(u) == N:
        break

for i in range(M):
    L = int(input())
    if i in used_edge_idx:
        print(1)
    else:
        print(0)