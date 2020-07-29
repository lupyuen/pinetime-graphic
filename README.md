# Convert PNG graphic for PineTime Bootloader

To convert the file `pinetime-graphic.png` to a C array...

```bash
cargo build
cargo run -v pinetime-graphic.png
```

To export the C array as a binary file, see [`export-bin.c`](export-bin.c)

Read the article ["MCUBoot Bootloader for PineTime Smart Watch (nRF52)"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/mcuboot)
