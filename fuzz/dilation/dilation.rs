#![no_main]

use fast_morphology::{
    morphology_rgb, BorderMode, ImageSize, KernelShape, MorphExOp, MorphScalar,
    MorphologyThreadingPolicy,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (u8, u8, u8, u8, u8, u8)| {
    let border_mode = match data.5 % 5 {
        0 => BorderMode::Clamp,
        1 => BorderMode::Wrap,
        2 => BorderMode::Reflect,
        3 => BorderMode::Reflect101,
        _ => BorderMode::Constant,
    };
    fuzz_2d_rgb(
        data.0,
        data.1 as usize,
        data.2 as usize,
        data.3 as usize,
        data.4 as usize,
        border_mode,
    );
});

fn fuzz_2d_rgb(
    value: u8,
    width: usize,
    height: usize,
    kernel_width: usize,
    kernel_height: usize,
    border_mode: BorderMode,
) {
    if width == 0 || height == 0 || kernel_width == 0 || kernel_height == 0 {
        return;
    }
    if kernel_width > 55 || kernel_height > 55 {
        return;
    }
    let mut se_element = vec![0u8; kernel_width * kernel_height];
    let se_length = se_element.len();
    se_element[0] = 1;
    se_element[kernel_width - 1] = 1;
    se_element[se_length - kernel_width] = 1;
    se_element[se_length.saturating_sub(kernel_width).saturating_sub(1)] = 1;
    se_element[se_length - 1] = 1;
    let mut dst = vec![0u8; width * height * 3];
    let src = vec![value; width * height * 3];
    morphology_rgb(
        &src,
        &mut dst,
        MorphExOp::Closing,
        ImageSize::new(width, height),
        &se_element,
        KernelShape::new(kernel_width, kernel_height),
        border_mode,
        MorphScalar::default(),
        MorphologyThreadingPolicy::Single,
    )
    .unwrap();
}
