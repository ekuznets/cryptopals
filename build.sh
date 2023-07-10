rustc libpal/pal.rs --crate-type=rlib
#rustc set1/exercise1.rs --extern libpal=libpal.rlib -o e1.exe
#rustc set1/exercise2.rs --extern libpal=libpal.rlib -o e2.exe
#rustc set1/exercise3.rs --extern libpal=libpal.rlib -o e3.exe
#rustc set1/exercise4.rs --extern libpal=libpal.rlib -o e4.exe
rustc set1/exercise5.rs --extern libpal=libpal.rlib -o e5.exe