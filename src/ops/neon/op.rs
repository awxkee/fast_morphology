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
use crate::op_type::MorphOp;
use colorutils_rs::{Rgb, Rgba};
use std::arch::aarch64::*;

#[inline(always)]
pub fn fast_morph_op_1d_neon<const OP_TYPE: u8>(slice: &[u8]) -> u8 {
    unsafe {
        let op_type: MorphOp = OP_TYPE.into();
        let mut current = 0usize;

        let mut best_value_16 = vdupq_n_u8(match op_type {
            MorphOp::Dilate => u8::MIN,
            MorphOp::Erode => u8::MAX,
        });

        let decision_16 = match op_type {
            MorphOp::Dilate => vmaxq_u8,
            MorphOp::Erode => vminq_u8,
        };

        let decision_8 = match op_type {
            MorphOp::Dilate => vmax_u8,
            MorphOp::Erode => vmin_u8,
        };

        let decision_horizontal_8 = match op_type {
            MorphOp::Dilate => vmaxv_u8,
            MorphOp::Erode => vminv_u8,
        };

        while current + 16 < slice.len() {
            let values = vld1q_u8(slice.as_ptr().add(current));
            best_value_16 = decision_16(best_value_16, values);
            current += 16;
        }

        let mut best_value_8 = decision_8(vget_low_u8(best_value_16), vget_high_u8(best_value_16));

        while current + 8 < slice.len() {
            let values = vld1_u8(slice.as_ptr().add(current));
            best_value_8 = decision_8(best_value_8, values);
            current += 8;
        }

        let mut best_value = decision_horizontal_8(best_value_8);

        while current < slice.len() {
            best_value = match op_type {
                MorphOp::Dilate => best_value.max(*slice.get_unchecked(current)),
                MorphOp::Erode => best_value.min(*slice.get_unchecked(current)),
            };
            current += 1;
        }

        best_value
    }
}

#[inline(always)]
pub fn fast_morph_op_4d_neon<const OP_TYPE: u8>(slice: &[Rgba<u8>]) -> Rgba<u8> {
    unsafe {
        let op_type: MorphOp = OP_TYPE.into();
        let mut current = 0usize;

        let b_val = match op_type {
            MorphOp::Dilate => u8::MIN,
            MorphOp::Erode => u8::MAX,
        };

        let mut best_value_16_r = vdupq_n_u8(b_val);
        let mut best_value_16_g = vdupq_n_u8(b_val);
        let mut best_value_16_b = vdupq_n_u8(b_val);
        let mut best_value_16_a = vdupq_n_u8(b_val);

        let decision_16 = match op_type {
            MorphOp::Dilate => vmaxq_u8,
            MorphOp::Erode => vminq_u8,
        };

        let decision_8 = match op_type {
            MorphOp::Dilate => vmax_u8,
            MorphOp::Erode => vmin_u8,
        };

        let decision_horizontal_8 = match op_type {
            MorphOp::Dilate => vmaxv_u8,
            MorphOp::Erode => vminv_u8,
        };

        while current + 16 < slice.len() {
            let values = vld4q_u8((slice.as_ptr() as *const u8).add(current * 4));
            best_value_16_r = decision_16(best_value_16_r, values.0);
            best_value_16_g = decision_16(best_value_16_g, values.1);
            best_value_16_b = decision_16(best_value_16_b, values.2);
            best_value_16_a = decision_16(best_value_16_a, values.3);
            current += 16;
        }

        let mut best_value_8_r =
            decision_8(vget_low_u8(best_value_16_r), vget_high_u8(best_value_16_r));
        let mut best_value_8_g =
            decision_8(vget_low_u8(best_value_16_g), vget_high_u8(best_value_16_g));
        let mut best_value_8_b =
            decision_8(vget_low_u8(best_value_16_b), vget_high_u8(best_value_16_b));
        let mut best_value_8_a =
            decision_8(vget_low_u8(best_value_16_a), vget_high_u8(best_value_16_a));

        while current + 8 < slice.len() {
            let values = vld4_u8((slice.as_ptr() as *const u8).add(current * 4));
            best_value_8_r = decision_8(best_value_8_r, values.0);
            best_value_8_g = decision_8(best_value_8_g, values.1);
            best_value_8_b = decision_8(best_value_8_b, values.2);
            best_value_8_a = decision_8(best_value_8_a, values.3);
            current += 8;
        }

        let mut best_value_r = decision_horizontal_8(best_value_8_r);
        let mut best_value_g = decision_horizontal_8(best_value_8_g);
        let mut best_value_b = decision_horizontal_8(best_value_8_b);
        let mut best_value_a = decision_horizontal_8(best_value_8_a);

        while current < slice.len() {
            let ptr = (slice.as_ptr() as *const u8).add(current * 4);
            best_value_r = match op_type {
                MorphOp::Dilate => best_value_r.max(ptr.read_unaligned()),
                MorphOp::Erode => best_value_r.min(ptr.read_unaligned()),
            };
            best_value_g = match op_type {
                MorphOp::Dilate => best_value_g.max(ptr.add(1).read_unaligned()),
                MorphOp::Erode => best_value_g.min(ptr.add(1).read_unaligned()),
            };
            best_value_b = match op_type {
                MorphOp::Dilate => best_value_b.max(ptr.add(2).read_unaligned()),
                MorphOp::Erode => best_value_b.min(ptr.add(2).read_unaligned()),
            };
            best_value_a = match op_type {
                MorphOp::Dilate => best_value_a.max(ptr.add(3).read_unaligned()),
                MorphOp::Erode => best_value_a.min(ptr.add(3).read_unaligned()),
            };
            current += 1;
        }

        Rgba::new(best_value_r, best_value_g, best_value_b, best_value_a)
    }
}

