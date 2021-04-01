# rust-chip-8

A CHIP-8 emulator written in Rust

## Keyboard mapping

CHIP-8 systems used a hexidecimal keyboard with the following layout:

|---|---|---|---|
| 1 | 2 | 3 | C |
|---|---|---|---|
| 4 | 5 | 6 | D |
|---|---|---|---|
| 7 | 8 | 0 | E |
|---|---|---|---|
| A | 0 | B | F |
|---|---|---|---|

In this emulator the following keys are mapped to the above layout:

|---|---|---|---|
| 1 | 2 | 3 | 4 |
|---|---|---|---|
| Q | W | E | R |
|---|---|---|---|
| A | S | D | F |
|---|---|---|---|
| Z | X | C | V |
|---|---|---|---|

So to get "deadbeef" inside the emulator you'd have to type "rfzrcffv".

## License

MIT License Copyright (c) 2021 Michael Kohl

For the full license text see [LICENSE](./LICENSE).
