from collections import defaultdict, deque

N = 20

def bfs(sy, sx, gy, gx):
    q = deque()
    q.append((sy, sx))
    while q:
        y, x = q.popleft()
        if (y, x) == (gy, gx):
            return
        for ny, nx in G[(y, x)]:
            if dist[ny][nx] != -1:
                continue
            dist[ny][nx] = dist[y][x] + 1
            q.append((ny, nx))


sy, sx, gy, gx, P = input().split()
sy, sx, gy, gx = map(int, [sy, sx, gy, gx])
P = float(P)

R = [input() for _ in range(20)]
D = [input() for _ in range(19)]

G = defaultdict(list)
for y in range(N):
    for x in range(N):
        if y - 1 >= 0 and D[y - 1][x] == '0':
            G[(y, x)].append((y - 1, x))
        if y + 1 < N and D[y][x] == '0':
            G[(y, x)].append((y + 1, x))
        if x - 1 >= 0 and R[y][x - 1] == '0':
            G[(y, x)].append((y, x - 1))
        if x + 1 < N and R[y][x] == '0':
            G[(y, x)].append((y, x + 1))

dist = [[-1] * N for _ in range(N)]
dist[sy][sx] = 0
bfs(sy, sx, gy, gx)

route = []
y, x = gy, gx
d = dist[gy][gx]
while d != 0:
    for ny, nx in G[(y, x)]:
        if dist[ny][nx] == d - 1:
            if y - 1 == ny:
                y -= 1
                route.append('D')
            elif y + 1 == ny:
                y += 1
                route.append('U')
            elif x - 1 == nx:
                x -= 1
                route.append('R')
            else:
                x += 1
                route.append('L')
            d -= 1
            break

route = route[::-1]
# print(''.join(route))



res = []
for i in range(16):
    res.append('RD' * 3)
    if i % 2:
        res.append('LLDD')
    else:
        res.append('UURR')


from random import randint

dir = 'UDLR'
for _ in range(20):
    res.append(dir[randint(0, 3)] * 2)

print(''.join(res))