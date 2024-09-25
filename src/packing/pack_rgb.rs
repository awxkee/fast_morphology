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
use crate::packing::neon::pack_rgb_neon;
use crate::packing::UnpackedRgbImage;
use crate::ImageSize;

pub fn interleave_rgb_naive<T>(
    unpacked_rgb_image: &UnpackedRgbImage<T>,
    dst_image: &mut [T],
    _: usize,
    _: usize,
) where
    T: Copy,
{
    for (((src, r), g), b) in dst_image
        .chunks_exact_mut(3)
        .zip(unpacked_rgb_image.r_channel.iter())
        .zip(unpacked_rgb_image.g_channel.iter())
        .zip(unpacked_rgb_image.b_channel.iter())
    {
        src[0] = *r;
        src[1] = *g;
        src[2] = *b;
    }
}

pub fn pack_rgb(
    unpacked_rgb_image: &UnpackedRgbImage<u8>,
    dst_image: &mut [u8],
    image_size: ImageSize,
) {
    let mut _dispatcher: fn(&UnpackedRgbImage<u8>, &mut [u8], usize, usize) = interleave_rgb_naive;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        _dispatcher = pack_rgb_neon;
    }
    _dispatcher(
        unpacked_rgb_image,
        dst_image,
        image_size.width,
        image_size.height,
    );
}
