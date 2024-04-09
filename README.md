# Dice
Simple utility to roll dice from the command line/terminal using "1d10+0" style format.

## Installation
### Linux AMD64 Releases
  - Download latest release link: https://github.com/satetheus/dice/releases/latest/download/dice_amd64
  - rename `dice_amd64` to `dice` & move it to /usr/bin/ if you would like to call it like: `dice 1d10`

### Windows Releases
  - Download latest release link: https://github.com/satetheus/dice/releases/latest/download/dice.exe

### Mac Releases
  - Download latest release link: https://github.com/satetheus/dice/releases/latest/download/dice_darwin
  - rename `dice_darwin` to `dice` & move it to /usr/bin/ if you would like to call it like: `dice 1d10`

### Compile binary
  - Install rust if not already installed
  - clone repo
  - run `cargo build -r`
  - the binary will be "targets/releases/dice", you can move this to /usr/bin/ or use it in place

## Usage
### dice syntax
`dice 1d10` will "roll" one dice with 10 sides.
`dice 100d20+1` will "roll" 100 dice with 20 sides & add 1 to each.
`dice 20d37-24` will "roll" 20 dice with 37 sides & subtract 24 from each.

### unrecognized formatting
Any unrecognized format (including no dice specifier) will output a warning & the result of a 1d20, assuming that it was what the user wanted.
ex: `dice Hello there!` will output:
```
1st parameter should be a valid dice format. Ex.: 1d20+1 or d10. We rolled a d20 for you in case that's what you wanted.
[13]
```

### Commands
Commands must come after the dice specifier with a space between. Otherwise, the program will treat it as unrecognized formatting like in the above section.
Only 1 command can be used at a time.

| name | description                                                                        | example           | example output |
| adv  | selects the highest total roll out of all rolled dice, including the plus or minus | `dice 2d20 adv`   | `19`           |
| dis  | selects the lowest total roll out of all rolled dice, including the plus or minus  | `dice 2d20+1 dis` | `8`            |
| sum  | sums all rolled dice including the plus or minus                                   | `dice 10d4 sum`   | `23`           |
