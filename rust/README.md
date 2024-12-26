# Running
Run with `cargo run -q -- run -y 2023 -d 1 -p 1`.

To use example inputs, add `e`, `e1` or `e2` to the above command.

Generate new days with `cargo run -q -- generate -y 2024 -d 7`.

For help, run `cargo run -- --help`.

# Inputs
Add input files in `./src/year*/input/*.txt`. For example:
```
./src/year2023/input/day01.txt
./src/year2023/input/day01.example01.txt

./src/year2023/input/day03
./src/year2023/input/day03.example01.txt
```
