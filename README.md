# Google Kickstart 2020 - Stable Wall

This repository contains a solution to the Round C 2020 - Kick Start 2020 problem, "Stable Wall."

## Overview

The solution approaches the problem by:
- Generating possible configurations for polyomino placements.
- Using dynamic programming and/or depth-first search techniques to construct the wall layer-by-layer.
- Validating each configuration to ensure stability, meaning every block is backed by sufficient support from the layer below.

## How It Works

1. Preprocess all possible polyomino configurations that can form a valid wall section.
2. Recursively or iteratively compose the wall while checking that all configurations meet the stability constraints.
3. Apply optimization techniques to prune invalid or redundant configurations for efficiency.

## Running the Solution

- Build the project with Cargo:
  ```bash
  cargo build --release
  ```
- Run the solution:
  ```bash
  ./target/release/polyominals < pol.in > result.txt && diff result.txt pol.out
  ```

## References

The problem statement can be found here: [Stable Wall](https://web.archive.org/web/20230405033449/https://codingcompetitions.withgoogle.com/kickstart/round/000000000019ff43/00000000003379bb#problem)

