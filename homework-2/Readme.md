# Rust Basics: Syntax and Variables

## Assignment Description
Write a program that reads from standard input, transmutes text according to the provided specification, and prints the result back to the user.

## Setup

Copy homework-1

```shell
cp -r homework-1 homework-2
cd homework-2
```

Remove locks

```shell
rm flake.lock Cargo.lock Cargo.toml
```

Init rust env

```shell
cargo init
cargo add slug
nix develop
```

## Transformations
Important to note that the first argument (`args[0]`) is always the program name because this is a standard convention in Unix-like operating systems and is part of how command-line programs are executed. So it is being skipped in my code.

### Suggested

| Argument | Action |
|-----------|---------|
| lowercase | Convert entire text to lowercase |
| uppercase | Convert entire text to uppercase |
| no-spaces | Remove all spaces from text |
| slugify | Convert text into URL-friendly slug |


### Custom

| Argument | Action |
|-----------|---------|
| cheese | Convert all the words into cheese |
| slugify-fr | Convert text into a friendly slug for real |

## Executing

You can convert text as many times as you want, to stop hit `Ctrl+C`

```shell
cargo run -- <one-of-the-above-args>
```
