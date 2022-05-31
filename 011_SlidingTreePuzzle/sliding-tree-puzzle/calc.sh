#!/bin/bash

# bash calc_multi_seed.sh [num : 0 ~ num.txt]


if [ $1 ]; then
    end=$(($1-1))
else
    end=49
fi

rm score_log.txt && touch score_log.txt

for i in `seq 0 $end`
do
    num=$(printf "%04d\n" "$i")
    cargo run --bin main < in/$num.txt >> score_log.txt
    printf "\r[ $((i + 1)) / $(($end + 1)) ]"
done
printf "\n"

python calc_total_score.py $end < score_log.txt