fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    // println!("{:?}", args);
    let mut flag: bool = false;
    let mut infile: String = String::new();
    let mut outfile: String = String::new();

    while !args.is_empty() {
        if !flag {
            infile = args.remove(0);
            outfile = args.remove(0);
            flag = true;
        }

        let subcommand = args.remove(0);
        match subcommand.as_str() {
            "blur" => {
                // if args.len() != 3 {
                //     print_usage_and_exit();
                // }
                let blurr = args.remove(0);
                let blur_percent: f32 = blurr.parse().expect("Failed to parse a number");
                // let infile = args.remove(0);
                // let outfile = args.remove(0);
                blur(&mut infile, outfile.clone(), blur_percent);
            }
            "brighten" => {
                // if args.len() != 3 {
                //     print_usage_and_exit();
                // }
                let bright = args.remove(0);
                let bright_percent: i32 = bright.parse().expect("Failed to parse a number");
                // let infile = args.remove(0);
                // let outfile = args.remove(0);
                brighten(&mut infile, outfile.clone(), bright_percent);
            }
            "crop" => {
                // if args.len() != 6 {
                //     print_usage_and_exit();
                // }
                let x = args.remove(0).parse().expect("Failed to parse a number");
                let y = args.remove(0).parse().expect("Failed to parse a number");
                let width = args.remove(0).parse().expect("Failed to parse a number");
                let height = args.remove(0).parse().expect("Failed to parse a number");
                // let infile = args.remove(0);
                // let outfile = args.remove(0);
                crop(&mut infile, outfile.clone(), x, y, width, height);
            }
            "rotate" => {
                // if args.len() != 3 {
                //     print_usage_and_exit();
                // }
                let rotation = args.remove(0);
                // let infile = args.remove(0);
                // let outfile = args.remove(0);
                rotate(&mut infile, outfile.clone(), rotation);
            }
            "invert" => {
                // if args.len() != 2 {
                //     print_usage_and_exit();
                // }
                // let infile = args.remove(0);
                // let outfile = args.remove(0);
                invert(&mut infile, outfile.clone());
            }
            "grayscale" => {
                // if args.len() != 2 {
                //     print_usage_and_exit();
                // }
                // let infile = args.remove(0);
                // let outfile = args.remove(0);
                grayscale(&mut infile, outfile.clone());
            }
            // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
            "fractal" => {
                if args.len() != 1 {
                    print_usage_and_exit();
                }
                // let outfile = args.remove(0);
                fractal(&outfile);
            }
            "generate" => {
                if args.len() != 4 {
                    print_usage_and_exit();
                }
                let red: u8 = args.remove(0).parse().expect("Failed to parse a number");
                let green: u8 = args.remove(0).parse().expect("Failed to parse a number");
                let blue: u8 = args.remove(0).parse().expect("Failed to parse a number");
                // let outfile = args.remove(0);
                generate(&outfile, red, blue, green);
            }
            // For everything else...
            _ => {
                print_usage_and_exit();
            }
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur blur_percent INFILE OUTFILE");
    println!("brighten bright_percent INFILE OUTFILE");
    println!("crop x y width height INFILE OUTFILE");
    println!("rotate rotation INFILE OUTFILE");
    println!("invert INFILE OUTFILE");
    println!("grayscale INFILE OUTFILE");
    println!("generate red blue green OUTFILE");
    println!("fractal OUTFILE");
    std::process::exit(-1);
}

fn blur(infile: &mut String, outfile: String, blur_percent: f32) {
    let img = image::open(infile.clone()).expect("Failed to open INFILE.");
    let img2 = img.blur(blur_percent);
    img2.save(outfile.clone()).expect("Failed writing OUTFILE.");
    // infile.push_str(&outfile);
    *infile = outfile;
}

fn brighten(infile: &mut String, outfile: String, bright_percent: i32) {
    let img = image::open(infile.clone()).expect("Failed to open INFILE.");
    let img2 = img.brighten(bright_percent);
    img2.save(outfile.clone()).expect("Failed writing OUTFILE.");
    *infile = outfile;
}

fn crop(infile: &mut String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let mut img = image::open(infile.clone()).expect("Failed to open INFILE.");
    let img2 = img.crop(x, y, width, height);
    img2.save(outfile.clone()).expect("Failed writing OUTFILE.");
    *infile = outfile;
}

fn rotate(infile: &mut String, outfile: String, rotation: String) {
    let mut img = image::open(infile.clone()).expect("Failed to open INFILE.");
    if rotation == "90" {
        img = img.rotate90();
    } else if rotation == "180" {
        img = img.rotate180();
    } else if rotation == "270" {
        img = img.rotate270();
    } else {
        print_usage_and_exit();
    }
    img.save(outfile.clone()).expect("Failed writing OUTFILE.");
    *infile = outfile;
}

fn invert(infile: &mut String, outfile: String) {
    let mut img = image::open(infile.clone()).expect("Failed to open INFILE.");
    img.invert();
    img.save(outfile.clone()).expect("Failed writing OUTFILE.");
    *infile = outfile;
}

fn grayscale(infile: &mut String, outfile: String) {
    let img = image::open(infile.clone()).expect("Failed to open INFILE.");
    let img2 = img.grayscale();
    img2.save(outfile.clone()).expect("Failed writing OUTFILE.");
    *infile = outfile;
}

fn generate(outfile: &String, red: u8, blue: u8, green: u8) {
    // Define the dimensions of the image
    let width = 800;
    let height = 600;

    // Create a new ImageBuffer with the specified dimensions
    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Define the solid color you want to set
    let solid_color = image::Rgb([red, blue, green]);

    // Iterate over the coordinates and pixels of the image
    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        // Set each pixel to the solid color
        *pixel = solid_color;
    }
    imgbuf.save(outfile).unwrap();
    // Challenge 2: Generate something more interesting!
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: &String) {
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
