# Image Processing

A simple image processing application with multiple options for processing the image. Two image files are included in the project root for the convenience: dyson.png and
pens.png, Feel free to use them or provide (or generate) your own images.

Following are the processing options available :-

- Brighten
- Blur
- Grayscale
- Rotation (90,180,270)
- Invert
- Crop

Also available are :-

- Fractal image generation
- Solid Color image generation

Documentation for the image library is here: https://github.com/image-rs/image/tree/master

NOTE :- Image processing is very CPU-intensive. Your program will run _noticeably_ faster if you
run it with the `--release` flag.

```
cargo run --release [ARG1 [ARG2]]
```

For example:

```
cargo run --release image.png blurred.png blur 7.5
```

Stackable commands example:-

```
cargo run --release pens.png output.png blur 27.5 invert rotate 180 brighten 30
```
