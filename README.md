
# Macboxx_rs

A hopefully more performant rewrite of [macboxx](https://github.com/prmaloney/macboxx).

## Summary
A virtual controller to interact with slippi dolphin.
[b0xx-ahk](https://github.com/agirardeau/b0xx-ahk) runs only on windows and this would give support for b0xx-y controller mappings to other platform users.

## Installation
Install with cargo:
```bash
cargo install macboxx
```

## Usage
```bash
macboxx -s <slippi path> -k <keymap path>
```
Where `<slippi path>` is the path to the slippi netplay directory. On MacOS, this is something like `~/Library/Application\ Support/com.project-slippi.dolphin/netplay`,
and `<keymap path>` is the path to the `keymap.toml` file. If you don't have one, one will be created for you in your home directory.

## Keymap
`keymap.toml` should have the following format:
```toml
[buttons]
A = 'J'
B = 'O'
X = 'K'
Y = '/'
Z = 'I'
START = 'Return'
D_UP = 'UpArrow'
D_DOWN = 'DownArrow'
D_LEFT = 'LeftArrow'
D_RIGHT = 'RightArrow'

[control_stick]
UP = 'W'
DOWN = 'S'
LEFT = 'A'
RIGHT = 'D'

[c_stick]
UP = ';'
DOWN = ','
LEFT = 'N'
RIGHT = 'L'

[triggers]
L = 'Q'
R = ' ' 

[mods]
MOD_Y = 'ShiftLeft'
MOD_X = 'ShiftRight'

[mod_factors]
x = 0.5
down = 0.42
up = 0.4
```
(shoutout to lord's [layout](https://imgur.com/a/3SmBW) he shared on The Reads)

For non-alphanumeric keys, refer to the [rdev Key enum](https://docs.rs/rdev/latest/rdev/enum.Key.html).
All other keys are uppercase letters, or the char they represent.

