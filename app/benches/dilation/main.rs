/*
 * Copyright (c) Radzivon Bartoshyk. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1.  Redistributions of source code must retain the above copyright notice, this
 * list of conditions and the following disclaimer.
 *
 * 2.  Redistributions in binary form must reproduce the above copyright notice,
 * this list of conditions and the following disclaimer in the documentation
 * and/or other materials provided with the distribution.
 *
 * 3.  Neither the name of the copyright holder nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

use criterion::{criterion_group, criterion_main, Criterion};
use fast_morphology::{
    dilate, dilate_rgb, dilate_rgba, BorderMode, ImageSize, KernelShape, MorphologyThreadingPolicy,
};
use image::{EncodableLayout, GenericImageView, ImageReader};
use opencv::core::{
    Mat, MatTrait, Point, Scalar, BORDER_REPLICATE, CV_8U, CV_8UC1, CV_8UC3, CV_8UC4,
};
use opencv::imgproc;

pub fn circle_se(radius: usize) -> Vec<u8> {
    let center_x = radius;
    let center_y = radius;

    // Create a vector to represent the image, initialized with 0 (black)
    let full_size = (radius * 2usize) + 1;
    let mut image = vec![0u8; full_size * full_size];

    let full_radius = full_size as f32 / 2.;

    // Iterate through each pixel in the image
    for y in 0..full_size {
        for x in 0..full_size {
            // Calculate the distance from the center using the circle equation
            let dx = x as f32 - center_x as f32;
            let dy = y as f32 - center_y as f32;
            let distance = dx.hypot(dy);

            // If the distance is less than or equal to the radius, fill the pixel
            if distance <= full_radius {
                image[y * full_size + x] = 1; // White pixel
            }
        }
    }

    image
}

fn exec_bench_rgb(c: &mut Criterion, size: usize) {
    let img = ImageReader::open("../assets/fruits.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let dimensions = img.dimensions();
    let rgb_image = img.to_rgb8();
    let rgb_bytes = rgb_image.as_bytes();

    let radius_size_7 = size;
    let se_size_15 = radius_size_7 * 2 + 1;
    let structuring_element_15 = circle_se(radius_size_7);

    let mut kernel_15 = Mat::new_rows_cols_with_default(
        se_size_15 as i32,
        se_size_15 as i32,
        CV_8U,
        Scalar::new(0., 0., 0., 0.),
    )
    .unwrap();
    unsafe {
        for (index, &byte) in structuring_element_15.iter().enumerate() {
            kernel_15.data_mut().add(index).write(byte);
        }
    }

    c.bench_function(
        format!("FM, RGB Image dilation: SE {}x{}", se_size_15, se_size_15).as_str(),
        |b| {
            b.iter(|| {
                let mut dst_image = vec![0u8; dimensions.0 as usize * dimensions.1 as usize * 3];
                dilate_rgb(
                    rgb_bytes,
                    &mut dst_image,
                    ImageSize::new(dimensions.0 as usize, dimensions.1 as usize),
                    &structuring_element_15,
                    KernelShape::new(se_size_15, se_size_15),
                    BorderMode::default(),
                    MorphologyThreadingPolicy::Adaptive,
                )
                .unwrap();
            })
        },
    );

    let mut mat = Mat::new_rows_cols_with_default(
        dimensions.1 as i32,
        dimensions.0 as i32,
        CV_8UC3,
        Scalar::new(0., 0., 0., 0.),
    )
    .unwrap();
    unsafe {
        for (index, &byte) in rgb_bytes.iter().enumerate() {
            mat.data_mut().add(index).write(byte);
        }
    }

    c.bench_function(
        format!(
            "OpenCV, RGB Image dilation: SE {}x{}",
            se_size_15, se_size_15
        )
        .as_str(),
        |b| {
            b.iter(|| {
                let mut dst_mat = Mat::default();
                imgproc::dilate(
                    &mat,
                    &mut dst_mat,
                    &kernel_15,
                    Point::new(-1, -1),
                    1,
                    BORDER_REPLICATE,
                    Scalar::new(0., 0., 0., 0.),
                )
                .unwrap();
            })
        },
    );
}

fn exec_bench_gray(c: &mut Criterion, size: usize) {
    let img = ImageReader::open("../assets/fruits.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let dimensions = img.dimensions();
    let rgb_image = img.to_luma8();
    let rgb_bytes = rgb_image.as_bytes();

    let radius_size_7 = size;
    let se_size_15 = radius_size_7 * 2 + 1;
    let structuring_element_15 = circle_se(radius_size_7);

    let mut kernel_15 = Mat::new_rows_cols_with_default(
        se_size_15 as i32,
        se_size_15 as i32,
        CV_8U,
        Scalar::new(0., 0., 0., 0.),
    )
    .unwrap();
    unsafe {
        for (index, &byte) in structuring_element_15.iter().enumerate() {
            kernel_15.data_mut().add(index).write(byte);
        }
    }

    c.bench_function(
        format!("FM, Gray Image dilation: SE {}x{}", se_size_15, se_size_15).as_str(),
        |b| {
            b.iter(|| {
                let mut dst_image = vec![0u8; dimensions.0 as usize * dimensions.1 as usize];
                dilate(
                    rgb_bytes,
                    &mut dst_image,
                    ImageSize::new(dimensions.0 as usize, dimensions.1 as usize),
                    &structuring_element_15,
                    KernelShape::new(se_size_15, se_size_15),
                    BorderMode::default(),
                    MorphologyThreadingPolicy::Adaptive,
                )
                .unwrap();
            })
        },
    );

    let mut mat = Mat::new_rows_cols_with_default(
        dimensions.1 as i32,
        dimensions.0 as i32,
        CV_8UC1,
        Scalar::new(0., 0., 0., 0.),
    )
    .unwrap();
    unsafe {
        for (index, &byte) in rgb_bytes.iter().enumerate() {
            mat.data_mut().add(index).write(byte);
        }
    }

    c.bench_function(
        format!(
            "OpenCV, Gray Image dilation: SE {}x{}",
            se_size_15, se_size_15
        )
        .as_str(),
        |b| {
            b.iter(|| {
                let mut dst_mat = Mat::default();
                imgproc::dilate(
                    &mat,
                    &mut dst_mat,
                    &kernel_15,
                    Point::new(-1, -1),
                    1,
                    BORDER_REPLICATE,
                    Scalar::new(0., 0., 0., 0.),
                )
                .unwrap();
            })
        },
    );
}

fn exec_bench_rgba(c: &mut Criterion, size: usize) {
    let img = ImageReader::open("../assets/fruits.jpg")
        .unwrap()
        .decode()
        .unwrap();
    let dimensions = img.dimensions();
    let rgb_image = img.to_rgba8();
    let rgb_bytes = rgb_image.as_bytes();

    let radius_size_7 = size;
    let se_size_15 = radius_size_7 * 2 + 1;
    let structuring_element_15 = circle_se(radius_size_7);

    let mut kernel_15 = Mat::new_rows_cols_with_default(
        se_size_15 as i32,
        se_size_15 as i32,
        CV_8U,
        Scalar::new(0., 0., 0., 0.),
    )
    .unwrap();
    unsafe {
        for (index, &byte) in structuring_element_15.iter().enumerate() {
            kernel_15.data_mut().add(index).write(byte);
        }
    }

    c.bench_function(
        format!("FM, RGBA Image dilation: SE {}x{}", se_size_15, se_size_15).as_str(),
        |b| {
            b.iter(|| {
                let mut dst_image = vec![0u8; dimensions.0 as usize * dimensions.1 as usize * 4];
                dilate_rgba(
                    rgb_bytes,
                    &mut dst_image,
                    ImageSize::new(dimensions.0 as usize, dimensions.1 as usize),
                    &structuring_element_15,
                    KernelShape::new(se_size_15, se_size_15),
                    BorderMode::default(),
                    MorphologyThreadingPolicy::Adaptive,
                )
                .unwrap();
            })
        },
    );

    let mut mat = Mat::new_rows_cols_with_default(
        dimensions.1 as i32,
        dimensions.0 as i32,
        CV_8UC4,
        Scalar::new(0., 0., 0., 0.),
    )
    .unwrap();
    unsafe {
        for (index, &byte) in rgb_bytes.iter().enumerate() {
            mat.data_mut().add(index).write(byte);
        }
    }

    c.bench_function(
        format!(
            "OpenCV, RGBA Image dilation: SE {}x{}",
            se_size_15, se_size_15
        )
        .as_str(),
        |b| {
            b.iter(|| {
                let mut dst_mat = Mat::default();
                imgproc::dilate(
                    &mat,
                    &mut dst_mat,
                    &kernel_15,
                    Point::new(-1, -1),
                    1,
                    BORDER_REPLICATE,
                    Scalar::new(0., 0., 0., 0.),
                )
                .unwrap();
            })
        },
    );
}

pub fn criterion_benchmark(c: &mut Criterion) {
    opencv::core::set_use_opencl(false).expect("Failed to disable OpenCL");
    opencv::core::set_use_ipp(false).expect("Failed to disable IPP");
    opencv::core::set_use_optimized(false).expect("Failed to disable opts");

    exec_bench_rgb(c, 4);
    exec_bench_rgb(c, 7);
    exec_bench_rgb(c, 10);
    exec_bench_rgb(c, 20);
    exec_bench_rgb(c, 30);

    exec_bench_rgba(c, 4);
    exec_bench_rgba(c, 7);
    exec_bench_rgba(c, 10);
    exec_bench_rgba(c, 20);
    exec_bench_rgba(c, 30);

    exec_bench_gray(c, 4);
    exec_bench_gray(c, 7);
    exec_bench_gray(c, 10);
    exec_bench_gray(c, 20);
    exec_bench_gray(c, 30);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
