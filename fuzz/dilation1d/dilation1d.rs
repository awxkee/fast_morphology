#![no_main]

use fast_morphology::{
    morph_1d_f32, morphology_rgb, BorderMode, ImageSize, KernelShape, MorphExOp, MorphScalar,
    MorphologyThreadingPolicy,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (f32, u8, u8, u8)| {
    let border_mode = match data.3 % 5 {
        0 => BorderMode::Clamp,
        1 => BorderMode::Wrap,
        2 => BorderMode::Reflect,
        3 => BorderMode::Reflect101,
        _ => BorderMode::Constant,
    };
    fuzz_1d(data.0, data.1 as usize, data.2 as usize, border_mode);
});

fn fuzz_1d(value: f32, width: usize, kernel_width: usize, border_mode: BorderMode) {
    if width == 0 || kernel_width == 0 || kernel_width % 2 == 0 {
        return;
    }
    if kernel_width > 150 {
        return;
    }
    let mut se_element = vec![0u8; kernel_width];
    let se_length = se_element.len();
    se_element[0] = 1;
    se_element[kernel_width - 1] = 1;
    se_element[se_length - kernel_width] = 1;
    se_element[se_length.saturating_sub(kernel_width).saturating_sub(1)] = 1;
    se_element[se_length - 1] = 1;
    let mut dst = vec![0.; width];
    let src = vec![value; width];
    morph_1d_f32(
        &src,
        &mut dst,
        MorphExOp::Dilate,
        &se_element,
        border_mode,
        MorphScalar::default(),
    )
    .unwrap();
}
