/*
 * Copyright (c) Radzivon Bartoshyk 3/2025. All rights reserved.
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
use crate::ops::{ScanMax, ScanMin};
use crate::se_scan::ScanPoint;
use std::arch::aarch64::*;

pub(crate) fn morph_1d_neon_f32<const N: usize, const DILATE: bool>(
    src: &[f32],
    dst: &mut [f32],
    pad: usize,
    points: &[ScanPoint],
) {
    let mut offsets = vec![0usize; points.len()];
    for (offset, point) in offsets.iter_mut().zip(points.iter()) {
        *offset = ((point.x as isize + pad as isize) as usize) * N;
    }
    let mut cx = 0usize;

    unsafe {
        let v_base = if DILATE {
            vdupq_n_f32(f32::MIN)
        } else {
            vdupq_n_f32(f32::MAX)
        };

        for chunk in dst.chunks_exact_mut(4) {
            let mut v_val = v_base;
            for &offset in offsets.iter() {
                let shifted = vld1q_f32(src.get_unchecked(cx + offset));
                if DILATE {
                    v_val = vmaxq_f32(v_val, shifted);
                } else {
                    v_val = vminq_f32(v_val, shifted);
                }
            }
            vst1q_f32(chunk.as_mut_ptr(), v_val);
            cx += 4;
        }

        if DILATE {
            for (index, dst) in dst.iter_mut().enumerate().skip(cx) {
                let mut v_max = f32::MIN;
                for &offset in offsets.iter() {
                    let shifted = src[index + offset];
                    v_max = v_max.s_max(shifted);
                }
                *dst = v_max;
            }
        } else {
            for (index, dst) in dst.iter_mut().enumerate().skip(cx) {
                let mut v_min = f32::MAX;
                for &offset in offsets.iter() {
                    let shifted = src[index + offset];
                    v_min = v_min.s_min(shifted);
                }
                *dst = v_min;
            }
        }
    }
}
