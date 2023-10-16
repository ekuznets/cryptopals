#!/bin/bash

# Simple test runner with failure detection
TestExercise () {
	cargo run --bin exercise$1
	ret_code=$?
	if [ $ret_code != 0 ]; then
		printf 'Build has failed, return status %d\n' $ret_code
		exit $ret_code
	fi
}

# Build the project
echo "Building the project..."

cd set1
cargo build --release
TestExercise 1
TestExercise 2
TestExercise 3
TestExercise 4
TestExercise 5
TestExercise 6
TestExercise 7
TestExercise 8

cd ../set2
TestExercise 9

echo "Done!"
