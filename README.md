# Image Processing

A simple image processing application. Two image files are included in the project root for the convenience: dyson.png and pens.png
Feel free to use them or provide (or generate) your own images.

Following options are available :-

- Brighten
- Blur
- Grayscale
- Rotation (90,180,270)
- Invert
- Crop

Documentation for the image library is here: https://github.com/image-rs/image/tree/master

NOTE :- Image processing is very CPU-intensive. Your program will run _noticeably_ faster if you
run it with the `--release` flag.

```
cargo run --release [ARG1 [ARG2]]
```

For example:

```
cargo run --release blur image.png blurred.png
```
