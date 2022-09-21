from collections import defaultdict
import sys

args = sys.argv
times = int(args[1]) + 1

total_score = 0
total_t = 0
result = defaultdict(list)
ts = []
input()
for i in range(times):
    input()
    input()
    n, m, score, t = map(float, input().split())
    n, m, score = int(n), int(m), int(score)
    total_score += score
    ts.append(t)
    total_t += t
    result[n].append((i, m, score, t))

path = "result.txt"
for k in sorted(result.keys()):
    result[k].sort(key=lambda x: -x[2])
    with open(path, mode='a') as f:
        f.write("----- n = " + str(k) + ' -----\n')
        for (i, m, score, t) in result[k]:
            f.write(str(i) + ' ' + str(m) + ' ' + str(score) + ' ' + str(t)[:5] + '\n')
        f.write('\n')

print('total score : ', '{:,}'.format(total_score))
print('max time : ', '{:,}'.format(max(ts)))
print('average time : ', '{:,}'.format(total_t / times))
# print('if 100 times:', '{:,}'.format(int(100 / times * total_score)))