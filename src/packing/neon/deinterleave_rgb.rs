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
use crate::packing::UnpackedRgbImage;
use std::arch::aarch64::*;

pub fn deinterleave_rgb_neon(
    rgb_image: &[u8],
    width: usize,
    height: usize,
) -> UnpackedRgbImage<u8> {
    if rgb_image.len() != width * height * 3 {
        panic!(
            "Image bounds in deinterleave_rgba_neon is mismatched! Expected {} but got {}",
            width * height * 3,
            rgb_image.len()
        );
    }
    let mut r_chan = vec![0u8; width * height];
    let mut g_chan = vec![0u8; width * height];
    let mut b_chan = vec![0u8; width * height];

    let mut r_dst = r_chan.as_mut_slice();
    let mut g_dst = g_chan.as_mut_slice();
    let mut b_dst = b_chan.as_mut_slice();

    let src_stride = width * 3;

    let mut src = rgb_image;
    unsafe {
        for _ in 0..height {
            let mut _cx = 0usize;

            while _cx + 16 < width {
                let px = _cx * 3;
                let pixels = vld3q_u8(src.as_ptr().add(px));
                vst1q_u8(r_dst.as_mut_ptr().add(_cx), pixels.0);
                vst1q_u8(g_dst.as_mut_ptr().add(_cx), pixels.1);
                vst1q_u8(b_dst.as_mut_ptr().add(_cx), pixels.2);
                _cx += 16;
            }

            while _cx + 8 < width {
                let px = _cx * 3;
                let pixels = vld3_u8(src.as_ptr().add(px));
                vst1_u8(r_dst.as_mut_ptr().add(_cx), pixels.0);
                vst1_u8(g_dst.as_mut_ptr().add(_cx), pixels.1);
                vst1_u8(b_dst.as_mut_ptr().add(_cx), pixels.2);
                _cx += 8;
            }

            while _cx < width {
                let px = _cx * 3;
                let src_align = src.get_unchecked(px..);
                *r_dst.get_unchecked_mut(_cx) = *src_align.get_unchecked(0);
                *g_dst.get_unchecked_mut(_cx) = *src_align.get_unchecked(1);
                *b_dst.get_unchecked_mut(_cx) = *src_align.get_unchecked(2);
                _cx += 1;
            }

            src = src.get_unchecked(src_stride..);
            r_dst = r_dst.get_unchecked_mut(width..);
            g_dst = g_dst.get_unchecked_mut(width..);
            b_dst = b_dst.get_unchecked_mut(width..);
        }
    }
    UnpackedRgbImage::new(r_chan, g_chan, b_chan)
}
