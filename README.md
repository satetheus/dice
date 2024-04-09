# Dice
Simple utility to roll dice from the command line/terminal using "1d10+0" dice notation.

## Installation
### Linux AMD64 Releases
  - Download latest binary: [link](https://github.com/satetheus/dice/releases/latest/download/dice_amd64)
  - Rename `dice_amd64` to `dice` & move it to /usr/bin/ if you would like to call it like: `dice 1d10`

### Windows Releases
  - Download latest binary: [link](https://github.com/satetheus/dice/releases/latest/download/dice.exe)

### Mac Releases
  - Download latest binary: [link](https://github.com/satetheus/dice/releases/latest/download/dice_darwin)
  - Rename `dice_darwin` to `dice` & move it to /usr/bin/ if you would like to call it like: `dice 1d10`

### Compile binary
  - [Install Rust](https://rustup.rs/) if not already installed
  - Clone repo
  - Run `cargo build -r`
  - The binary will be "targets/releases/dice", you can move this to /usr/bin/ or use it in place

## Usage
### Dice Notation
This program supports basic [dice notation](https://en.wikipedia.org/wiki/Dice_notation). Multiplication, division, & parenthesis are not supported yet but likely will be in the future.
 - `dice 1d10` will "roll" one dice with 10 sides.
 - `dice d20` will "roll" one dice with 20 sides.
 - `dice 100d20+1` will "roll" 100 dice with 20 sides & add 1 to each.
 - `dice 20d37-24` will "roll" 20 dice with 37 sides & subtract 24 from each.

### Unrecognized Formatting
Any unrecognized format (including no dice notation) will output a warning & the result of a 1d20, assuming that it was what the user wanted.
ex: `dice Hello there!` will output:
```
1st parameter should be a valid dice format. Ex.: 1d20+1 or d10. We rolled a d20 for you in case that's what you wanted.
[13]
```

### Commands
Commands must come after the dice notation with a space between. Otherwise, the program will treat it as unrecognized formatting like in the above section.
Only 1 command can be used at a time.

| name | description                                                                         | example           | example output |
| ---- | -------------                                                                       | -------           | -------------- |
| adv  | Selects the highest total roll out of all rolled dice, including the plus or minus. | `dice 2d20 adv`   | `19`           |
| dis  | Selects the lowest total roll out of all rolled dice, including the plus or minus.  | `dice 2d20+1 dis` | `8`            |
| sum  | Sums all rolled dice including the plus or minus.                                   | `dice 10d4 sum`   | `23`           |
