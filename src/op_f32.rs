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
use crate::border_mode::BorderMode;
use crate::morph_rgb::make_morphology_rgb;
use crate::morph_rgba::make_morphology_rgba;
use crate::op_impl::make_morphology;
use crate::op_type::MorphOp;
use crate::structuring_element::KernelShape;
use crate::{ImageSize, MorphologyThreadingPolicy};
use crate::morph_gray_alpha::make_morphology_gray_alpha;

/// Dilate a gray (planar) stored in u16 image
///
/// # Arguments
///
/// * `src`: Source image slice
/// * `dst`: Destination image slice
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `threading_policy`: Threads usage policy
///
pub fn dilate_f32(
    src: &[f32],
    dst: &mut [f32],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology::<f32, { MorphOp::Dilate as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            threading_policy,
        )
    }
}

/// Dilate an RGB stored in u16 image
///
/// # Arguments
///
/// * `src`: Source slice with RGB data
/// * `dst`: Destination slice for RGB data
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `threading_policy`: Threads usage policy
///
pub fn dilate_rgb_f32(
    src: &[f32],
    dst: &mut [f32],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology_rgb::<f32, { MorphOp::Dilate as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            threading_policy,
        )
    }
}

/// Erode a gray (planar) stored in u16 image
///
/// # Arguments
///
/// * `src`: Source image slice
/// * `dst`: Destination image slice
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `threading_policy`: Threads usage policy
///
pub fn erode_f32(
    src: &[f32],
    dst: &mut [f32],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology::<f32, { MorphOp::Erode as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            threading_policy,
        )
    }
}

/// Erode an RGB image stored in u16
///
/// # Arguments
///
/// * `src`: Source slice with RGB data
/// * `dst`: Destination slice for RGB data
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `threading_policy`: Threads usage policy
///
pub fn erode_rgb_f32(
    src: &[f32],
    dst: &mut [f32],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology_rgb::<f32, { MorphOp::Erode as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            threading_policy,
        )
    }
}

/// Erode an RGBA image stored in u16
///
/// # Arguments
///
/// * `src`: Source slice with RGBA data
/// * `dst`: Destination slice for RGBA data
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `threading_policy`: Threads usage policy
///
pub fn erode_rgba_f32(
    src: &[f32],
    dst: &mut [f32],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology_rgba::<f32, { MorphOp::Erode as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            threading_policy,
        )
    }
}

/// Dilate an RGBA image stored in u16
///
/// # Arguments
///
/// * `src`: Source slice with RGBA data
/// * `dst`: Destination slice for RGBA data
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `threading_policy`: Threads usage policy
///
pub fn dilate_rgba_f32(
    src: &[f32],
    dst: &mut [f32],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology_rgba::<f32, { MorphOp::Dilate as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            threading_policy,
        )
    }
}

/// Erode Gray image with alpha
///
/// # Arguments
///
/// * `src`: Source slice with Gray with alpha data
/// * `dst`: Destination slice for Gray with alpha data
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `threading_policy`: Threads usage policy
///
pub fn erode_gray_alpha_f32(
    src: &[f32],
    dst: &mut [f32],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology_gray_alpha::<f32, { MorphOp::Erode as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            threading_policy,
        )
    }
}

/// Dilate an Gray image with alpha
///
/// # Arguments
///
/// * `src`: Source slice with Gray with alpha data
/// * `dst`: Destination slice for Gray with alpha data
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `threading_policy`: Threads usage policy
///
pub fn dilate_gray_alpha_f32(
    src: &[f32],
    dst: &mut [f32],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology_gray_alpha::<f32, { MorphOp::Dilate as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            threading_policy,
        )
    }
}
