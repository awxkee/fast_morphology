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
use crate::filter::Row2DFilter;
use crate::morph_base::MorphNativeOp;
use crate::op_impl::make_morphology;
use crate::op_type::{MorphExOp, MorphOp};
use crate::structuring_element::KernelShape;
use crate::{ImageSize, MorphologyThreadingPolicy};
use num_traits::AsPrimitive;

/// Dilate a gray (planar) image
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
pub fn dilate(
    src: &[u8],
    dst: &mut [u8],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    make_morphology::<u8, { MorphOp::Dilate as u8 }, 1>(
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

/// Dilate an RGB image
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
pub fn dilate_rgb(
    src: &[u8],
    dst: &mut [u8],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    make_morphology::<u8, { MorphOp::Dilate as u8 }, 3>(
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

/// Erode a gray (planar) image
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
pub fn erode(
    src: &[u8],
    dst: &mut [u8],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    make_morphology::<u8, { MorphOp::Erode as u8 }, 1>(
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

/// Erode an RGB image
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
pub fn erode_rgb(
    src: &[u8],
    dst: &mut [u8],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    make_morphology::<u8, { MorphOp::Erode as u8 }, 3>(
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

/// Erode an RGBA image
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
pub fn erode_rgba(
    src: &[u8],
    dst: &mut [u8],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    make_morphology::<u8, { MorphOp::Erode as u8 }, 4>(
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

/// Dilate an RGBA image
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
pub fn dilate_rgba(
    src: &[u8],
    dst: &mut [u8],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    make_morphology::<u8, { MorphOp::Dilate as u8 }, 4>(
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
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn erode_gray_alpha(
    src: &[u8],
    dst: &mut [u8],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    make_morphology::<u8, { MorphOp::Erode as u8 }, 2>(
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
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn dilate_gray_alpha(
    src: &[u8],
    dst: &mut [u8],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    make_morphology::<u8, { MorphOp::Dilate as u8 }, 2>(
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

/// Morphology a gray (planar) image
///
/// # Arguments
///
/// * `src`: Source image slice
/// * `dst`: Destination image slice
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morphology(
    src: &[u8],
    dst: &mut [u8],
    morph_op: MorphExOp,
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    morph_impl::<u8, 1>(
        src,
        dst,
        morph_op,
        image_size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )
}

/// Morphology a RGB 8-bit image
///
/// # Arguments
///
/// * `src`: Source RGB image slice
/// * `dst`: Destination RGB image slice
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morphology_rgb(
    src: &[u8],
    dst: &mut [u8],
    morph_op: MorphExOp,
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    morph_impl::<u8, 3>(
        src,
        dst,
        morph_op,
        image_size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )
}

/// Morphology a Planar image with alpha 8-bit image
///
/// # Arguments
///
/// * `src`: Source image slice
/// * `dst`: Destination image slice
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morphology_gray_alpha(
    src: &[u8],
    dst: &mut [u8],
    morph_op: MorphExOp,
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    morph_impl::<u8, 2>(
        src,
        dst,
        morph_op,
        image_size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )
}

pub(crate) fn morph_impl<
    T: Copy + MorphGradient<T> + Default + 'static + Send + Sync + MorphNativeOp<T> + Row2DFilter<T>,
    const CN: usize,
>(
    src: &[T],
    dst: &mut [T],
    morph_op: MorphExOp,
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String>
where
    f64: AsPrimitive<T>,
{
    match morph_op {
        MorphExOp::Dilate => make_morphology::<T, { MorphOp::Dilate as u8 }, CN>(
            src,
            dst,
            image_size,
            structuring_element,
            structuring_element_size,
            border_mode,
            border_scalar,
            threading_policy,
        ),
        MorphExOp::Erode => make_morphology::<T, { MorphOp::Erode as u8 }, CN>(
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
            let mut transient = vec![T::default(); dst.len()];
            make_morphology::<T, { MorphOp::Erode as u8 }, CN>(
                src,
                &mut transient,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            make_morphology::<T, { MorphOp::Dilate as u8 }, CN>(
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
            let mut transient = vec![T::default(); dst.len()];
            make_morphology::<T, { MorphOp::Dilate as u8 }, CN>(
                src,
                &mut transient,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            make_morphology::<T, { MorphOp::Erode as u8 }, CN>(
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
            let mut dilation = vec![T::default(); dst.len()];
            make_morphology::<T, { MorphOp::Dilate as u8 }, CN>(
                src,
                &mut dilation,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            let mut erosion = vec![T::default(); dst.len()];
            make_morphology::<T, { MorphOp::Erode as u8 }, CN>(
                src,
                &mut erosion,
                image_size,
                structuring_element,
                structuring_element_size,
                border_mode,
                border_scalar,
                threading_policy,
            )?;
            T::morph_gradient(&dilation, &erosion, dst);
            Ok(())
        }
        MorphExOp::TopHat => {
            let mut opened = vec![T::default(); dst.len()];
            morph_impl::<T, CN>(
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
            T::morph_gradient(src, &opened, dst);
            Ok(())
        }
        MorphExOp::BlackHat => {
            let mut closed = vec![T::default(); dst.len()];
            morph_impl::<T, CN>(
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
            T::morph_gradient(&closed, src, dst);
            Ok(())
        }
    }
}

/// Morphology a RGBA 8-bit image
///
/// # Arguments
///
/// * `src`: Source RGBA image slice
/// * `dst`: Destination RGBA image slice
/// * `morph_op`: Requested [MorphExOp]
/// * `image_size`: Image size declared by [ImageSize]
/// * `structuring_element`: 2D structuring element
/// * `structuring_element_size`: (W,H) structuring element size
/// * `border_mode`: Border handling mode, for reference see [BorderMode]
/// * `border_scalar`: [MorphScalar] scalar value that will be used to fill border in [BorderMode::Constant]
/// * `threading_policy`: Threads usage policy
///
pub fn morphology_rgba(
    src: &[u8],
    dst: &mut [u8],
    morph_op: MorphExOp,
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_scalar: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String> {
    morph_impl::<u8, 4>(
        src,
        dst,
        morph_op,
        image_size,
        structuring_element,
        structuring_element_size,
        border_mode,
        border_scalar,
        threading_policy,
    )
}
