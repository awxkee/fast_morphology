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
use crate::filter::Row2DFilter;
use crate::morph_base::MorphNativeOp;
use crate::op_impl::make_morphology;
use crate::packing::{RgbaPackable, UnpackedRgbaImage};
use crate::{BorderMode, ImageSize, KernelShape, MorphologyThreadingPolicy};
use num_traits::AsPrimitive;

pub(crate) unsafe fn make_morphology_rgba<T, const OP_TYPE: u8>(
    src: &[T],
    dst: &mut [T],
    image_size: ImageSize,
    structuring_element: &[u8],
    structuring_element_size: KernelShape,
    border_mode: BorderMode,
    border_constant: MorphScalar,
    threading_policy: MorphologyThreadingPolicy,
) -> Result<(), String>
where
    T: RgbaPackable<T>
        + Default
        + Copy
        + Clone
        + Send
        + Sync
        + 'static
        + MorphNativeOp<T>
        + Row2DFilter<T>,
    f64: AsPrimitive<T>,
{
    if src.len() != dst.len() || dst.len() != image_size.width * image_size.height * 4 {
        return Err(format!(
            "Source and Destination image slice expected to be {} but it was src {}, dst {}",
            image_size.width * image_size.height * 2,
            src.len(),
            dst.len()
        ));
    }
    let unpacked = T::unpack(src, image_size);
    let mut dst_unpacked = UnpackedRgbaImage::alloc(image_size);
    make_morphology::<T, OP_TYPE>(
        &unpacked.r_channel,
        &mut dst_unpacked.r_channel,
        image_size,
        structuring_element,
        structuring_element_size,
        border_mode,
        MorphScalar::dup(border_constant[0]),
        threading_policy,
    )?;
    make_morphology::<T, OP_TYPE>(
        &unpacked.g_channel,
        &mut dst_unpacked.g_channel,
        image_size,
        structuring_element,
        structuring_element_size,
        border_mode,
        MorphScalar::dup(border_constant[1]),
        threading_policy,
    )?;
    make_morphology::<T, OP_TYPE>(
        &unpacked.b_channel,
        &mut dst_unpacked.b_channel,
        image_size,
        structuring_element,
        structuring_element_size,
        border_mode,
        MorphScalar::dup(border_constant[2]),
        threading_policy,
    )?;
    make_morphology::<T, OP_TYPE>(
        &unpacked.a_channel,
        &mut dst_unpacked.a_channel,
        image_size,
        structuring_element,
        structuring_element_size,
        border_mode,
        MorphScalar::dup(border_constant[3]),
        threading_policy,
    )?;

    T::pack(&dst_unpacked, dst, image_size);
    Ok(())
}
