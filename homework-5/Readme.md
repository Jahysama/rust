# Rust Error handling

## Assignment Description
This exercise emphasizes proper error propagation, eliminating the use of unwraps and expects, and utilizing Option and Result types for comprehensive error representation.

## Setup

This time I have decided to ditch crane completly in favour of more familiar tool, devenv.


Activate devenv

```shell
devenv shell
```

Init rust env

```shell
cargo init
cargo add csv nu-ansi-term nu-table
```

## Transformations
For all transformations descriptions please reference [homework-2](../homework-2/)

## CSV

### Rendering
Since I'm an active user of nushell and like all awesome tools it has been written in Rust, I decided to borrow its table rendering from official internal crates.

### Executing

You can input rows of csv as long as you want, it would count empty string as end of input.

```shell
cargo run -- csv
```
or you can just cat your csv

```shell
cat my_snp.csv | cargo run -- csv 
```

## About my_snp.csv

This file contains some of single nucleotide polymorphisms (snp for short) from mine 1st chromosome. I use this file to test my program.
