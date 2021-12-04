# 🎄 Advent of Code 2021 🎄

For this year I asked Santa to help me solve it in Rust.

## Process

1. The first upload for each day is roughly the code that generated
   the solution that I "turned in" to the web tool. This code is somewhere
   between "idiomatic" and "a mess". 

2. Then, I might do some small cleanups in place to arrive at a 
   mostly-idiomatic solution.

3. Finally, and this is the real fun for me, I'll work on benchmarking 
   using the new rust `Bencher` lib. These will be duplicatd below the main solution.
   (I may need to clean up days 1-3 to match this standard.) 

   Use `cargo bench solve` to see the high-level benchmarks for each day. This should include
   an `original` benchmark that is wired up to the idiomatic solution, as well as a `current` 
   benchmark that shows the fastest I've been able to get it. Sometimes this code is not very idiomatic
   at all. However, sometimes fast code is reasonably expressive as well! It's been interesting
   to explore the intersection of these as part of my Rust evaluation.

   Use `cargo bench --bin part$DAY` to see _all_ benchmarks for the given day. These will usually 
   include a number of "unit" benchmarks that were used in the process of optimizing the various
   components of the solution.