#[inline(always)]
pub fn fast_morph_op_3d_neon<const OP_TYPE: u8>(slice: &[Rgb<u8>]) -> Rgb<u8> {
    unsafe {
        let op_type: MorphOp = OP_TYPE.into();
        let mut current = 0usize;

        let b_val = match op_type {
            MorphOp::Dilate => u8::MIN,
            MorphOp::Erode => u8::MAX,
        };

        let mut best_value_16_r = vdupq_n_u8(b_val);
        let mut best_value_16_g = vdupq_n_u8(b_val);
        let mut best_value_16_b = vdupq_n_u8(b_val);

        let decision_16 = match op_type {
            MorphOp::Dilate => vmaxq_u8,
            MorphOp::Erode => vminq_u8,
        };

        let decision_8 = match op_type {
            MorphOp::Dilate => vmax_u8,
            MorphOp::Erode => vmin_u8,
        };

        let decision_horizontal_8 = match op_type {
            MorphOp::Dilate => vmaxv_u8,
            MorphOp::Erode => vminv_u8,
        };

        while current + 16 < slice.len() {
            let values = vld3q_u8((slice.as_ptr() as *const u8).add(current * 3));
            best_value_16_r = decision_16(best_value_16_r, values.0);
            best_value_16_g = decision_16(best_value_16_g, values.1);
            best_value_16_b = decision_16(best_value_16_b, values.2);
            current += 16;
        }

        let mut best_value_8_r =
            decision_8(vget_low_u8(best_value_16_r), vget_high_u8(best_value_16_r));
        let mut best_value_8_g =
            decision_8(vget_low_u8(best_value_16_g), vget_high_u8(best_value_16_g));
        let mut best_value_8_b =
            decision_8(vget_low_u8(best_value_16_b), vget_high_u8(best_value_16_b));

        while current + 8 < slice.len() {
            let values = vld3_u8((slice.as_ptr() as *const u8).add(current * 3));
            best_value_8_r = decision_8(best_value_8_r, values.0);
            best_value_8_g = decision_8(best_value_8_g, values.1);
            best_value_8_b = decision_8(best_value_8_b, values.2);
            current += 8;
        }

        let mut best_value = Rgb::new(
            decision_horizontal_8(best_value_8_r),
            decision_horizontal_8(best_value_8_g),
            decision_horizontal_8(best_value_8_b),
        );

        while current < slice.len() {
            let item = *slice.get_unchecked(current);
            best_value = match op_type {
                MorphOp::Dilate => best_value.max_p(item),
                MorphOp::Erode => best_value.min_p(item),
            };
            current += 1;
        }

        best_value
    }
}
