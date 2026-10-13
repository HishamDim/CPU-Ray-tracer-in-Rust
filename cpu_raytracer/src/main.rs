use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    // Image dimensions:
    let img_width: u16 = 256;
    let img_heigh: u16 = 256;

    // primative encodig vv
    // println!("P3\n{img_width} {img_heigh}\n255\n");

    // Smarter encoding

    // Creating file and moving ownership to the BufWriter obj output
    let file = File::create("gradient.ppm")?;
    let mut output = BufWriter::new(file);

    // header
    writeln!(output, "P3\n{img_width} {img_heigh}\n255\n")?;

    for j in 0..img_width {
        for i in 0..img_heigh {
            // creating intensities
            let strength_r: f32 = i as f32 / (img_width - 1) as f32;
            let strength_g: f32 = j as f32 / (img_heigh - 1) as f32;
            let strength_b: f32 = (i + j) as f32 / ((img_heigh + img_width) / 2) as f32;

            // normalizing to int between 0 and 255 (for rgb)
            // Why 255.999 instead of 255? .99 is truncated, ensures a value of 1 becomes 255
            let r: u8 = (strength_r * 255.99) as u8;
            let g: u8 = (strength_g * 255.99) as u8;
            let b: u8 = (strength_b * 255.99) as u8;

            // print; instead of printing and using cargo run --quiet | Out-File -Encoding ascii image.ppm, i use the below and write directly in file
            // println!("{r} {g} {b}")

            writeln!(output, "{r} {g} {b}")?;
        }
    }

    // explicitly flush buffer to catch errors
    output.flush()?;
    Ok(())
}
