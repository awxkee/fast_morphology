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
use crate::border_mode::MorphScalar;
use crate::op_type::MorphExOp;
use crate::{
    morphology, morphology_gray_alpha_u16, morphology_gray_u16, morphology_rgb, morphology_rgb_f32,
    morphology_rgb_u16, morphology_rgba, morphology_rgba_f32, morphology_rgba_u16, BorderMode,
    ImageSize, KernelShape, MorphologyThreadingPolicy,
};
use image::{
    DynamicImage, GrayAlphaImage, GrayImage, ImageBuffer, Luma, LumaA, Rgb, Rgb32FImage, RgbImage,
    Rgba, Rgba32FImage, RgbaImage,
};

/// Performs morphology on image
///
/// # Arguments
///
/// * `image`: Image from image crate
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morphology_image(
    image: DynamicImage,
    morph_op: MorphExOp,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<DynamicImage, String> {
    match image {
        DynamicImage::ImageLuma8(plane) => {
            match morph_gray_image(
                plane,
                morph_op,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            ) {
                Ok(img) => Ok(DynamicImage::from(img)),
                Err(err) => Err(err),
            }
        }
        DynamicImage::ImageLumaA8(plane_with_alpha) => {
            match morph_gray_alpha_image(
                plane_with_alpha,
                morph_op,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            ) {
                Ok(img) => Ok(DynamicImage::from(img)),
                Err(err) => Err(err),
            }
        }
        DynamicImage::ImageRgb8(rgb_image) => {
            match morph_rgb_image(
                rgb_image,
                morph_op,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            ) {
                Ok(img) => Ok(DynamicImage::from(img)),
                Err(err) => Err(err),
            }
        }
        DynamicImage::ImageRgba8(rgba_image) => {
            match morph_rgba_image(
                rgba_image,
                morph_op,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            ) {
                Ok(img) => Ok(DynamicImage::from(img)),
                Err(err) => Err(err),
            }
        }
        DynamicImage::ImageLuma16(gray_16) => {
            match morph_gray_16_image(
                gray_16,
                morph_op,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            ) {
                Ok(img) => Ok(DynamicImage::from(img)),
                Err(err) => Err(err),
            }
        }
        DynamicImage::ImageLumaA16(gray_16_with_alpha) => {
            match morph_gray_alpha_16_image(
                gray_16_with_alpha,
                morph_op,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            ) {
                Ok(img) => Ok(DynamicImage::from(img)),
                Err(err) => Err(err),
            }
        }
        DynamicImage::ImageRgb16(rgb_16) => {
            match morph_rgb_16_image(
                rgb_16,
                morph_op,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            ) {
                Ok(img) => Ok(DynamicImage::from(img)),
                Err(err) => Err(err),
            }
        }
        DynamicImage::ImageRgba16(rgba_16) => {
            match morph_rgba_16_image(
                rgba_16,
                morph_op,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            ) {
                Ok(img) => Ok(DynamicImage::from(img)),
                Err(err) => Err(err),
            }
        }
        DynamicImage::ImageRgb32F(rgb_f32) => {
            match morph_rgb_f32_image(
                rgb_f32,
                morph_op,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            ) {
                Ok(img) => Ok(DynamicImage::from(img)),
                Err(err) => Err(err),
            }
        }
        DynamicImage::ImageRgba32F(rgba_f32) => {
            match morph_rgba_f32_image(
                rgba_f32,
                morph_op,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            ) {
                Ok(img) => Ok(DynamicImage::from(img)),
                Err(err) => Err(err),
            }
        }
        _ => Err("This type is not implemented.".parse().unwrap()),
    }
}

/// Performs morphology on image
///
/// # Arguments
///
/// * `image`: Image from image crate
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morph_gray_image(
    image: GrayImage,
    morph_op: MorphExOp,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<GrayImage, String> {
    let bytes = image.as_raw();
    let mut dst_bytes = vec![0u8; bytes.len()];
    let size = ImageSize::new(image.dimensions().0 as usize, image.dimensions().1 as usize);
    morphology(
        bytes,
        &mut dst_bytes,
        morph_op,
        size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )?;
    if let Some(img) = GrayImage::from_raw(size.width as u32, size.height as u32, dst_bytes) {
        Ok(img)
    } else {
        Err("Can't create a Gray Image".parse().unwrap())
    }
}

/// Performs morphology on image
///
/// # Arguments
///
/// * `image`: Image from image crate
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morph_rgb_image(
    image: RgbImage,
    morph_op: MorphExOp,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<RgbImage, String> {
    let bytes = image.as_raw();
    let mut dst_bytes = vec![0u8; bytes.len()];
    let size = ImageSize::new(image.dimensions().0 as usize, image.dimensions().1 as usize);
    morphology_rgb(
        bytes,
        &mut dst_bytes,
        morph_op,
        size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )?;
    if let Some(img) = RgbImage::from_raw(size.width as u32, size.height as u32, dst_bytes) {
        Ok(img)
    } else {
        Err("Can't create a Rgb Image".parse().unwrap())
    }
}

/// Performs morphology on image
///
/// # Arguments
///
/// * `image`: Image from image crate
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morph_gray_alpha_image(
    image: GrayAlphaImage,
    morph_op: MorphExOp,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<GrayAlphaImage, String> {
    let bytes = image.as_raw();
    let mut dst_bytes = vec![0u8; bytes.len()];
    let size = ImageSize::new(image.dimensions().0 as usize, image.dimensions().1 as usize);
    morphology_rgb(
        bytes,
        &mut dst_bytes,
        morph_op,
        size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )?;
    if let Some(img) = GrayAlphaImage::from_raw(size.width as u32, size.height as u32, dst_bytes) {
        Ok(img)
    } else {
        Err("Can't create a Rgb Image".parse().unwrap())
    }
}

/// Performs morphology on image
///
/// # Arguments
///
/// * `image`: Image from image crate
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morph_rgba_image(
    image: RgbaImage,
    morph_op: MorphExOp,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<RgbaImage, String> {
    let bytes = image.as_raw();
    let mut dst_bytes = vec![0u8; bytes.len()];
    let size = ImageSize::new(image.dimensions().0 as usize, image.dimensions().1 as usize);
    morphology_rgba(
        bytes,
        &mut dst_bytes,
        morph_op,
        size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )?;
    if let Some(img) = RgbaImage::from_raw(size.width as u32, size.height as u32, dst_bytes) {
        Ok(img)
    } else {
        Err("Can't create a Rgb Image".parse().unwrap())
    }
}

/// Performs morphology on image
///
/// # Arguments
///
/// * `image`: Image from image crate
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morph_gray_alpha_16_image(
    image: ImageBuffer<LumaA<u16>, Vec<u16>>,
    morph_op: MorphExOp,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<ImageBuffer<LumaA<u16>, Vec<u16>>, String> {
    let bytes = image.as_raw();
    let mut dst_bytes = vec![0u16; bytes.len()];
    let size = ImageSize::new(image.dimensions().0 as usize, image.dimensions().1 as usize);
    morphology_gray_alpha_u16(
        bytes,
        &mut dst_bytes,
        morph_op,
        size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )?;
    if let Some(img) = ImageBuffer::<LumaA<u16>, Vec<u16>>::from_raw(
        size.width as u32,
        size.height as u32,
        dst_bytes,
    ) {
        Ok(img)
    } else {
        Err("Can't create a Rgb Image".parse().unwrap())
    }
}

/// Performs morphology on image
///
/// # Arguments
///
/// * `image`: Image from image crate
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morph_gray_16_image(
    image: ImageBuffer<Luma<u16>, Vec<u16>>,
    morph_op: MorphExOp,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<ImageBuffer<Luma<u16>, Vec<u16>>, String> {
    let bytes = image.as_raw();
    let mut dst_bytes = vec![0u16; bytes.len()];
    let size = ImageSize::new(image.dimensions().0 as usize, image.dimensions().1 as usize);
    morphology_gray_u16(
        bytes,
        &mut dst_bytes,
        morph_op,
        size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )?;
    if let Some(img) = ImageBuffer::<Luma<u16>, Vec<u16>>::from_raw(
        size.width as u32,
        size.height as u32,
        dst_bytes,
    ) {
        Ok(img)
    } else {
        Err("Can't create a Rgb Image".parse().unwrap())
    }
}

/// Performs morphology on image
///
/// # Arguments
///
/// * `image`: Image from image crate
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morph_rgb_16_image(
    image: ImageBuffer<Rgb<u16>, Vec<u16>>,
    morph_op: MorphExOp,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<ImageBuffer<Rgb<u16>, Vec<u16>>, String> {
    let bytes = image.as_raw();
    let mut dst_bytes = vec![0u16; bytes.len()];
    let size = ImageSize::new(image.dimensions().0 as usize, image.dimensions().1 as usize);
    morphology_rgb_u16(
        bytes,
        &mut dst_bytes,
        morph_op,
        size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )?;
    if let Some(img) = ImageBuffer::<Rgb<u16>, Vec<u16>>::from_raw(
        size.width as u32,
        size.height as u32,
        dst_bytes,
    ) {
        Ok(img)
    } else {
        Err("Can't create a Rgb Image".parse().unwrap())
    }
}

/// Performs morphology on image
///
/// # Arguments
///
/// * `image`: Image from image crate
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morph_rgba_16_image(
    image: ImageBuffer<Rgba<u16>, Vec<u16>>,
    morph_op: MorphExOp,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<ImageBuffer<Rgba<u16>, Vec<u16>>, String> {
    let bytes = image.as_raw();
    let mut dst_bytes = vec![0u16; bytes.len()];
    let size = ImageSize::new(image.dimensions().0 as usize, image.dimensions().1 as usize);
    morphology_rgba_u16(
        bytes,
        &mut dst_bytes,
        morph_op,
        size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )?;
    if let Some(img) = ImageBuffer::<Rgba<u16>, Vec<u16>>::from_raw(
        size.width as u32,
        size.height as u32,
        dst_bytes,
    ) {
        Ok(img)
    } else {
        Err("Can't create a Rgb Image".parse().unwrap())
    }
}

/// Performs morphology on image
///
/// # Arguments
///
/// * `image`: Image from image crate
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morph_rgba_f32_image(
    image: Rgba32FImage,
    morph_op: MorphExOp,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<Rgba32FImage, String> {
    let bytes = image.as_raw();
    let mut dst_bytes = vec![0f32; bytes.len()];
    let size = ImageSize::new(image.dimensions().0 as usize, image.dimensions().1 as usize);
    morphology_rgba_f32(
        bytes,
        &mut dst_bytes,
        morph_op,
        size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )?;
    if let Some(img) = Rgba32FImage::from_raw(size.width as u32, size.height as u32, dst_bytes) {
        Ok(img)
    } else {
        Err("Can't create a Rgb Image".parse().unwrap())
    }
}

/// Performs morphology on image
///
/// # Arguments
///
/// * `image`: Image from image crate
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morph_rgb_f32_image(
    image: Rgb32FImage,
    morph_op: MorphExOp,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<Rgb32FImage, String> {
    let bytes = image.as_raw();
    let mut dst_bytes = vec![0f32; bytes.len()];
    let size = ImageSize::new(image.dimensions().0 as usize, image.dimensions().1 as usize);
    morphology_rgb_f32(
        bytes,
        &mut dst_bytes,
        morph_op,
        size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )?;
    if let Some(img) = Rgb32FImage::from_raw(size.width as u32, size.height as u32, dst_bytes) {
        Ok(img)
    } else {
        Err("Can't create a Rgb Image".parse().unwrap())
    }
}
