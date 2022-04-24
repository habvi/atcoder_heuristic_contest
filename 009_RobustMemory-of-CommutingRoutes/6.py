from random import randint

N = 20

sy, sx, gy, gx, P = input().split()
sy, sx, gy, gx = map(int, [sy, sx, gy, gx])
P = float(P)

R = [input() for _ in range(20)]
D = [input() for _ in range(19)]


def s1():
    res = []
    for i in range(16):
        res.append('RD' * 3)
        if i % 2:
            res.append('UURR')
        else:
            res.append('LLDD')

    dir = 'UDLR'
    for _ in range(20):
        res.append(dir[randint(0, 3)] * 2)

    return ''.join(res)


def s2():
    res = []
    for i in range(12):
        res.append('RD' * 5)
        if i % 2:
            res.append('UURR')
        else:
            res.append('LLDD')

    dir = 'UDLR'
    for _ in range((200 - len(''.join(res))) // 2):
        res.append(dir[randint(0, 3)] * 2)

    return ''.join(res)


def s3():
    res = []
    for i in range(20):
        res.append('RD' * 3)
        if i % 2:
            res.append('UUR')
        else:
            res.append('LLDD')

    dir = 'UDLR'
    for _ in range((200 - len(''.join(res))) // 2):
        res.append(dir[randint(0, 3)] * 2)

    return ''.join(res)


def s4():
    res = []
    for i in range(14):
        res.append('RD' * 5)
        if i % 2:
            res.append('UU')
        else:
            res.append('LLDD')

    dir = 'UDLR'
    for _ in range((200 - len(''.join(res))) // 2):
        res.append(dir[randint(0, 3)] * 2)

    return ''.join(res)


print(s4())