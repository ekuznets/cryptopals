#!/bin/bash

# TODO: Rewrite this script into a Rust Cargo
if [ $# -eq 0 ]; then
	echo "Usage: ./build.sh [exercise number]"
	echo "If no argument is given, all exercises will be build."
fi

# Clean library if it exists
if [ -f libpal.rlib ]; then
	rm libpal.rlib
	rm libbase64Tools.rlib
fi

rustc libpal/base64Tools.rs --crate-type=rlib
rustc libpal/pal.rs --crate-type=rlib --extern base64Tools=libbase64Tools.rlib 

# Build and run exercise if argument is given
if [ $# -ne 0 ]; then
	rustc set1/exercise$1.rs --extern base64Tools=libbase64Tools.rlib --extern libpal=libpal.rlib -o e$1.exe
	./e$1.exe
else
	# Build and run all exercises
	for i in {1..5}
	do
		rustc set1/exercise$i.rs --extern base64Tools=libbase64Tools.rlib --extern libpal=libpal.rlib -o e$i.exe
	done
	for i in {1..5}
	do
		./e$i.exe
	done
fi
