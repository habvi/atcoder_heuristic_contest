from heapq import heapify, heappush, heappop

center_x, center_y = 400, 400
N = 1000
M = 50

def _input(all_place):
    for i in range(N):
        res_x, res_y, cus_x, cus_y = map(int, input().split())
        dst_cen_to_res = abs(center_x - res_x) + abs(center_y - res_y)
        dst_cen_to_cus = abs(center_x - cus_x) + abs(center_y - cus_y)
        dst_total = dst_cen_to_res + dst_cen_to_cus
        heappush(all_place, (dst_total, res_x, res_y, cus_x, cus_y, i+1))
    return all_place

def select_50(all_place):
    place_50 = []
    heapify(place_50)
    for _ in range(M):
        heappush(place_50, heappop(all_place))
    return place_50

def output(place_50):
    ans1 = [50]
    ans2 = [102, 400, 400]
    for i in range(M):
        _dst, res_x, res_y, cus_x, cus_y, idx = heappop(place_50)
        ans1.append(idx)
        for xy in (res_x, res_y, cus_x, cus_y):
            ans2.append(xy)
    ans2.append(400)
    ans2.append(400)
    print(*ans1)
    print(*ans2)

def main():
    all_place = []
    heapify(all_place)
    all_place = _input(all_place)
    place_50 = select_50(all_place)
    output(place_50)

main()