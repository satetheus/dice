# Dice
Simple utility to roll dice from the command line/terminal using dice notation.

## Installation
### Linux AMD64 Releases
  - Download latest binary: [link](https://github.com/satetheus/dice/releases/latest/download/dice_amd64)
  - Rename `dice_amd64` to `dice` & move it to /usr/bin/ if you would like to call it like: `dice 1d10`. Otherwise it will be `dice_amd64 1d10`

### Windows Releases
1. Download latest binary: [link](https://github.com/satetheus/dice/releases/latest/download/dice.exe)
2. To run the file, run something like the following in powershell:
    ```powershell
    & "C:\Users\YOU\PATH_TO\dice.exe" 1d20
    ```
    where `YOU` is your username & `PATH_TO` is the path to the file.

### Mac Releases
  - Download latest binary: [link](https://github.com/satetheus/dice/releases/latest/download/dice_darwin)
  - Rename `dice_darwin` to `dice` & move it to /usr/bin/ if you would like to call it like: `dice 1d10`. Otherwise it will be `dice_darwin 1d10`

### Compile binary
  - [Install Rust](https://rustup.rs/) if not already installed
  - Clone repo
  - In the repo directory run `cargo build -r`
  - The binary will be "targets/releases/dice", you can move this to /usr/bin/ or use it in place

## Usage
### Dice Notation
This program supports basic [dice notation](https://en.wikipedia.org/wiki/Dice_notation).
 - `dice 1d10` will "roll" one dice with 10 sides.
 - `dice d20` will "roll" one dice with 20 sides.
 - `dice 100d20+1` will "roll" 100 dice with 20 sides & add 1 to each.
 - `dice 20d37-24` will "roll" 20 dice with 37 sides & subtract 24 from each.
 - `dice (1d10)d(1d10)` will "roll" 1-10 dice with 1-10 sides (all dice will have the same number of sides).
 - `dice 1d10*1d10` will "roll" 2 dice with 10 sides each & multiply them together.
 - `dice 1d20/2` will "roll" 1 dice & divide the result by 2, ignoring remainders.

### Supported Symbols:
| Symbol | description                                                                                          | example                               |
| ---- | -------------                                                                                          | -------                               |
|  d   | Indicates a dice roll with an optional number before it & a required number after (which must be > 1). | `dice d20` or `dice 1d20`             |
|  +   | Indicates a sum, or explicitly showing a number is positive if there is no number ahead of it          | `dice 2d20+4` or `dice 5+d10`         |
|  -   | Indicates a subtraction, or explicitly showing a number is negative if there is no number ahead of it  | `dice 2d20-1` or `dice 10-1d4`        |
|  *   | Indicates a multiplication. Numbers required before & after it.                                        | `dice 5*2d20` or `dice 1d10*1d10`     |
|  /   | Indicates a division. Numbers required before & after it. Does not consider remainders, so 1/2 = 0.    | `dice 10d4/2` or `dice 10/1d10`       |
|  ()  | Indicates whatever is inside should be done first, regardless of sequence priority.                    | `dice (5*30/10)d4` or `dice (1+2)*d6` |


### Roadmap:
| Version | Additions                                                                                           |
|---------|-----------------------------------------------------------------------------------------------------|
| 0.4.0   | 'k' notation, which can be used to roll with advantage & disadvantage. Example: "2d20kh" & "2d20kl" |
| 0.5.0   | ',' notation, allowing multiple separated rolls. Example: "1d10,1d20"                               |
| 0.6.0   | 'c' notation, allowing display of count number of times each side was rolled. Example: "10d10c"     |
| 0.7.0   | '{}' notation, allowing custom symbols for dice sides. Example: "1d{' ','-','+'}" (fate dice)       |
