## Dice Notation Specification

| Symbol | Description                                                                                            | Example                   | Example Explanation                                                                        |
| ------ | -----------                                                                                            | -------                   | -------------------                                                                        |
| d      | Separates the number of dice & side count of the dice.                                                 | 1d10                      | Rolls 1 dice with 10 sides.                                                                |
| +      | Indicates a number or dice roll added to a dice roll                                                   | 1d20+1                    | Rolls 1 dice with 20 sides & add 1 to the result.                                          |
| -      | Indicates a number or dice roll subtracted from a dice roll.                                           | 1d6-2                     | Rolls 1 dice with 6 sides & subtract 2 to the result.                                      |
| x      | Indicates a number or dice roll to multiply a dice roll by.                                            | 1d10x10+1d10              | Rolls 1 dice with 10 sides, multiplies it by 10, & added another 1d10 to the result.       |
| /      | Indicates a number or dice roll to divide a dice roll by.                                              | 1d20/2                    | Rolls 1 dice with 20 sides, & divides it by 2, rounding up.                                |
| %      | Same as '/', but always round down.                                                                    | 1d20%2                    | Rolls 1 dice with 20 sides, & divides it by 2, rounding down.                              |
| !      | "Explode", or reroll numbers following the '!'. Add the result to the original.                        | 1d10!10                   | Rolls 1 dice with 10 sides, & rerolls it if it's a 10, adding to new roll to the original. |
| ()     | Enforces items within to be handled at the beginning of the order of operations.                       | 2d(10+2)                  | Rolls 2 dice with 10 sides each, & add 2 to each roll.                                     |
| {}     | Used to denote other options than numbers to roll. If 2 numbers are given, will be treated as a range. | 1d{'','','-','-','+','+'} | "Rolls" or picks 1 option from the given list in the {}.                                   |
| kh     | Keep the highest roll. Can also be used as knh for keeping the n highest.                              | 2d20kh                    | Rolls 2 dice with 20 sides each, & keeps the highest (aka: advantage).                     |
| kl     | Keep the lowest roll. Can also be used as knl for keeping the n lowest.                                | 2d20kl                    | Rolls 2 dice with 20 sides each, & keeps the lowest (aka: disadvantage).                   |
| kc     | Keep the player choice. Can also be used as knc for keeping n player choices.                          | 2d20kc                    | Rolls 2 dice with 20 sides each, & keeps the players choice of the 2.                      |
| ,      | Indicates that multiple rolls will be made separately.                                                 | 1d20,1d10                 | Rolls 1 dice with 20 side, 1 dice with 10 sides, & returns the results separately.         |
| D      | Denotes a short form is used. See below for more details.                                              | D%                        | Rolls a "percentile roll" (2 10-sided dice, 1 for tens place, 1 for ones place.)           |
| *      | Currently unused, but reserved.                                                                        |                           |                                                                                            |
| &      | Currently unused, but reserved.                                                                        |                           |                                                                                            |
| #      | Currently unused, but reserved.                                                                        |                           |                                                                                            |
| ^      | Currently unused, but reserved.                                                                        |                           |                                                                                            |
| @      | Currently unused, but reserved.                                                                        |                           |                                                                                            |
| \      | Currently unused, but reserved.                                                                        |                           |                                                                                            |


## Supported Shorthands

| Short Form | Long Form                 |
| ---------- | ------------------------- |
| D%         | 1d100 or 1d10x10+1d10     |
| D10x       | 1d10x1d10                 |
| Df         | 1d{'','','-','-','+','+'} |
| D66        | 1d6x10+1d6                |
| D666       | 2d6,1d6                   |

