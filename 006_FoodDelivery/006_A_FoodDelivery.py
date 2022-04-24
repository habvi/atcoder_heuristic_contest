from heapq import heapify, heappush, heappop

NOW_X, NOW_Y = 400, 400
N = 1000
M = 50

def _input(all_place):
    for i in range(N):
        res_x, res_y, cus_x, cus_y = map(int, input().split())
        dst_cen_to_res = abs(NOW_X - res_x) + abs(NOW_Y - res_y)
        dst_cen_to_cus = abs(NOW_X - cus_x) + abs(NOW_Y - cus_y)
        dst_res_to_cus = abs(res_x - cus_x) + abs(res_y - cus_y)
        dst_total = dst_cen_to_res + dst_cen_to_cus
        heappush(all_place, (dst_cen_to_res, dst_total, dst_res_to_cus, res_x, res_y, i+1, 1))
        heappush(all_place, (dst_cen_to_cus, dst_total, dst_res_to_cus, cus_x, cus_y, i+1, 2))
    return all_place

def select_50(all_place):
    place_50 = []
    heapify(place_50)
    cnt = [0]*(N + 1)
    stay = [[] for _ in range(N + 1)]
    while len(place_50) != M:
        dst_cen_to_, dst_total, dst_res_to_cus, x, y, idx, num = heappop(all_place)
        stay[idx].append((dst_total, x, y, idx, num))
        if cnt[idx] == 1:
            _push = [stay[idx][0][0]]
            for dst_total, x, y, idx, num in sorted(stay[idx], key=lambda x: x[-1]):
                _push.append(x) 
                _push.append(y) 
            _push.append(stay[idx][0][-2])
            heappush(place_50, tuple(_push))
            continue
        cnt[idx] += 1
    return place_50

def close_order(place_50):
    global NOW_X, NOW_Y
    now_x, now_y = NOW_X, NOW_Y

    next_place = []
    heapify(next_place)
    for _ in range(len(place_50)):
        _dst, res_x, res_y, cus_x, cus_y, idx = heappop(place_50)
        dst_now_res = abs(now_x - res_x) + abs(now_y - res_y)
        dst_now_cus = abs(now_x - cus_x) + abs(now_y - cus_y)
        heappush(next_place, (dst_now_res, res_x, res_y, idx, 1))
        heappush(next_place, (dst_now_cus, cus_x, cus_y, idx, 2))

    ordered_50 = []
    rest_all = next_place
    received = [0] * (N+1)
    while len(ordered_50) != M*2:
        next_place = []
        heapify(next_place)
        for _ in range(len(rest_all)):
            _dst, x, y, idx, num = heappop(rest_all)
            dst_from_now = abs(now_x - x) + abs(now_y - y)
            heappush(next_place, (dst_from_now, x, y, idx, num))

        go = False
        stay = []
        heapify(stay)
        while not go:
            _dst, x, y, idx, num = heappop(next_place)
            if num == 1:
                received[idx] = 1
                go = True
            else:
                if received[idx] != 1:
                    heappush(stay, (_dst, x, y, idx, num))
                    received[idx] = 2
                    continue
                received[idx] = 2
                go = True

        ordered_50.append((x, y, idx, num))
        rest_all = next_place + stay
        now_x, now_y = x, y
    return ordered_50

def output(ordered_50):
    ans1 = [50]
    ans2 = [102, 400, 400]
    ids = set()
    for x, y, idx, num in ordered_50:
        ids.add(idx)
        for xy in (x, y):
            ans2.append(xy)
    for _ in range(2):
        ans2.append(400)
    ans1 += list(ids)
    print(*ans1)
    print(*ans2)

def main():
    all_place = []
    heapify(all_place)
    all_place = _input(all_place)
    place_50 = select_50(all_place)
    ordered_50 = close_order(place_50)
    output(ordered_50)

main()