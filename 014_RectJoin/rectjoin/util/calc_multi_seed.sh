#!/bin/bash

# bash calc_multi_seed.sh [filename] $2[num : 0 ~ num.txt]


if [ "$2" ]; then
    end=$(("$2"-1))
else
    end=9
fi

echo "" > score_log.txt

for i in $(seq 0 "$end")
do
    num=$(printf "%04d\n" "$i")
    cargo run --bin "$1" < ../in/"$num".txt > out/out"$num".txt 2>> score_log.txt
    printf "\r[ $((i + 1)) / $(("$end" + 1)) ]"
done
printf "\n"

echo "" > result.txt
python calc_total_score.py "$end" + 1 < score_log.txt