//  Export the Bootloader Graphic to BIN format (pinetime-graphic.bin):
//  cargo run -v pinetime-graphic.png >pinetime-graphic.inc
//  gcc -o export-bin export-bin.c
//  ./export-bin
#include <stdio.h>
#include <assert.h>

static const char graphic_bytes[] = {
#include "pinetime-graphic.inc"
};

int main(void) {
    printf("BIN file size (should be %d): %lu\n", 
        240 * 240 * 2, 
        sizeof(graphic_bytes));
    FILE *f = fopen("pinetime-graphic.bin", "wb");
    size_t bytes_written = fwrite(graphic_bytes, 1, sizeof(graphic_bytes), f);
    fclose(f);

    assert(bytes_written == sizeof(graphic_bytes));
    puts("*** Done! Written to pinetime-graphic.bin");
    return 0;
}