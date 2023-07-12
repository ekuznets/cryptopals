#!/bin/sh

# TODO: Rewrite this script into a Rust Cargo
if [ $# -eq 0 ]; then
	echo "Usage: ./build.sh [exercise number]"
	echo "If no argument is given, only the library will be built."
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
fi
