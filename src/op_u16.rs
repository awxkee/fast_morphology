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
use crate::border_mode::{BorderMode, MorphScalar};
use crate::difference::MorphGradient;
use crate::morph_gray_alpha::make_morphology_gray_alpha;
use crate::morph_rgb::make_morphology_rgb;
use crate::morph_rgba::make_morphology_rgba;
use crate::op_impl::make_morphology;
use crate::op_type::MorphOp;
use crate::structuring_element::KernelShape;
use crate::{ImageSize, MorphExOp, MorphologyThreadingPolicy};

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
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn dilate_u16(
    src: &[u16],
    dst: &mut [u16],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology::<u16, { MorphOp::Dilate as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
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
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn dilate_rgb_u16(
    src: &[u16],
    dst: &mut [u16],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology_rgb::<u16, { MorphOp::Dilate as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
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
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn erode_u16(
    src: &[u16],
    dst: &mut [u16],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology::<u16, { MorphOp::Erode as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
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
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn erode_rgb_u16(
    src: &[u16],
    dst: &mut [u16],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology_rgb::<u16, { MorphOp::Erode as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
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
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn erode_rgba_u16(
    src: &[u16],
    dst: &mut [u16],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology_rgba::<u16, { MorphOp::Erode as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
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
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn dilate_rgba_u16(
    src: &[u16],
    dst: &mut [u16],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology_rgba::<u16, { MorphOp::Dilate as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
            threading_policy,
        )
    }
}

/// Erode Gray image with alpha stored in u16
///
/// # Arguments
///
/// * `src`: Source slice with Gray with alpha data
/// * `dst`: Destination slice for Gray with alpha data
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn erode_gray_alpha_u16(
    src: &[u16],
    dst: &mut [u16],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology_gray_alpha::<u16, { MorphOp::Erode as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
            threading_policy,
        )
    }
}

/// Dilate an Gray image with alpha stored in u16
///
/// # Arguments
///
/// * `src`: Source slice with Gray with alpha data
/// * `dst`: Destination slice for Gray with alpha data
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn dilate_gray_alpha_u16(
    src: &[u16],
    dst: &mut [u16],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    unsafe {
        make_morphology_gray_alpha::<u16, { MorphOp::Dilate as u8 }>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
            threading_policy,
        )
    }
}

/// Morphology Gray image with alpha stored in u16
///
/// # Arguments
///
/// * `src`: Source slice with Gray with alpha data
/// * `dst`: Destination slice for Gray with alpha data
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morphology_gray_alpha_u16(
    src: &[u16],
    dst: &mut [u16],
    morph_op: MorphExOp,
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    match morph_op {
        MorphExOp::Dilate => dilate_gray_alpha_u16(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
            threading_policy,
        ),
        MorphExOp::Erode => erode_gray_alpha_u16(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
            threading_policy,
        ),
        MorphExOp::Opening => {
            let mut transient = vec![0u16; dst.len()];
            erode_gray_alpha_u16(
                src,
                &mut transient,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            dilate_gray_alpha_u16(
                &transient,
                dst,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )
        }
        MorphExOp::Closing => {
            let mut transient = vec![0u16; dst.len()];
            dilate_gray_alpha_u16(
                src,
                &mut transient,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            erode_gray_alpha_u16(
                &transient,
                dst,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )
        }
        MorphExOp::Gradient => {
            let mut dilation = vec![0u16; dst.len()];
            dilate_gray_alpha_u16(
                src,
                &mut dilation,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            let mut erosion = vec![0u16; dst.len()];
            erode_gray_alpha_u16(
                &src,
                &mut erosion,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            u16::morph_gradient(&dilation, &erosion, dst);
            Ok(())
        }
        MorphExOp::TopHat => {
            let mut opened = vec![0u16; dst.len()];
            morphology_gray_alpha_u16(
                src,
                &mut opened,
                MorphExOp::Opening,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            u16::morph_gradient(&src, &opened, dst);
            Ok(())
        }
        MorphExOp::BlackHat => {
            let mut closed = vec![0u16; dst.len()];
            morphology_gray_alpha_u16(
                src,
                &mut closed,
                MorphExOp::Closing,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            u16::morph_gradient(&closed, &src, dst);
            Ok(())
        }
    }
}

/// Morphology Gray image stored in u16
///
/// # Arguments
///
/// * `src`: Source slice with Gray data
/// * `dst`: Destination slice for Gray  data
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morphology_gray_u16(
    src: &[u16],
    dst: &mut [u16],
    morph_op: MorphExOp,
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    match morph_op {
        MorphExOp::Dilate => dilate_u16(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
            threading_policy,
        ),
        MorphExOp::Erode => erode_u16(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
            threading_policy,
        ),
        MorphExOp::Opening => {
            let mut transient = vec![0u16; dst.len()];
            erode_u16(
                src,
                &mut transient,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            dilate_u16(
                &transient,
                dst,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )
        }
        MorphExOp::Closing => {
            let mut transient = vec![0u16; dst.len()];
            dilate_u16(
                src,
                &mut transient,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            erode_u16(
                &transient,
                dst,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )
        }
        MorphExOp::Gradient => {
            let mut dilation = vec![0u16; dst.len()];
            dilate_u16(
                src,
                &mut dilation,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            let mut erosion = vec![0u16; dst.len()];
            erode_u16(
                &src,
                &mut erosion,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            u16::morph_gradient(&dilation, &erosion, dst);
            Ok(())
        }
        MorphExOp::TopHat => {
            let mut opened = vec![0u16; dst.len()];
            morphology_gray_u16(
                src,
                &mut opened,
                MorphExOp::Opening,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            u16::morph_gradient(&src, &opened, dst);
            Ok(())
        }
        MorphExOp::BlackHat => {
            let mut closed = vec![0u16; dst.len()];
            morphology_gray_u16(
                src,
                &mut closed,
                MorphExOp::Closing,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            u16::morph_gradient(&closed, &src, dst);
            Ok(())
        }
    }
}

/// Morphology RGB image stored in u16
///
/// # Arguments
///
/// * `src`: Source slice with RGB data
/// * `dst`: Destination slice for RGB data
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morphology_rgb_u16(
    src: &[u16],
    dst: &mut [u16],
    morph_op: MorphExOp,
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    match morph_op {
        MorphExOp::Dilate => dilate_rgb_u16(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
            threading_policy,
        ),
        MorphExOp::Erode => erode_rgb_u16(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
            threading_policy,
        ),
        MorphExOp::Opening => {
            let mut transient = vec![0u16; dst.len()];
            erode_rgb_u16(
                src,
                &mut transient,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            dilate_rgb_u16(
                &transient,
                dst,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )
        }
        MorphExOp::Closing => {
            let mut transient = vec![0u16; dst.len()];
            dilate_rgb_u16(
                src,
                &mut transient,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            erode_rgb_u16(
                &transient,
                dst,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )
        }
        MorphExOp::Gradient => {
            let mut dilation = vec![0u16; dst.len()];
            dilate_rgb_u16(
                src,
                &mut dilation,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            let mut erosion = vec![0u16; dst.len()];
            erode_rgb_u16(
                &src,
                &mut erosion,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            u16::morph_gradient(&dilation, &erosion, dst);
            Ok(())
        }
        MorphExOp::TopHat => {
            let mut opened = vec![0u16; dst.len()];
            morphology_rgb_u16(
                src,
                &mut opened,
                MorphExOp::Opening,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            u16::morph_gradient(&src, &opened, dst);
            Ok(())
        }
        MorphExOp::BlackHat => {
            let mut closed = vec![0u16; dst.len()];
            morphology_rgb_u16(
                src,
                &mut closed,
                MorphExOp::Closing,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            u16::morph_gradient(&closed, &src, dst);
            Ok(())
        }
    }
}

/// Morphology RGBA image stored in u16
///
/// # Arguments
///
/// * `src`: Source slice with RGBA data
/// * `dst`: Destination slice for RGBA data
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morphology_rgba_u16(
    src: &[u16],
    dst: &mut [u16],
    morph_op: MorphExOp,
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    match morph_op {
        MorphExOp::Dilate => dilate_rgba_u16(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
            threading_policy,
        ),
        MorphExOp::Erode => erode_rgba_u16(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
            threading_policy,
        ),
        MorphExOp::Opening => {
            let mut transient = vec![0u16; dst.len()];
            erode_rgba_u16(
                src,
                &mut transient,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            dilate_rgba_u16(
                &transient,
                dst,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )
        }
        MorphExOp::Closing => {
            let mut transient = vec![0u16; dst.len()];
            dilate_rgba_u16(
                src,
                &mut transient,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            erode_rgba_u16(
                &transient,
                dst,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )
        }
        MorphExOp::Gradient => {
            let mut dilation = vec![0u16; dst.len()];
            dilate_rgba_u16(
                src,
                &mut dilation,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            let mut erosion = vec![0u16; dst.len()];
            erode_rgba_u16(
                &src,
                &mut erosion,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            u16::morph_gradient(&dilation, &erosion, dst);
            Ok(())
        }
        MorphExOp::TopHat => {
            let mut opened = vec![0u16; dst.len()];
            morphology_rgba_u16(
                src,
                &mut opened,
                MorphExOp::Opening,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            u16::morph_gradient(&src, &opened, dst);
            Ok(())
        }
        MorphExOp::BlackHat => {
            let mut closed = vec![0u16; dst.len()];
            morphology_rgba_u16(
                src,
                &mut closed,
                MorphExOp::Closing,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            u16::morph_gradient(&closed, &src, dst);
            Ok(())
        }
    }
}
