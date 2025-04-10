test:
  @cargo test -- --test-threads=1

test-rom:
  @cargo run -- roms/BC_test.ch8
