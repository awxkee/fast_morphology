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
use crate::filter_op_declare::{Arena, MorthOpFilterFlat2DRow};
use crate::flat_se::AnalyzedSe;
use crate::op_type::MorphOp;
use crate::ops::sse::hminmax::{_mm_hmax_epu8, _mm_hmin_epu8};
use crate::ops::sse::op::make_morph_op_1d_sse;
use crate::unsafe_slice::UnsafeSlice;
use crate::ImageSize;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[derive(Clone)]
pub struct MorphOpFilterSse2DRow<const OP_TYPE: u8> {}

impl<const OP_TYPE: u8> Default for MorphOpFilterSse2DRow<OP_TYPE> {
    fn default() -> Self {
        MorphOpFilterSse2DRow {}
    }
}

impl<const OP_TYPE: u8> MorthOpFilterFlat2DRow for MorphOpFilterSse2DRow<OP_TYPE> {
    #[target_feature(enable = "sse4.1")]
    unsafe fn dispatch_row(
        &self,
        src: &[u8],
        dst: &UnsafeSlice<u8>,
        image_size: ImageSize,
        analyzed_se: AnalyzedSe,
        y: usize,
        arena: &Option<Arena>,
    ) {
        let width = image_size.width;
        let height = image_size.height;

        let op_type: MorphOp = OP_TYPE.into();
        let stride = width;

        let decision = match op_type {
            MorphOp::Dilate => _mm_max_epu8,
            MorphOp::Erode => _mm_min_epu8,
        };
        let across_vec_decision = match op_type {
            MorphOp::Dilate => _mm_hmax_epu8,
            MorphOp::Erode => _mm_hmin_epu8,
        };

        let base_val = match op_type {
            MorphOp::Dilate => u8::MIN,
            MorphOp::Erode => u8::MAX,
        };

        let upper_fix = _mm_set_epi8(
            base_val as i8,
            base_val as i8,
            base_val as i8,
            base_val as i8,
            base_val as i8,
            base_val as i8,
            base_val as i8,
            base_val as i8,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        );

        if let Some(arena) = arena {
            let src = &arena.arena;

            let dx = arena.pad_w as i32;
            let dy = arena.pad_h as i32;

            let fast_morph_op = make_morph_op_1d_sse::<OP_TYPE>();

            let mut items0 = vec![base_val; analyzed_se.left_front.element_offsets.len()];

            for x in 0..width {
                let filter_bounds = &analyzed_se.left_front.filter_bounds;

                let mut index_iter = 0usize;

                for &filter in filter_bounds.iter() {
                    let filter_start_x = (filter.x + x as i32 + dx) as usize;
                    let filter_start_y = (filter.y + y as i32 + dy) as usize;

                    let filter_size = filter.size as usize;

                    let py0 = filter_start_y * arena.width;

                    let mut current_x = 0usize;

                    while current_x + 16 < filter_size {
                        let px = filter_start_x + current_x;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = _mm_loadu_si128(current0.as_ptr() as *const __m128i);

                        *items0.get_unchecked_mut(index_iter) = across_vec_decision(new_value0);
                        index_iter += 1;

                        current_x += 16;
                    }

                    while current_x + 8 < filter_size {
                        let px = filter_start_x + current_x;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = _mm_or_si128(_mm_loadu_si64(current0.as_ptr()), upper_fix);

                        *items0.get_unchecked_mut(index_iter) = across_vec_decision(new_value0);
                        index_iter += 1;

                        current_x += 8;
                    }

                    while current_x + 4 < filter_size {
                        let px = filter_start_x + current_x;
                        let base_offset_0 = py0 + px;

                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current0.as_ptr() as *const i32).read_unaligned(),
                        );

                        *items0.get_unchecked_mut(index_iter) = across_vec_decision(new_value0);
                        index_iter += 1;

                        current_x += 4;
                    }

                    while current_x < filter_size {
                        let px = filter_start_x + current_x;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);

                        *items0.get_unchecked_mut(index_iter) = *current0.get_unchecked(0);
                        index_iter += 1;

                        current_x += 1;
                    }
                }

                let ptr0 = (dst.slice.as_ptr() as *mut u8).add(y * stride + x);
                ptr0.write_unaligned(fast_morph_op(items0.get_unchecked(..index_iter)));
            }
        } else {
            let max_width = width as i32 - 1;
            let max_height = height as i32 - 1;

            for x in 0..width {
                let filter_bounds = &analyzed_se.left_front.filter_bounds;

                let mut value0 = base_val;

                for &filter in filter_bounds.iter() {
                    let filter_start_x = filter.x + x as i32;
                    let filter_start_y = filter.y + y as i32;
                    if (filter_start_y + 1) < 0 {
                        continue;
                    }
                    if (filter_start_x + filter.size as i32) < 0 {
                        continue;
                    }
                    let mut filter_size = filter.size as usize;
                    if filter_size as i32 + filter_start_x >= width as i32 {
                        filter_size = (width as i32 - filter_start_x) as usize;
                    }

                    let py = filter_start_y;
                    let py0 = py.min(max_height).max(0) as usize * stride;

                    let mut current_x = 0usize;

                    let mut values_v = _mm_set1_epi8(value0 as i8);

                    while current_x + 16 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = _mm_loadu_si128(current0.as_ptr() as *const __m128i);

                        values_v = decision(new_value0, values_v);

                        current_x += 16;
                    }

                    while current_x + 8 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = _mm_or_si128(_mm_loadu_si64(current0.as_ptr()), upper_fix);

                        values_v = decision(new_value0, values_v);

                        current_x += 8;
                    }

                    while current_x + 4 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize;
                        let base_offset_0 = py0 + px;

                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current0.as_ptr() as *const i32).read_unaligned(),
                        );

                        values_v = decision(new_value0, values_v);

                        current_x += 4;
                    }

                    value0 = across_vec_decision(values_v);

                    while current_x < filter_size {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize;
                        let base_offset_0 = py0 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        value0 = match op_type {
                            MorphOp::Dilate => value0.max(*current0.get_unchecked(0)),
                            MorphOp::Erode => value0.min(*current0.get_unchecked(0)),
                        };

                        current_x += 1;
                    }
                }

                let ptr0 = (dst.slice.as_ptr() as *mut u8).add(y * stride + x);
                ptr0.write_unaligned(value0);
            }
        }
    }
}
