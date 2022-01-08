# crop
Simple command line tool to crop one or multiple images at once; written in Rust.

## Installation

0. If you haven't installed *Rust* / *rustup* yet, go to [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) and follow the instructions for your operating system. 
1. `rustup update`
2. `cargo install --git https://github.com/k-gruenberg/crop`

## Usage

```
crop 1.0.0
Kendrick Grünberg
Simple command line tool to crop one or multiple images at once

USAGE:
    crop [OPTIONS] --x1 <X1> --y1 <Y1> <--width <WIDTH>|--x2 <X2>> <--height <HEIGHT>|--y2 <Y2>> <PATH>

ARGS:
    <PATH>    Path to an image file or to a directory containing image files

OPTIONS:
    -h, --help               Print help information
        --height <HEIGHT>    Height of the cropped section
        --relative           With this flag, the x1, y1, x2, y2, width, height arguments are treated
                             as relative values between 0.0 and 1.0 instead of absolute pixel values
        --silent             Deactivates output to stdout, errors are still printed to stderr
    -V, --version            Print version information
        --width <WIDTH>      Width of the cropped section
        --x1 <X1>            X position of the top-left crop point
        --x2 <X2>            X position of the bottom-right crop point
        --y1 <Y1>            Y position of the top-left crop point
        --y2 <Y2>            Y position of the bottom-right crop point
```
