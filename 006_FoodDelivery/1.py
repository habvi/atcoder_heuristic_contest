from heapq import heapify, heappush, heappop, heappushpop

center_x, center_y = 400, 400
all_place = []
heapify(all_place)

n = 1000
for i in range(n):
    res_x, res_y, cus_x, cus_y = map(int, input().split())
    dst_cen_to_res = abs(center_x - res_x) + abs(center_y - res_y)
    dst_cen_to_cus = abs(center_x - cus_x) + abs(center_y - cus_y)
    dst_total = dst_cen_to_res + dst_cen_to_cus
    heappush(all_place, (dst_total, res_x, res_y, cus_x, cus_y, i+1))

m = 50
ans1 = [50]
ans2 = [102, 400, 400]
for i in range(m):
    _dst, res_x, res_y, cus_x, cus_y, idx = heappop(all_place)
    ans1.append(idx)
    for xy in (res_x, res_y, cus_x, cus_y):
        ans2.append(xy)

for _ in range(2):
    ans2.append(400)
print(*ans1)
print(*ans2)