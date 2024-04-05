
# Macboxx_rs

rewrite it in rust

## Summary
A virtual controller to interact with slippi dolphin.
!(https://github.com/agirardeau/b0xx-ahk)[boxx-ahk] runs only on windows and this would give support for boxx-y controller mappings to other platform users.

## Installation
Install with cargo:
```bash
cargo install macboxx
```

## Usage
```bash
macboxx -s <slippi path> -k <keymap path>
```
Where `<slippi path>` is the path to the slippi netplay directory. On MacOS, this is something like `~/Library/Application\ Support/com.project-slippi.dolphon/netplay`,
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
DPAD_UP = 'UpArrow'
DPAD_DOWN = 'DownArrow'
DPAD_LEFT = 'LeftArrow'
DPAD_RIGHT = 'RightArrow'

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
```
(shoutout to lord's [layout](https://imgur.com/a/3SmBW) he shared on The Reads)

For non-alphanumeric keys, refer to the [rdev Ken enum](https://docs.rs/rdev/latest/rdev/enum.Key.html).
All other keys are uppercase letters, or the char they represent.

