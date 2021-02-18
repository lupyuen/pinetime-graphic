# Convert PNG graphic to Triple Colour and RGB565 for Grove E-Ink Display, ST7789, PineTime Watch Faces and PineTime Bootloader

## Triple Colour Image for Grove E-Ink Display

To convert the PNG image `uart-cartoon2.png` (202 x 104 resolution) to C arrays `image_black.inc` (black bitmap) and `image_red.inc` (red bitmap)...

```bash
# Download the source code
git clone https://github.com/lupyuen/pinetime-graphic
cd pinetime-graphic

# TODO: Copy uart-cartoon2.png to the pinetime-graphic folder

# Convert the PNG file to a C array (black bitmap) with these min and max thresholds
cargo run -- --min 0  --max 85  uart-cartoon2.png >image_black.inc

# Convert the PNG file to a C array (red bitmap) with these min and max thresholds
cargo run -- --min 86 --max 215 uart-cartoon2.png >image_red.inc
```

Read the article ["PineCone BL602 Talks UART to Grove E-Ink Display"](https://lupyuen.github.io/articles/uart)

## ST7789 Image

To convert the PNG image `image.png` (240 x 240 resolution or smaller) to a C array `image.inc`...

```bash
# Download the source code
git clone https://github.com/lupyuen/pinetime-graphic
cd pinetime-graphic

# TODO: Copy image.png to the pinetime-graphic folder

# Convert the PNG file to a C array
cargo run -v image.png >image.inc
```

Read the article ["PineCone BL602 Blasting Pixels to ST7789 Display with LVGL Library"](https://lupyuen.github.io/articles/display)

## PineTime Watch Face

To convert the PNG image `0.png` (240 x 240 resolution or smaller) to RGB565 image `0.bin`...

```bash
# Download the source code
git clone https://github.com/lupyuen/pinetime-graphic
cd pinetime-graphic

# TODO: Copy 0.png to the pinetime-graphic folder

# Convert the PNG file to a C array
cargo run -v 0.png >pinetime-graphic.inc

# Compile the C array
gcc -o export-bin export-bin.c

# Export the C array as an RGB565 binary file
./export-bin
mv pinetime-graphic.bin 0.bin
```

Read the article ["Draw your own PineTime Watch Face... From WebAssembly to Embedded Rust"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/handdrawn)

## PineTime Bootloader

To convert the file `pinetime-graphic.png` (240 x 240 resolution) to an RGB565 binary file `pinetime-graphic.bin`...

```bash
# Download the source code
git clone https://github.com/lupyuen/pinetime-graphic
cd pinetime-graphic

# TODO: Copy pinetime-graphic.png to the pinetime-graphic folder

# Convert the PNG file to a C array
cargo run -v pinetime-graphic.png >pinetime-graphic.inc

# Compile the C array
gcc -o export-bin export-bin.c

# Export the C array as an RGB565 binary file pinetime-graphic.bin
./export-bin
```

To convert binary file `pinetime-graphic.bin` to HEX file `pinetime-graphic.hex`...

```bash
arm-none-eabi-objcopy -I binary -O ihex pinetime-graphic.bin pinetime-graphic.hex
```

Read the article ["MCUBoot Bootloader for PineTime Smart Watch (nRF52)"](https://lupyuen.github.io/pinetime-rust-mynewt/articles/mcuboot)
