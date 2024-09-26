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
use std::arch::aarch64::*;

pub fn morph_gradient_neon(dilation: &[u8], erosion: &[u8], dst: &mut [u8]) {
    if dilation.len() != erosion.len() || erosion.len() != dst.len() {
        panic!(
            "All array must match in size for gradient but received v0: {}, v1: {}, v2: {}",
            dilation.len(),
            erosion.len(),
            dst.len()
        );
    }
    let length = dilation.len();
    let mut _cx = 0usize;

    unsafe {
        while _cx + 64 < length {
            let v0_set = vld1q_u8_x4(dilation.get_unchecked(_cx..).as_ptr());
            let v1_set = vld1q_u8_x4(erosion.get_unchecked(_cx..).as_ptr());
            let result_set = uint8x16x4_t(
                vqsubq_u8(v0_set.0, v1_set.0),
                vqsubq_u8(v0_set.1, v1_set.1),
                vqsubq_u8(v0_set.2, v1_set.2),
                vqsubq_u8(v0_set.3, v1_set.3),
            );
            vst1q_u8_x4(dst.get_unchecked_mut(_cx..).as_mut_ptr(), result_set);
            _cx += 64;
        }

        while _cx + 32 < length {
            let v0_set = vld1q_u8_x2(dilation.get_unchecked(_cx..).as_ptr());
            let v1_set = vld1q_u8_x2(erosion.get_unchecked(_cx..).as_ptr());
            let result_set =
                uint8x16x2_t(vqsubq_u8(v0_set.0, v1_set.0), vqsubq_u8(v0_set.1, v1_set.1));
            vst1q_u8_x2(dst.get_unchecked_mut(_cx..).as_mut_ptr(), result_set);
            _cx += 32;
        }

        while _cx + 16 < length {
            let v0_set = vld1q_u8(dilation.get_unchecked(_cx..).as_ptr());
            let v1_set = vld1q_u8(erosion.get_unchecked(_cx..).as_ptr());
            let result_set = vqsubq_u8(v0_set, v1_set);
            vst1q_u8(dst.get_unchecked_mut(_cx..).as_mut_ptr(), result_set);
            _cx += 16;
        }

        while _cx + 8 < length {
            let v0_set = vld1_u8(dilation.get_unchecked(_cx..).as_ptr());
            let v1_set = vld1_u8(erosion.get_unchecked(_cx..).as_ptr());
            let result_set = vqsub_u8(v0_set, v1_set);
            vst1_u8(dst.get_unchecked_mut(_cx..).as_mut_ptr(), result_set);
            _cx += 8;
        }

        while _cx < length {
            *dst.get_unchecked_mut(_cx) = dilation
                .get_unchecked(_cx)
                .saturating_sub(*erosion.get_unchecked(_cx));
            _cx += 1;
        }
    }
}
