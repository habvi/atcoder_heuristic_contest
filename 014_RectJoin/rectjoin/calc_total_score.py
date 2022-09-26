from collections import defaultdict
import sys

args = sys.argv
times = int(args[1]) + 1

total_score = 0
total_t = 0
result = defaultdict(list)
time_list = []

i = 0
path = "score_log.txt"
with open(path) as f:
    lines = f.read()
    for line in lines.split('\n'):
        if line and line[0].isdigit():
            n, m, score, t, ans_num = map(float, line.split())
            n, m, score, ans_num = int(n), int(m), int(score), int(ans_num)
            total_score += score
            time_list.append(t)
            total_t += t
            result[n].append((i, m, score, t, ans_num))
            i += 1

path = "result.txt"
for k in sorted(result.keys()):
    result[k].sort(key=lambda x: -x[2])
    with open(path, mode='a') as f:
        f.write("----- n = " + str(k) + ' -----\n')
        for (i, m, score, t, ans_num) in result[k]:
            f.write(str(i) + ' ' + str(m) + ' ' + str(score) + ' ' + str(t)[:5] + ' ' + str(ans_num) + '\n')
        f.write('\n')

print('total score : ', '{:,}'.format(total_score))
print('max time : ', '{:,}'.format(max(time_list)))
print('average time : ', '{:,}'.format(total_t / times))
# print('if 100 times:', '{:,}'.format(int(100 / times * total_score)))
