from random import randint

N = 20

sy, sx, gy, gx, P = input().split()
sy, sx, gy, gx = map(int, [sy, sx, gy, gx])
P = float(P)

R = [input() for _ in range(20)]
D = [input() for _ in range(19)]

res = []
for i in range(13):
    res.append('RD' * 5)
    if i % 2:
        res.append('UURR')
    else:
        res.append('LLDD')

dir = 'UDLR'
for _ in range((200 - len(''.join(res))) // 2):
    res.append(dir[randint(0, 3)] * 2)

print(''.join(res))