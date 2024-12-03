#!/bin/bash

set -x

session_id=<id>
file_name="$(printf "%02d" $1).txt"

curl -o "inputs/$file_name" --cookie "session=$session_id" https://adventofcode.com/2024/day/$1/input

set +x
