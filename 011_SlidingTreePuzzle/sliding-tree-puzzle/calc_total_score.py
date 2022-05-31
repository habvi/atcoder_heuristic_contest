import sys

args = sys.argv
times = int(args[1]) + 1

total = 0

for i in range(times):
    input_ = input().split()
    total += int(input_[0])

print('total score : ', '{:,}'.format(total))