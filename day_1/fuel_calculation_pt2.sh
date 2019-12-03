#!/usr/bin/env bash

# set -euo pipefail

function calculate_fuel_required () {
    mass=$1
    fuel=0
    while [ $mass -gt 6 ];
    do
        foo=$((mass/3))
        foo=$((foo-2))
        fuel=$((fuel+foo))
        mass=$foo
    done

    echo $fuel
}

inputfile='../input/day_1.txt'
overall_sum=0

# bla=$(calculate_fuel_required 200)

while read line;
do
    this_fuel=$(calculate_fuel_required $line)
    overall_sum=$((overall_sum+this_fuel))
done < $inputfile

echo "Overall fuel required: $overall_sum."