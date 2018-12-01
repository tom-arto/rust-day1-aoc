# Day 1 - Rust

To run use:

```
cargo run --package rust-day1-aoc --bin rust-day1-aoc [--release]
```

Where `--release` is optional for a faster build.

Optional: put your input in the input.txt file

## Pain points

* Had trouble reading in a file, Rust wouldn't let me use `File::open(..)?` as it said it only works with a result type,
 but the type was `()`. Is `io::Result` different? Not sure what went on here as i had seen example code online that
  did this fine...
* Had to use a lot of mutable variables which is a bit ugly, normally I write using recursive functions in Elixir, but
people don't seem to do recursion in Rust as it doesn't have tail-call optimisation apparently
* Summing on line 20 took a while to puzzle out, at first wasn't giving a type parameter to sum and was getting a strange error
but adding the type solved it