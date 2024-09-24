use fast_morphology::{
    dilate, dilate_rgb, dilate_rgba, BorderMode, ImageSize, KernelShape, MorphologyThreadingPolicy,
};
use image::{EncodableLayout, GenericImageView, ImageReader};
use opencv::core::{
    Mat, MatTrait, MatTraitConstManual, Point, Scalar, BORDER_CONSTANT, BORDER_ISOLATED,
    BORDER_REPLICATE, CV_8U, CV_8UC3, CV_8UC4,
};
use opencv::imgproc;
use std::time::Instant;

fn circle_se(radius: usize) -> Vec<u8> {
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

fn gaussian_kernel(size: usize, sigma: f32) -> Vec<Vec<f32>> {
    let mut kernel = vec![vec![0.0; size]; size];
    let half_size = size as isize / 2;
    let sigma2 = 2f32 * sigma * sigma;
    let sigma_sqrt_2pi = 1.0 / (2.0 * std::f32::consts::PI * sigma * sigma);
    let mut sum = 0.0;

    // Fill in the kernel values
    for i in 0..size {
        for j in 0..size {
            let x = (i as isize - half_size) as f32;
            let y = (j as isize - half_size) as f32;
            let value = sigma_sqrt_2pi * (-((x * x + y * y) / sigma2)).exp();
            kernel[i][j] = value;
            sum += value;
        }
    }

    // Normalize the kernel so that the sum of all elements equals 1
    for i in 0..size {
        for j in 0..size {
            kernel[i][j] /= sum;
        }
    }

    kernel
}

fn main() {
    let radius_size = 5;
    let mut structuring_element = circle_se(radius_size);

    // opencv::core::set_use_opencl(false).expect("Failed to disable OpenCL");
    // opencv::core::set_use_ipp(true).expect("Failed to disable IPP");
    // opencv::core::set_use_optimized(false).expect("Failed to disable opts");

    let se_size = radius_size * 2 + 1;
    let full_size = se_size;
    for y in 0..full_size {
        let elements = &structuring_element[y * full_size..(y * full_size + full_size)];
        println!("{:?}", Vec::from(elements));
    }

    let img = ImageReader::open("./assets/ebelhard.jpg")
        .unwrap()
        .decode()
        .unwrap();

    let dimensions = img.dimensions();
    let transient = img.to_rgb8();
    let transient_rgba = img.to_rgba8();
    let mut bytes: Vec<u8> = Vec::from(transient.as_bytes());

    let mut channel_1_src = vec![0u8; dimensions.0 as usize * dimensions.1 as usize];
    let mut channel_2_src = vec![0u8; dimensions.0 as usize * dimensions.1 as usize];
    let mut channel_3_src = vec![0u8; dimensions.0 as usize * dimensions.1 as usize];

    let mut channel_1_dst = vec![0u8; dimensions.0 as usize * dimensions.1 as usize];
    let mut channel_2_dst = vec![0u8; dimensions.0 as usize * dimensions.1 as usize];
    let mut channel_3_dst = vec![0u8; dimensions.0 as usize * dimensions.1 as usize];

    for ((((a, dst_1), dst_2), dst_3)) in bytes
        .chunks_exact(3)
        .zip(&mut channel_1_src)
        .zip(&mut channel_2_src)
        .zip(&mut channel_3_src)
    {
        *dst_1 = a[0];
        *dst_2 = a[1];
        *dst_3 = a[2];
    }

    let border_mode = BorderMode::default();

    let exec_time = Instant::now();

    let image_size = ImageSize::new(dimensions.0 as usize, dimensions.1 as usize);

    dilate(
        &channel_1_src,
        &mut channel_1_dst,
        image_size,
        &structuring_element,
        KernelShape::new(se_size, se_size),
        border_mode,
        MorphologyThreadingPolicy::default(),
    )
    .unwrap();

    dilate(
        &channel_2_src,
        &mut channel_2_dst,
        image_size,
        &structuring_element,
        KernelShape::new(se_size, se_size),
        border_mode,
        MorphologyThreadingPolicy::default(),
    )
    .unwrap();

    dilate(
        &channel_3_src,
        &mut channel_3_dst,
        image_size,
        &structuring_element,
        KernelShape::new(se_size, se_size),
        border_mode,
        MorphologyThreadingPolicy::default(),
    )
    .unwrap();

    println!("exec time {:?}", exec_time.elapsed());

    let saved_origin = bytes.to_vec();

    for (((a, dst_1), dst_2), dst_3) in bytes
        .chunks_exact_mut(3)
        .zip(channel_1_dst)
        .zip(channel_2_dst)
        .zip(channel_3_dst)
    {
        a[0] = dst_1;
        a[1] = dst_2;
        a[2] = dst_3;
    }

    let rgba_image = transient_rgba.as_bytes();
    let mut dst = vec![0u8; saved_origin.len()];

    let exec_time = Instant::now();
    dilate_rgb(
        &saved_origin,
        &mut dst,
        image_size,
        &structuring_element,
        KernelShape::new(se_size, se_size),
        border_mode,
        MorphologyThreadingPolicy::default(),
    )
    .unwrap();

    println!("rgb exec time {:?}", exec_time.elapsed());

    // let mut mat = Mat::new_rows_cols_with_default(
    //     dimensions.1 as i32,
    //     dimensions.0 as i32,
    //     CV_8UC3,
    //     Scalar::new(0., 0., 0., 0.),
    // )
    // .unwrap();
    // unsafe {
    //     for (index, &byte) in saved_origin.iter().enumerate() {
    //         mat.data_mut().add(index).write(byte);
    //     }
    // }
    // let mut kernel = Mat::new_rows_cols_with_default(
    //     full_size as i32,
    //     full_size as i32,
    //     CV_8U,
    //     Scalar::new(0., 0., 0., 0.),
    // )
    // .unwrap();
    // unsafe {
    //     for (index, &byte) in structuring_element.iter().enumerate() {
    //         kernel.data_mut().add(index).write(byte);
    //     }
    // }

    let exec_time = Instant::now();

    // let mut dst_mat = Mat::default();
    // imgproc::dilate(
    //     &mat,
    //     &mut dst_mat,
    //     &kernel,
    //     Point::new(-1, -1),
    //     1,
    //     BORDER_REPLICATE,
    //     Scalar::new(0., 0., 0., 0.),
    // )
    // .unwrap();

    // let open_cv_bytes = dst_mat.data_bytes().unwrap();

    println!("opencv exec time {:?}", exec_time.elapsed());

    image::save_buffer(
        "converted.png",
        &bytes,
        dimensions.0,
        dimensions.1,
        image::ColorType::Rgb8,
    )
    .unwrap();

    image::save_buffer(
        "converted_rgb.png",
        &dst,
        dimensions.0,
        dimensions.1,
        image::ColorType::Rgb8,
    )
    .unwrap();

    // image::save_buffer(
    //     "converted_opencv.png",
    //     &open_cv_bytes,
    //     dimensions.0,
    //     dimensions.1,
    //     image::ColorType::Rgb8,
    // )
    // .unwrap();
}
