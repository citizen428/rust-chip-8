test:
  @cargo test

test-bc:
  @cargo run -- roms/BC_test.ch8

test-opcode:
  @cargo run -- roms/test_opcode.ch8

test-audio:
  @cargo run -- roms/chip8-test-rom-with-audio.ch8
