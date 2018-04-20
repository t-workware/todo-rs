#!/bin/bash
# use:
#   echo "arg" | to command
# to pass stdin lines as args: `command arg`

lines=()
while read line
do
    echo $line
    lines+=("$line")
done <&0
$@ "${lines[@]}"
