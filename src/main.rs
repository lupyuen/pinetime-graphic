#![feature(rustc_private)]
#![allow(non_upper_case_globals)]

extern crate getopts;
extern crate glob;
extern crate png;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use getopts::{Matches, Options, ParsingStyle};
use term::{color, Attr};

fn parse_args() -> Matches {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("c", "", "colorize output (for ANSI terminals)")
        .optflag("q", "", "test quietly (output only errors)")
        //.optflag("t", "", "print contents of tEXt chunks (can be used with -q)");
        .optflag("v", "", "test verbosely (print most chunk data)")
        .optopt("", "min", "minimum threshold (0-255, for black and white)", "0")
        .optopt("", "max", "maximum threshold (0-255, for black and white)", "128")
        .parsing_style(ParsingStyle::StopAtFirstFree);
    if args.len() > 1 {
        match opts.parse(&args[1..]) {
            Ok(matches) => return matches,
            Err(err) => println!("{}", err),
        }
    }
    println!(
        "{}",
        opts.usage(&format!("Usage: pngcheck [-cpt] [file ...]"))
    );
    std::process::exit(0);
}

#[derive(Clone)]
struct Config {
    quiet: bool,
    verbose: bool,
    color: bool,
    min: String,
    max: String,
}

fn display_interlaced(i: bool) -> &'static str {
    if i {
        "interlaced"
    } else {
        "non-interlaced"
    }
}

fn display_image_type(bits: u8, color: png::ColorType) -> String {
    use png::ColorType::*;
    format!(
        "{}-bit {}",
        bits,
        match color {
            Grayscale => "grayscale",
            RGB => "RGB",
            Indexed => "palette",
            GrayscaleAlpha => "grayscale+alpha",
            RGBA => "RGB+alpha",
        }
    )
}
// channels after expansion of tRNS
fn final_channels(c: png::ColorType, trns: bool) -> u8 {
    use png::ColorType::*;
    match c {
        Grayscale => 1 + if trns { 1 } else { 0 },
        RGB => 3,
        Indexed => 3 + if trns { 1 } else { 0 },
        GrayscaleAlpha => 2,
        RGBA => 4,
    }
}

static mut ROW_COUNT: usize = 240;
static mut COL_COUNT: usize = 240;
const BYTES_PER_PIXEL: usize = 3;
const BYTES_PER_LINE: usize = 16;

/// Dump image as RGB565 or black and white
fn dump_image<P: AsRef<Path>>(c: Config, fname: P) -> io::Result<()> {
    if c.min.len() == 0 && c.max.len() == 0 {
        //  If min and max not specified: Dump as RGB565
        dump_image_rgb565(c, fname)
    } else {
        //  If min and max are specified: Dump as black and white
        dump_image_bw(c, fname)
    }
}

