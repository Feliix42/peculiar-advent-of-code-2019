#!/usr/bin/env bash

set -euo pipefail

function calculate_fuel_required () {
    fuel=$1
    foo=$((fuel/3))
    foo=$((foo-2))
    echo $foo
}

inputfile='../input/day_1.txt'
overall_sum=0

while read line;
do
    overall_sum=$((overall_sum+$(calculate_fuel_required $line)))
done < $inputfile

echo "Overall fuel required: $overall_sum."