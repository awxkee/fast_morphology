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
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use crate::packing::neon::pack_rgba_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::packing::sse::pack_rgba_sse;
use crate::packing::UnpackedRgbaImage;
use crate::ImageSize;

pub fn interleave_rgba_naive<T>(
    unpacked_rgba_image: &UnpackedRgbaImage<T>,
    dst_image: &mut [T],
    _: usize,
    _: usize,
) where
    T: Copy,
{
    for ((((src, r), g), b), a) in dst_image
        .chunks_exact_mut(4)
        .zip(unpacked_rgba_image.r_channel.iter())
        .zip(unpacked_rgba_image.g_channel.iter())
        .zip(unpacked_rgba_image.b_channel.iter())
        .zip(unpacked_rgba_image.a_channel.iter())
    {
        src[0] = *r;
        src[1] = *g;
        src[2] = *b;
        src[3] = *a;
    }
}

pub fn pack_rgba(
    unpacked_rgb_image: &UnpackedRgbaImage<u8>,
    dst_image: &mut [u8],
    image_size: ImageSize,
) {
    let mut _dispatcher: fn(&UnpackedRgbaImage<u8>, &mut [u8], usize, usize) =
        interleave_rgba_naive;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        _dispatcher = pack_rgba_neon;
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if std::arch::is_x86_feature_detected!("sse4.1") {
            _dispatcher = pack_rgba_sse;
        }
    }
    _dispatcher(
        unpacked_rgb_image,
        dst_image,
        image_size.width,
        image_size.height,
    )
}
