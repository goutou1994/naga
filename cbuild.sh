cargo build --lib --release
clang++ -Oz -o ctest -std=c++17 main.cpp target/release/libnaga.a
./ctest
