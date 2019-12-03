#!/usr/bin/env bash

function calculate_fuel_required () {
    foo=$(($1 / 3))
    foo=$(($foo - 2))
    echo $foo # fun fact: You can't remove this line without the program breaking ¯\_(ツ)_/¯ 
    return $foo
}

inputfile='../input/day_1.txt'
overall_sum=0

while read line;
do
    overall_sum=$(($overall_sum+$(calculate_fuel_required $line)))
done < $inputfile

echo "Overall fuel required: $overall_sum."