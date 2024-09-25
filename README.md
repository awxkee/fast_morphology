# Fast morphology in pure Rust

This crate provides fast 2D arbitrary shaped structuring element for planar, RGB and RGBA images.
In most cases performance when implemented fully in hardware faster than OpenCV.

```rust
dilate_rgb(
    &src,
    &mut dst,
    ImageSize::new(500, 500),
    &structuring_element,
    KernelShape::new(15, 15),
    BorderMode::Clamp,
    MorphologyThreadingPolicy::default(),
).unwrap();
```

#### Usage with image crate

```rust
let img = ImageReader::open("./assets/fruits.jpg")
          .unwrap()
          .decode()
          .unwrap();
let new_image = morphology_image(
    img,
    MorphOp::Dilate,
    &structuring_element,
    KernelShape::new(se_size, se_size),
    BorderMode::default(),
    MorphologyThreadingPolicy::default(),
)
.unwrap();
new_image.save("dilated.jpg").unwrap();
```

## Results

Here is some examply bokeh effect

<p float="left">
    <img src="https://github.com/awxkee/fast_morphology/blob/master/assets/fruits.jpg?raw=true" width="273" height="409">
    <img src="https://github.com/awxkee/fast_morphology/blob/master/assets/bokeh.jpg?raw=true" width="273" height="409">
</p>

And erosion

<p float="left">
    <img src="https://github.com/awxkee/fast_morphology/blob/master/assets/fruits.jpg?raw=true" width="273" height="409">
    <img src="https://github.com/awxkee/fast_morphology/blob/master/assets/erosion.jpg?raw=true" width="273" height="409">
</p>

# Benchmarking

If you wish to run benchmarks then

```bash
cargo bench --manifest-path ./app/Cargo.toml
```

FM is shorthand for fast-morphology

M3 Pro, NEON dilation RGB image 2731x4096 with specified kernel size

| SE     | 9x9     | 15x15   | 21x21    | 41x41    | 61x61    |
|--------|---------|---------|----------|----------|----------|
| FM     | 16.81ms | 17.99ms | 24.53ms  | 69.00ms  | 142.76ms |
| OpenCV | 20.65ms | 54.43ms | 107.58ms | 418.66ms | 905.21ms |

M3 Pro, NEON dilation RGBA image 2731x4096 with specified kernel size

| SE     | 9x9     | 15x15   | 21x21    | 41x41    | 61x61    |
|--------|---------|---------|----------|----------|----------|
| FM     | 21.35ms | 27.20ms | 36.31ms  | 93.81ms  | 191.31ms |
| OpenCV | 30.22ms | 72.63ms | 138.69ms | 555.51ms | 1.19s    |

SSE dilation RGB image 2731x4096 with specified kernel size

| SE     | 9x9     | 15x15   | 21x21    | 41x41    | 61x61    |
|--------|---------|---------|----------|----------|----------|
| FM     | 30.71ms | 34.87ms | 39.93ms  | 81.56ms  | 149.37ms |
| OpenCV | 27.36ms | 63.05ms | 112.54ms | 419.40ms | 1.08s    |

SSE dilation RGBA image 2731x4096 with specified kernel size

| SE     | 9x9     | 15x15   | 21x21    | 41x41    | 61x61    |
|--------|---------|---------|----------|----------|----------|
| FM     | 45.03ms | 49.03ms | 56.40ms  | 114.72ms | 206.05ms |
| OpenCV | 35.50ms | 79.60ms | 147.32ms | 556.56ms | 1.33s    |

This project is licensed under either of

- BSD-3-Clause License (see [LICENSE](LICENSE.md))
- Apache License, Version 2.0 (see [LICENSE](LICENSE-APACHE.md))

at your option.