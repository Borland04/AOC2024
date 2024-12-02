#!/bin/bash

set -x

session_id=53616c7465645f5fa69a15c5fc04d31535125ac2b2aead0071b73ce8fd00bc15e49e40a1a7ff1bb3a02ecfec85cf309af12111fd8a46e03c5b86e4a0fc8b6994
file_name="$(printf "%02d" $1).txt"

curl -o "inputs/$file_name" --cookie "session=$session_id" https://adventofcode.com/2024/day/$1/input

set +x
