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
use crate::packing::avx::v_load::_mm256_load_deinterleave_rgba;
use crate::packing::sse::{_mm_load_deinterleave_half_rgba, _mm_load_deinterleave_rgba};
use crate::packing::UnpackedRgbaImage;
use crate::ImageSize;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub fn deinterleave_rgba_avx(
    rgb_image: &[u8],
    width: usize,
    height: usize,
) -> UnpackedRgbaImage<u8> {
    unsafe { deinterleave_rgba_avx_impl(rgb_image, width, height) }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn deinterleave_rgba_avx_impl(
    rgb_image: &[u8],
    width: usize,
    height: usize,
) -> UnpackedRgbaImage<u8> {
    if rgb_image.len() != width * height * 4 {
        panic!(
            "Image bounds in deinterleave_rgba_sse is mismatched! Expected {} but got {}",
            width * height * 4,
            rgb_image.len()
        );
    }
    let mut unpacked_image = UnpackedRgbaImage::alloc(ImageSize::new(width, height));

    let mut r_dst: &mut [u8] = unpacked_image.r_channel.as_mut_slice();
    let mut g_dst: &mut [u8] = unpacked_image.g_channel.as_mut_slice();
    let mut b_dst: &mut [u8] = unpacked_image.b_channel.as_mut_slice();
    let mut a_dst: &mut [u8] = unpacked_image.a_channel.as_mut_slice();

    let src_stride = width * 4;

    let mut src = rgb_image;
    unsafe {
        for _ in 0..height {
            let mut _cx = 0usize;

            while _cx + 32 < width {
                let px = _cx * 4;
                let pixels = _mm256_load_deinterleave_rgba(src.as_ptr().add(px));
                _mm256_storeu_si256(r_dst.as_mut_ptr().add(_cx) as *mut __m256i, pixels.0);
                _mm256_storeu_si256(g_dst.as_mut_ptr().add(_cx) as *mut __m256i, pixels.1);
                _mm256_storeu_si256(b_dst.as_mut_ptr().add(_cx) as *mut __m256i, pixels.2);
                _mm256_storeu_si256(a_dst.as_mut_ptr().add(_cx) as *mut __m256i, pixels.3);
                _cx += 32;
            }

            while _cx + 16 < width {
                let px = _cx * 4;
                let pixels = _mm_load_deinterleave_rgba(src.as_ptr().add(px));
                _mm_storeu_si128(r_dst.as_mut_ptr().add(_cx) as *mut __m128i, pixels.0);
                _mm_storeu_si128(g_dst.as_mut_ptr().add(_cx) as *mut __m128i, pixels.1);
                _mm_storeu_si128(b_dst.as_mut_ptr().add(_cx) as *mut __m128i, pixels.2);
                _mm_storeu_si128(a_dst.as_mut_ptr().add(_cx) as *mut __m128i, pixels.3);
                _cx += 16;
            }

            while _cx + 8 < width {
                let px = _cx * 4;
                let pixels = _mm_load_deinterleave_half_rgba(src.as_ptr().add(px), 0);
                let v0 = pixels.0;
                std::ptr::copy_nonoverlapping(
                    &v0 as *const _ as *const u8,
                    r_dst.as_mut_ptr().add(_cx),
                    8,
                );
                let v1 = pixels.1;
                std::ptr::copy_nonoverlapping(
                    &v1 as *const _ as *const u8,
                    g_dst.as_mut_ptr().add(_cx),
                    8,
                );
                let v2 = pixels.2;
                std::ptr::copy_nonoverlapping(
                    &v2 as *const _ as *const u8,
                    b_dst.as_mut_ptr().add(_cx),
                    8,
                );
                let v3 = pixels.3;
                std::ptr::copy_nonoverlapping(
                    &v3 as *const _ as *const u8,
                    a_dst.as_mut_ptr().add(_cx),
                    8,
                );
                _cx += 8;
            }

            while _cx < width {
                let px = _cx * 4;
                let src_align = src.get_unchecked(px..);
                *r_dst.get_unchecked_mut(_cx) = *src_align.get_unchecked(0);
                *g_dst.get_unchecked_mut(_cx) = *src_align.get_unchecked(1);
                *b_dst.get_unchecked_mut(_cx) = *src_align.get_unchecked(2);
                *a_dst.get_unchecked_mut(_cx) = *src_align.get_unchecked(3);
                _cx += 1;
            }

            src = src.get_unchecked(src_stride..);
            r_dst = r_dst.get_unchecked_mut(width..);
            g_dst = g_dst.get_unchecked_mut(width..);
            b_dst = b_dst.get_unchecked_mut(width..);
            a_dst = a_dst.get_unchecked_mut(width..);
        }
    }
    unpacked_image
}
