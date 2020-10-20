# Convert PNG graphic to RGB565 for PineTime Watch Faces and PineTime Bootloader

## PineTime Watch Face

To convert the PNG image `0.png` (240 x 240 resolution or smaller) to RGB565 image `0.bin`...

```bash
cargo run -v 0.png >pinetime-graphic.inc
gcc -o export-bin export-bin.c
./export-bin
mv pinetime-graphic.bin 0.bin
```

## PineTime Bootloader

To convert the file `pinetime-graphic.png` (240 x 240 resolution) to a C array...

```bash
cargo run -v pinetime-graphic.png

```

To export the C array as a binary file `pinetime-graphic.bin`...

```bash
cargo run -v pinetime-graphic.png >pinetime-graphic.inc

gcc -o export-bin export-bin.c

./export-bin

```

To convert binary file `pinetime-graphic.bin` to HEX file `pinetime-graphic.hex`...

```bash
arm-none-eabi-objcopy -I binary -O ihex pinetime-graphic.bin pinetime-graphic.hex

```

Read the article ["MCUBoot Bootloader for PineTime Smart Watch (nRF52)"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/mcuboot)
