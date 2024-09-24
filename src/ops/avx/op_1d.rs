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
use crate::ops::sse::{_mm_hmax_epu8, _mm_hmin_epu8};
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline]
pub fn fast_morph_op_1d_avx<const OP_TYPE: u8>(slice: &[u8]) -> u8 {
    unsafe { fast_morph_op_1d_avx_del::<OP_TYPE>(slice) }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fast_morph_op_1d_avx_del<const OP_TYPE: u8>(slice: &[u8]) -> u8 {
    fast_morph_op_1d_avx2_impl::<OP_TYPE>(slice)
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fast_morph_op_1d_avx2_impl<const OP_TYPE: u8>(slice: &[u8]) -> u8 {
    unsafe {
        let op_type: MorphOp = OP_TYPE.into();
        let mut current = 0usize;

        let o_val = match op_type {
            MorphOp::Dilate => u8::MIN as i8,
            MorphOp::Erode => u8::MAX as i8,
        };
        let mut best_value_32 = _mm256_set1_epi8(o_val);

        let decision = match op_type {
            MorphOp::Dilate => _mm_max_epu8,
            MorphOp::Erode => _mm_min_epu8,
        };

        let decision_avx = match op_type {
            MorphOp::Dilate => _mm256_max_epu8,
            MorphOp::Erode => _mm256_min_epu8,
        };

        let decision_horizontal = match op_type {
            MorphOp::Dilate => _mm_hmax_epu8,
            MorphOp::Erode => _mm_hmin_epu8,
        };

        let upper_fix = _mm_set_epi8(
            o_val, o_val, o_val, o_val, o_val, o_val, o_val, o_val, 0, 0, 0, 0, 0, 0, 0, 0,
        );

        while current + 32 < slice.len() {
            let values = _mm256_loadu_si256(slice.as_ptr().add(current) as *const __m256i);
            best_value_32 = decision_avx(best_value_32, values);
            current += 32;
        }

        let mut best_value_16 = decision(
            _mm256_castsi256_si128(best_value_32),
            _mm256_extracti128_si256::<1>(best_value_32),
        );

        while current + 16 < slice.len() {
            let values = _mm_loadu_si128(slice.as_ptr().add(current) as *const __m128i);
            best_value_16 = decision(best_value_16, values);
            current += 16;
        }

        while current + 8 < slice.len() {
            let values = _mm_or_si128(_mm_loadu_si64(slice.as_ptr().add(current)), upper_fix);
            best_value_16 = decision(best_value_16, values);
            current += 8;
        }

        let mut best_value = decision_horizontal(best_value_16);

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
