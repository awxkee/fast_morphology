# Fast morphology in pure Rust

This crate provides fast 2D arbitrary shaped structuring element for planar, RGB and RGBA images.
In most cases performance stays close to OpenCV, for some shapes on larger kernels works faster than OpenCV.
For small kernels OpenCV performs faster.

# Usage example

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

## Results

Here is some examply bokeh effect

<p float="left">
    <img src="https://github.com/awxkee/fast_morphology/blob/master/assets/fruits.jpg?raw=true" width="273" height="409">
    <img src="https://github.com/awxkee/fast_morphology/blob/master/assets/bokeh.jpg?raw=true" width="273" height="409">
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
| FM     | 41.85ms | 71.51ms | 90.88ms  | 213.45ms | 377.64ms |
| OpenCV | 20.65ms | 54.43ms | 107.58ms | 418.66ms | 905.21ms |

M3 Pro, NEON dilation RGBA image 2731x4096 with specified kernel size

| SE     | 9x9     | 15x15   | 21x21    | 41x41    | 61x61    |
|--------|---------|---------|----------|----------|----------|
| FM     | 47.47ms | 84.64ms | 111.79ms | 274.01ms | 515.54ms |
| OpenCV | 30.22ms | 72.63ms | 138.69ms | 555.51ms | 1.19s    |

This project is licensed under either of

- BSD-3-Clause License (see [LICENSE](LICENSE.md))
- Apache License, Version 2.0 (see [LICENSE](LICENSE-APACHE.md))

at your option.