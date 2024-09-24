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
use crate::ops::avx::v_load::_mm256_deinterleave_rgb;
use crate::ops::sse::{_mm_deinterleave_rgb, _mm_hmax_epu8, _mm_hmin_epu8};
use colorutils_rs::Rgb;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline]
pub fn fast_morph_op_3d_avx<const OP_TYPE: u8>(slice: &[Rgb<u8>]) -> Rgb<u8> {
    unsafe { fast_morph_op_3d_avx_del::<OP_TYPE>(slice) }
}
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fast_morph_op_3d_avx_del<const OP_TYPE: u8>(slice: &[Rgb<u8>]) -> Rgb<u8> {
    fast_morph_op_3d_avx_impl::<OP_TYPE>(slice)
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn fast_morph_op_3d_avx_impl<const OP_TYPE: u8>(slice: &[Rgb<u8>]) -> Rgb<u8> {
    unsafe {
        let op_type: MorphOp = OP_TYPE.into();
        let mut current = 0usize;

        let o_val = match op_type {
            MorphOp::Dilate => u8::MIN as i8,
            MorphOp::Erode => u8::MAX as i8,
        };

        let vo_val = _mm_set1_epi8(o_val);

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

        let mut best_value_32_rv = _mm256_set1_epi8(o_val);
        let mut best_value_32_gv = _mm256_set1_epi8(o_val);
        let mut best_value_32_bv = _mm256_set1_epi8(o_val);

        while current + 32 < slice.len() {
            let src_ptr = slice.get_unchecked(current..).as_ptr() as *const u8;
            let row0 = _mm256_loadu_si256(src_ptr as *const __m256i);
            let row1 = _mm256_loadu_si256(src_ptr.add(32) as *const __m256i);
            let row2 = _mm256_loadu_si256(src_ptr.add(64) as *const __m256i);
            let (rv, gv, bv) = _mm256_deinterleave_rgb(row0, row1, row2);
            best_value_32_rv = decision_avx(best_value_32_rv, rv);
            best_value_32_gv = decision_avx(best_value_32_gv, gv);
            best_value_32_bv = decision_avx(best_value_32_bv, bv);
            current += 16;
        }

        let mut best_value_16_rv = decision(
            _mm256_castsi256_si128(best_value_32_rv),
            _mm256_extracti128_si256::<1>(best_value_32_rv),
        );
        let mut best_value_16_gv = decision(
            _mm256_castsi256_si128(best_value_32_gv),
            _mm256_extracti128_si256::<1>(best_value_32_gv),
        );
        let mut best_value_16_bv = decision(
            _mm256_castsi256_si128(best_value_32_bv),
            _mm256_extracti128_si256::<1>(best_value_32_bv),
        );

        while current + 16 < slice.len() {
            let src_ptr = slice.get_unchecked(current..).as_ptr() as *const u8;
            let row0 = _mm_loadu_si128(src_ptr as *const __m128i);
            let row1 = _mm_loadu_si128(src_ptr.add(16) as *const __m128i);
            let row2 = _mm_loadu_si128(src_ptr.add(32) as *const __m128i);
            let (rv, gv, bv) = _mm_deinterleave_rgb(row0, row1, row2);
            best_value_16_rv = decision(best_value_16_rv, rv);
            best_value_16_gv = decision(best_value_16_gv, gv);
            best_value_16_bv = decision(best_value_16_bv, bv);
            current += 16;
        }

        while current + 8 < slice.len() {
            let src_ptr = slice.get_unchecked(current..).as_ptr() as *const u8;
            let row0 = _mm_loadu_si128(src_ptr as *const __m128i);
            let row1 = _mm_or_si128(_mm_loadu_si64(src_ptr.add(16)), upper_fix);
            let (rv, gv, bv) = _mm_deinterleave_rgb(row0, row1, vo_val);
            best_value_16_rv = decision(best_value_16_rv, rv);
            best_value_16_gv = decision(best_value_16_gv, gv);
            best_value_16_bv = decision(best_value_16_bv, bv);
            current += 8;
        }

        let mut best_value = Rgb::new(
            decision_horizontal(best_value_16_rv),
            decision_horizontal(best_value_16_gv),
            decision_horizontal(best_value_16_bv),
        );

        while current < slice.len() {
            best_value = match op_type {
                MorphOp::Dilate => best_value.max_p(*slice.get_unchecked(current)),
                MorphOp::Erode => best_value.min_p(*slice.get_unchecked(current)),
            };
            current += 1;
        }

        best_value
    }
}
