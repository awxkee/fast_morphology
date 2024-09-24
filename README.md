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
| FM     | 35.41ms | 68.53ms | 85.31ms  | 208.47ms | 377.04ms |
| OpenCV | 20.65ms | 54.43ms | 107.58ms | 418.66ms | 905.21ms |

M3 Pro, NEON dilation RGBA image 2731x4096 with specified kernel size

| SE     | 9x9     | 15x15   | 21x21    | 41x41    | 61x61    |
|--------|---------|---------|----------|----------|----------|
| FM     | 48.25ms | 81.12ms | 111.79ms | 274.01ms | 515.54ms |
| OpenCV | 30.22ms | 72.63ms | 138.69ms | 555.51ms | 1.19s    |

SSE dilation RGB image 2731x4096 with specified kernel size

| SE     | 9x9     | 15x15    | 21x21    | 41x41    | 61x61  |
|--------|---------|----------|----------|----------|--------|
| FM     | 84.19ms | 186.53ms | 254.70ms | 673.45ms | 1.37s  |
| OpenCV | 28.61ms | 62.43ms  | 114.80ms | 428.87ms | 1.16ms |

SSE dilation RGBA image 2731x4096 with specified kernel size

| SE     | 9x9      | 15x15    | 21x21    | 41x41    | 61x61  |
|--------|----------|----------|----------|----------|--------|
| FM     | 109.37ms | 229.11ms | 329.31ms | 981.48ms | 2.05ms |
| OpenCV | 39.20ms  | 76.09ms  | 149.12ms | 569.36ms | 1.33s  |


This project is licensed under either of

- BSD-3-Clause License (see [LICENSE](LICENSE.md))
- Apache License, Version 2.0 (see [LICENSE](LICENSE-APACHE.md))

at your option.