/// Dump image as black and white. Set the min/max thresholds to dump the black and red bitmaps...
/// cargo run -- --min 0  --max 85  uart-cartoon2.png >image_black.inc
/// cargo run -- --min 86 --max 215 uart-cartoon2.png >image_red.inc
fn dump_image_bw<P: AsRef<Path>>(c: Config, fname: P) -> io::Result<()> {
    println!("//  Min: {}, Max: {}", c.min, c.max);
    let min: u32 = c.min.parse().unwrap();
    let max: u32 = c.max.parse().unwrap();

    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::EXPAND
    // | Transformations::STRIP_ALPHA`.
    let decoder = png::Decoder::new(File::open(fname).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(&mut buf).unwrap();
    println!("//  Rows: {}, Columns: {}, Buffer Size: {}", unsafe { ROW_COUNT }, unsafe { COL_COUNT }, info.buffer_size());

    let mut byte_count = 0;
    let mut bit_count = 0;
    let mut byte: u8 = 0;
    //  Dump each column
    for col in 0..unsafe { COL_COUNT } {
        //  Flip the columns
        let col = unsafe{ COL_COUNT } - col - 1;
        //  Dump each row
        for row in 0..unsafe { ROW_COUNT } {
            let index = ((row * unsafe { COL_COUNT }) + col) * BYTES_PER_PIXEL;
            let r = buf[index] as u32;
            let g = buf[index + 1] as u32;
            let b = buf[index + 2] as u32;
            //  Set bit to 1 (pixel off) if RGB is outside min and max threshold
            if (r + g + b) < (min * 3)
                ||  (r + g + b) > (max * 3) {
                byte = byte | 1;
            }
            //  Shift the bit
            bit_count += 1;
            if bit_count % 8 > 0 {
                byte = byte << 1;
                continue;
            }

            //  Print the byte
            if byte_count % BYTES_PER_LINE > 0 { print!(" ") }
            print!("0x{:02x},", byte);
            byte = 0;
            byte_count += 1;
            if (byte_count + 1) % BYTES_PER_LINE == 0 { print!("\n"); }
        }
    }
    Ok(())
}

/// Dump image as rgb565
fn dump_image_rgb565<P: AsRef<Path>>(_c: Config, fname: P) -> io::Result<()> {
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::EXPAND
    // | Transformations::STRIP_ALPHA`.
    let decoder = png::Decoder::new(File::open(fname).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(&mut buf).unwrap();
    println!("//  Buffer Size: {}", info.buffer_size());
    let mut count = 0;
    for row in 0..unsafe { ROW_COUNT } {
        for col in 0..unsafe { COL_COUNT } {
            let index = ((row * unsafe{ COL_COUNT}) + col) * BYTES_PER_PIXEL;
            let r = buf[index];
            let g = buf[index + 1];
            let b = buf[index + 2];
            //  Write out RRRRRGGG GGGBBBBB
            let byte1: u8 = 
                (r & 0b11111000) |  //  RRRRR000
                (g >> 5);           //  00000GGG
            let byte2: u8 = 
                ((g & 0b11100) << 3) |  //  GGG00000
                (b >> 3);               //  000BBBBB
            if count > 0 { print!(" ") }
            print!("0x{:02x}, 0x{:02x},", byte1, byte2);
            count += 2;
            if count >= BYTES_PER_LINE {
                count = 0;
                print!("\n");
            }
        }
    }
    Ok(())
}

fn check_image<P: AsRef<Path>>(c: Config, fname: P) -> io::Result<()> {
    // TODO improve performance by resusing allocations from decoder
    use png::Decoded::*;
    let mut t = term::stdout().ok_or(io::Error::new(
        io::ErrorKind::Other,
        "could not open terminal",
    ))?;
    let data = &mut vec![0; 10 * 1024][..];
    let mut reader = io::BufReader::new(File::open(&fname)?);
    let fname = fname.as_ref().to_string_lossy();
    let n = reader.read(data)?;
    let mut buf = &data[..n];
    let mut pos = 0;
    let mut decoder = png::StreamingDecoder::new();
    // Image data
    let mut width = 0;
    let mut height = 0;
    let mut color = png::ColorType::Grayscale;
    let mut bits = 0;
    let mut trns = false;
    let mut interlaced = false;
    let mut compressed_size = 0;
    let mut n_chunks = 0;
    let mut have_idat = false;
    macro_rules! c_ratio(
        // TODO add palette entries to compressed_size
        () => ({
            compressed_size as f32/(
                height as u64 *
                (width as u64 * final_channels(color, trns) as u64 * bits as u64 + 7)>>3
            ) as f32
        });
    );
    let display_error = |err| -> Result<_, io::Error> {
        let mut t = term::stdout().ok_or(io::Error::new(
            io::ErrorKind::Other,
            "could not open terminal",
        ))?;
        if c.verbose {
            if c.color {
                print!(": ");
                t.fg(color::RED)?;
                writeln!(t, "{}", err)?;
                t.attr(Attr::Bold)?;
                write!(t, "ERRORS DETECTED")?;
                t.reset()?;
            } else {
                println!(": {}", err);
                print!("ERRORS DETECTED")
            }
            println!(" in {}", fname);
        } else {
            if !c.quiet {
                if c.color {
                    t.fg(color::RED)?;
                    t.attr(Attr::Bold)?;
                    write!(t, "ERROR")?;
                    t.reset()?;
                    write!(t, ": ")?;
                    t.fg(color::YELLOW)?;
                    writeln!(t, "{}", fname)?;
                    t.reset()?;
                } else {
                    println!("ERROR: {}", fname)
                }
            }
            print!("{}: ", fname);
            if c.color {
                t.fg(color::RED)?;
                writeln!(t, "{}", err)?;
                t.reset()?;
            } else {
                println!("{}", err);
            }
        }
        Ok(())
    };

    if c.verbose {
        print!("//  File: ");
        if c.color {
            t.attr(Attr::Bold)?;
            write!(t, "{}", fname)?;
            t.reset()?;
        } else {
            print!("{}", fname);
        }
        print!(" ({}) bytes", data.len())
    }
    loop {
        if buf.len() == 0 {
            // circumvent borrow checker
            assert!(!data.is_empty());
            let n = reader.read(data)?;

            // EOF
            if n == 0 {
                println!("ERROR: premature end of file {}", fname);
                break;
            }
            buf = &data[..n];
        }
        match decoder.update(buf, &mut Vec::new()) {
            Ok((_, ImageEnd)) => {
                if !have_idat {
                    display_error(png::DecodingError::Format("IDAT chunk missing".into()))?;
                    break;
                }
                if !c.verbose && !c.quiet {
                    if c.color {
                        t.fg(color::GREEN)?;
                        t.attr(Attr::Bold)?;
                        write!(t, "OK")?;
                        t.reset()?;
                        write!(t, ": ")?;
                        t.fg(color::YELLOW)?;
                        write!(t, "{}", fname)?;
                        t.reset()?;
                    } else {
                        print!("//  OK: {}", fname)
                    }
                    println!(
                        " ({}x{}, {}{}, {}, {:.1}%)",
                        width,
                        height,
                        display_image_type(bits, color),
                        (if trns { "+trns" } else { "" }),
                        display_interlaced(interlaced),
                        100.0 * (1.0 - c_ratio!())
                    )
                } else if !c.quiet {
                    println!("");
                    if c.color {
                        t.fg(color::GREEN)?;
                        t.attr(Attr::Bold)?;
                        write!(t, "No errors detected ")?;
                        t.reset()?;
                    } else {
                        print!("//  No errors detected ");
                    }
                    println!(
                        "in {} ({} chunks, {:.1}% compression)",
                        fname,
                        n_chunks,
                        100.0 * (1.0 - c_ratio!())
                    )
                }
                break;
            }
            Ok((n, res)) => {
                buf = &buf[n..];
                pos += n;
                match res {
                    Header(w, h, b, c, i) => {
                        width = w;
                        height = h;
                        bits = b as u8;
                        color = c;
                        interlaced = i;
                    }
                    ChunkBegin(len, type_str) => {
                        use png::chunk;
                        n_chunks += 1;
                        if c.verbose {
                            let chunk = String::from_utf8_lossy(&type_str);
                            println!("");
                            print!("//    chunk ");
                            if c.color {
                                t.fg(color::YELLOW)?;
                                write!(t, "{}", chunk)?;
                                t.reset()?;
                            } else {
                                print!("{}", chunk)
                            }
                            print!(
                                " at offset {:#07x}, length {}",
                                pos - 4, // substract chunk name length
                                len
                            )
                        }
                        match type_str {
                            chunk::IDAT => {
                                have_idat = true;
                                compressed_size += len
                            }
                            chunk::tRNS => {
                                trns = true;
                            }
                            _ => (),
                        }
                    }
                    ImageData => {
                        //  println!("got {} bytes of image data", data.len())
                    }
                    ChunkComplete(_, type_str) if c.verbose => {
                        use png::chunk::*;
                        match type_str {
                            IHDR => {
                                unsafe { 
                                    COL_COUNT = width as usize; 
                                    ROW_COUNT = height as usize;
                                }
                                println!("");
                                print!(
                                    "//      {} x {} image, {}{}, {}",
                                    width,
                                    height,
                                    display_image_type(bits, color),
                                    (if trns { "+trns" } else { "" }),
                                    display_interlaced(interlaced),
                                );
                            }
                            _ => (),
                        }
                    }
                    AnimationControl(actl) => {
                        println!("");
                        print!("//      {} frames, {} plays", actl.num_frames, actl.num_plays,);
                    }
                    FrameControl(fctl) => {
                        println!("");
                        println!(
                            "    sequence #{}, {} x {} pixels @ ({}, {})",
                            fctl.sequence_number,
                            fctl.width,
                            fctl.height,
                            fctl.x_offset,
                            fctl.y_offset,
                            /*fctl.delay_num,
                            fctl.delay_den,
                            fctl.dispose_op,
                            fctl.blend_op,*/
                        );
                        print!(
                            "//      {}/{} s delay, dispose: {}, blend: {}",
                            fctl.delay_num,
                            if fctl.delay_den == 0 {
                                100
                            } else {
                                fctl.delay_den
                            },
                            fctl.dispose_op,
                            fctl.blend_op,
                        );
                    }
                    _ => (),
                }
                //println!("{} {:?}", n, res)
            }
            Err(err) => {
                let _ = display_error(err);
                break;
            }
        }
    }
    Ok(())
}

fn main() {
    let m = parse_args();

    let config = Config {
        quiet: false,  //  m.opt_present("q"),
        verbose: true, //  m.opt_present("v"),
        color: false,  //  m.opt_present("c"),
        min: m.opt_str("min").unwrap_or("".to_string()),
        max: m.opt_str("max").unwrap_or("".to_string()),
    };

    for file in m.free {
        let result = if file.contains("*") {
            glob::glob(&file)
                .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
                .and_then(|mut glob| {
                    glob.try_for_each(|entry| {
                        entry
                            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
                            .and_then(|file| {
                                check_image(config.clone(), &file).unwrap();
                                dump_image(config.clone(), &file)
                        })
                    })
                })
        } else {
            check_image(config.clone(), &file).unwrap();
            dump_image(config.clone(), &file)
        };

        result.unwrap_or_else(|err| {
            println!("{}: {}", file, err);
            std::process::exit(1)
        });
    }
}