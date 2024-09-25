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
use crate::packing::neon::deinterleave_rgb_neon;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::packing::sse::deinterleave_rgb_sse;
use crate::packing::UnpackedRgbImage;
use crate::ImageSize;

pub fn deinterleave_rgb_naive<T>(
    rgb_image: &[T],
    width: usize,
    height: usize,
) -> UnpackedRgbImage<T>
where
    T: Copy + Default,
{
    if rgb_image.len() != width * height * 3 {
        panic!(
            "Image bounds in deinterleave_rgba_neon is mismatched! Expected {} but got {}",
            width * height * 3,
            rgb_image.len()
        );
    }
    let mut r_chan = vec![T::default(); width * height];
    let mut g_chan = vec![T::default(); width * height];
    let mut b_chan = vec![T::default(); width * height];

    for (((src, r), g), b) in rgb_image
        .chunks_exact(3)
        .zip(r_chan.iter_mut())
        .zip(g_chan.iter_mut())
        .zip(b_chan.iter_mut())
    {
        *r = src[0];
        *g = src[1];
        *b = src[2];
    }

    UnpackedRgbImage::new(r_chan, g_chan, b_chan)
}

pub fn unpack_rgb(rgb_image: &[u8], image_size: ImageSize) -> UnpackedRgbImage<u8> {
    let mut _dispatcher: fn(&[u8], usize, usize) -> UnpackedRgbImage<u8> = deinterleave_rgb_naive;
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    {
        _dispatcher = deinterleave_rgb_neon;
    }
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if std::arch::is_x86_feature_detected!("sse4.1") {
            _dispatcher = deinterleave_rgb_sse;
        }
    }
    _dispatcher(rgb_image, image_size.width, image_size.height)
}
