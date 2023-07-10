if [ $# -eq 0 ]; then
	echo "Usage: ./build.sh [exercise number]"
	echo "If no argument is given, only the library will be built."
fi

# Clean library if it exists
if [ -f libpal.rlib ]; then
	rm libpal.rlib
fi

rustc libpal/pal.rs --crate-type=rlib

# Build and run exercise if argument is given
if [ $# -ne 0 ]; then
	rm e$1.exe
	rustc set1/exercise$1.rs --extern libpal=libpal.rlib -o e$1.exe
	./e$1.exe
fi
