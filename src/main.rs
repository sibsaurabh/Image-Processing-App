fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let blurr = args.remove(0);
            let blur_percent: f32 = blurr.parse().expect("Failed to parse a number");
            let infile = args.remove(0);
            let outfile = args.remove(0);
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            blur(infile, outfile, blur_percent);
        }
        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let bright = args.remove(0);
            let bright_percent: i32 = bright.parse().expect("Failed to parse a number");
            let infile = args.remove(0);
            let outfile = args.remove(0);
            brighten(infile, outfile, bright_percent);
        }
        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let x = args.remove(0).parse().expect("Failed to parse a number");
            let y = args.remove(0).parse().expect("Failed to parse a number");
            let width = args.remove(0).parse().expect("Failed to parse a number");
            let height = args.remove(0).parse().expect("Failed to parse a number");
            let infile = args.remove(0);
            let outfile = args.remove(0);
            crop(infile, outfile, x, y, width, height);
        }
        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let rotation = args.remove(0);
            let infile = args.remove(0);
            let outfile = args.remove(0);
            rotate(infile, outfile, rotation);
        }
        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            invert(infile, outfile);
        }
        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            grayscale(infile, outfile);
        }
        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }
        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, blur_percent: f32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(blur_percent);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, bright_percent: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.brighten(bright_percent);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.crop(x, y, width, height);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String, rotation: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    if rotation == "90" {
        img = img.rotate90();
    } else if rotation == "180" {
        img = img.rotate180();
    } else if rotation == "270" {
        img = img.rotate270();
    } else {
        print_usage_and_exit();
    }
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.grayscale();
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

// fn generate(outfile: String) {
//     // Create an ImageBuffer -- see fractal() for an example

//     // Iterate over the coordinates and pixels of the image -- see fractal() for an example

//     // Set the image to some solid color. -- see fractal() for an example

//     // Challenge: parse some color data from the command-line, pass it through
//     // to this function to use for the solid color.

//     // Challenge 2: Generate something more interesting!

//     // See blur() for an example of how to save the image
// }

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
