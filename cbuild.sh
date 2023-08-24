# build test exec
cargo build --lib --release
clang++ -Oz -o ctest -std=c++17 main.cpp target/release/libnaga.a
strip ctest -o ctest_strip
./ctest_strip
spirv-dis ctest.spv -o ctest.spvasm
