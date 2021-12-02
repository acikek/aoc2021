# acikek's AoC 2021

This year I'll be using Rust for my solutions. Initially I was very hesitant to use it, since I've written most of my recent projects in it, but I caved in.

**Obvious spoilers for AoC2021 solutions.**

## Running the Solutions

Clone this repository:
```sh
git clone https://github.com/acikek/aoc2021
```

The `input` folder is meant for the input text files that the AoC site provides. These files should be piped into stdin before running the solution. The `test` folder is similar, but for the example keys that the site provides, which are smaller scale and easier to debug.

In `src/main.rs`, change the import line (7) to the requested day. Yes, this is how it actually works...