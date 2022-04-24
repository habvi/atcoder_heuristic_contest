from random import randrange

N = 30

TO = [[1, 0, -1, -1],
      [3, -1, -1, 0],
      [-1, -1, 3, 2],
      [-1, 2, 1, -1],
      [1, 0, 3, 2],
      [3, 2, 1, 0],
      [2, -1, 0, -1],
      [-1, 3, -1, 1],]

DI = [0, -1, 0, 1]
DJ = [-1, 0, 1, 0]

ROTATE = [[0, 1, 2, 3],
          [1, 2, 3, 0],
          [2, 3, 0, 1],
          [3, 0, 1, 2],
          [4, 5, 4, 5],
          [5, 4, 5, 4],
          [6, 7, 6, 7],
          [7, 6, 7, 6]]


def calc_score(si, sj, dir, route, tiles):
    i, j = si, sj
    d = dir
    length = 0
    while True:
        nxt_dir = TO[tiles[i][j]][d]
        if nxt_dir == -1:
            return 0, route
        i += DI[nxt_dir]
        j += DJ[nxt_dir]
        if not (0 <= i < N and 0 <= j < N):
            return 0, route
        d = (nxt_dir + 2) % 4
        route.append((i, j))
        length += 1
        if (i, j) == (si, sj) and d == dir:
            return length, route


def get_tile_dir(t):
    for i, to in enumerate(TO[t]):
        if to != -1:
            return i


def calc_max_score(tiles):
    seen = [[0] * N for _ in range(N)]
    scores = []
    for i in range(N):
        for j in range(N):
            if seen[i][j]:
                continue

            route = [(i, j)]
            score, route = calc_score(i, j, get_tile_dir(tiles[i][j]), route, tiles)
            if score:
                scores.append(score)
            for ri, rj in route:
                seen[ri][rj] = 1

    if len(scores) <= 1:
        return 0
    else:
        scores.sort()
        return scores[-1] * scores[-2]


def get_random_int():
    return randrange(0, 4)


def random_move(times):
    max_score = 0
    max_move = ['0'] * (N * N)
    for _ in range(times):
        move = []
        tiles_after = [[None] * N for _ in range(N)]
        for i in range(N):
            for j in range(N):
                ri = get_random_int()
                move.append(str(ri))
                tiles_after[i][j] = ROTATE[tiles[i][j]][ri]

        score = calc_max_score(tiles_after)
        if score > max_score:
            max_score = score
            max_move = move

    return ''.join(max_move), max_score



tiles = [tuple(map(int, list(input()))) for _ in range(N)]

times = 100
max_move, max_score = random_move(times)
print(max_move)