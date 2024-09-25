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
use crate::packing::sse::v_store::{_mm_store_interleaved_rgb, _mm_store_rgb_half_u8};
use crate::packing::UnpackedRgbImage;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub fn pack_rgb_sse(
    unpacked_rgb_image: &UnpackedRgbImage<u8>,
    dst_image: &mut [u8],
    width: usize,
    height: usize,
) {
    unsafe {
        pack_rgb_sse_impl(unpacked_rgb_image, dst_image, width, height);
    }
}

#[inline]
#[target_feature(enable = "sse4.1")]
unsafe fn pack_rgb_sse_impl(
    unpacked_rgb_image: &UnpackedRgbImage<u8>,
    dst_image: &mut [u8],
    width: usize,
    height: usize,
) {
    if dst_image.len() != width * height * 3 {
        panic!(
            "Image bounds in pack_rgb_neon is mismatched! Expected {} but got {}",
            width * height * 3,
            dst_image.len()
        );
    }

    let mut r_src: &[u8] = unpacked_rgb_image.r_channel.as_slice();
    let mut g_src: &[u8] = unpacked_rgb_image.g_channel.as_slice();
    let mut b_src: &[u8] = unpacked_rgb_image.b_channel.as_slice();

    let src_stride = width * 3;

    let mut dst = dst_image;
    unsafe {
        for _ in 0..height {
            let mut _cx = 0usize;

            while _cx + 16 < width {
                let px = _cx * 3;
                let set = (
                    _mm_loadu_si128(r_src.as_ptr().add(_cx) as *const __m128i),
                    _mm_loadu_si128(g_src.as_ptr().add(_cx) as *const __m128i),
                    _mm_loadu_si128(b_src.as_ptr().add(_cx) as *const __m128i),
                );
                _mm_store_interleaved_rgb(dst.as_mut_ptr().add(px), set.0, set.1, set.2);
                _cx += 16;
            }

            while _cx + 8 < width {
                let px = _cx * 3;
                let set = (
                    _mm_loadu_si64(r_src.as_ptr().add(_cx)),
                    _mm_loadu_si64(g_src.as_ptr().add(_cx)),
                    _mm_loadu_si64(b_src.as_ptr().add(_cx)),
                );
                _mm_store_rgb_half_u8(dst.as_mut_ptr().add(px), set.0, set.1, set.2);
                _cx += 8;
            }

            while _cx < width {
                let px = _cx * 3;
                let dst_align = dst.get_unchecked_mut(px..);
                *dst_align.get_unchecked_mut(0) = *r_src.get_unchecked(_cx);
                *dst_align.get_unchecked_mut(1) = *g_src.get_unchecked(_cx);
                *dst_align.get_unchecked_mut(2) = *b_src.get_unchecked(_cx);
                _cx += 1;
            }

            dst = dst.get_unchecked_mut(src_stride..);
            r_src = r_src.get_unchecked(width..);
            g_src = g_src.get_unchecked(width..);
            b_src = b_src.get_unchecked(width..);
        }
    }
}
