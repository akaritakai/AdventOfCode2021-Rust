# Advent Of Code 2021 Solutions

[![Build Status](https://github.com/akaritakai/AdventOfCode2021-Rust/actions/workflows/main.yml/badge.svg)](https://github.com/akaritakai/AdventOfCode2021-Rust/actions)
[![Code Coverage](https://img.shields.io/codecov/c/github/akaritakai/AdventOfCode2021-Rust.svg)](https://codecov.io/gh/akaritakai/AdventOfCode2021-Rust)

This repo contains my Advent Of Code 2021 solutions in Rust.

This repo is a learning opportunity (a "my first Rust project") and thus is likely full of horrors and bad practices.

For a more user-friendly repo, go here: [Advent of Code 2021 Solutions](https://github.com/akaritakai/AdventOfCode2021).

## Providing Your Puzzle Input

There are two supported methods for inputting your puzzle data into this application.

### Automatic Puzzle Fetcher (via Session Cookie)

First, get your cookie session data.

You will need to log into the Advent of Code website and then inspect your cookies.
If you are using Chrome, you can follow the directions [here](https://developers.google.com/web/tools/chrome-devtools/storage/cookies).

You will be looking for a cookie called `session`. It will contain a long sequence of hexadecimal digits.

Place that data into a file called `cookie.txt` in the project directory.

The application will use that data to automatically fetch your puzzle input for each day.

### Manual Input

This code will also look in a particular location on your local machine for puzzle input.

In the project directory, it will check a directory called `puzzle`.
Within that directory it will expect Day 1's input to be in a file called `1`, Day 2's input to be in a file called `2`, etc.
 
You can find your puzzle input for a given day by logging into the Advent of Code website and then navigating to the URL
for that puzzle's input.

The URL for your puzzle input will be at:
```
https://adventofcode.com/2021/day/${DAY}/input
```
where `${DAY}` is the day number of the puzzle.

As an example, Day 1's input is at https://adventofcode.com/2021/day/1/input,
Day 2's input is at https://adventofcode.com/2021/day/2/input, etc.

## Docker Instructions (Mac/Linux)

1. Follow the instructions above for providing your puzzle input.
2. Run `docker run --rm -it $(docker build -q .)`

## Windows/Mac/Linux Instructions

1. Follow the instructions above for providing your puzzle input.
2. Install Rust. You can follow the installation instructions [here](https://www.rust-lang.org/tools/install).
3. Open a command prompt and navigate to the project directory.
4. Run `cargo run --release`
