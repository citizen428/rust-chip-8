# rust-chip-8

A CHIP-8 emulator written in Rust

## Keyboard mapping

CHIP-8 systems used a hexadecimal keyboard with the layout shown on the left.
This is mapped to the physical keyboard as shown on the right.
>[!NOTE]
> While the diagram uses a QWERTY layout, the implementation is based on scancodes, so this
> should work with any keyboard layout.

```
|---|---|---|---|               |---|---|---|---|
| 1 | 2 | 3 | C |               | 1 | 2 | 3 | 4 |
|---|---|---|---|               |---|---|---|---|
| 4 | 5 | 6 | D |               | Q | W | E | R |
|---|---|---|---|               |---|---|---|---|
| 7 | 8 | 0 | E |               | A | S | D | F |
|---|---|---|---|               |---|---|---|---|
| A | 0 | B | F |               | Z | X | C | V |
|---|---|---|---|               |---|---|---|---|
 ```

So to get "deadbeef" inside the emulator you'd have to type "rfzrcffv".

## Resources

* [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator)
* [Rust Programming: From Jim to Him Pt 1 — CHIP8 Emulator](https://alecgarza96.medium.com/rust-from-jim-to-him-pt-1-chip8-emulator-22c268e17b6e)

## TODO and ideas

* Build an alternative frontend, e.g. WASM.

## License

MIT License Copyright (c) 2021 Michael Kohl

For the full license text see [LICENSE](./LICENSE).

## Test ROM licenses

* `BC_test.ch8`: test ROM by BestCoder, see [BC_test.txt](./roms/BC_test.txt)
* Other ROMS under MIT license, see [NinjaWeedle/chip8-test-rom-with-audio](https://github.com/NinjaWeedle/chip8-test-rom-with-audio)
