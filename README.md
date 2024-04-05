
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
A = 'j'
B = 'o'
X = 'k'
Y = '/'
Z = 'i'
START = 'return'
DPAD_UP = 'uparrow'
DPAD_DOWN = 'downarrow'
DPAD_LEFT = 'leftarrow'
DPAD_RIGHT = 'rightarrow'

[control_stick]
UP = 'w'
DOWN = 's'
LEFT = 'a'
RIGHT = 'd'

[c_stick]
UP = 'semicolon'
DOWN = 'comma'
LEFT = 'n'
RIGHT = 'l'

[triggers]
L = 'q'
R = 'space' 

[mods]
MOD_Y = 'shiftleft'
MOD_X = 'shiftright'
```
(shoutout to lord's [layout](https://imgur.com/a/3SmBW) he shared on The Reads)
