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
pub struct MorphOpFilterSse2D4Rows<const OP_TYPE: u8> {}

impl<const OP_TYPE: u8> Default for MorphOpFilterSse2D4Rows<OP_TYPE> {
    fn default() -> Self {
        MorphOpFilterSse2D4Rows {}
    }
}

impl<const OP_TYPE: u8> MorthOpFilterFlat2DRow for MorphOpFilterSse2D4Rows<OP_TYPE> {
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

            let morph_vec_op = make_morph_op_1d_sse::<OP_TYPE>();

            let dx = arena.pad_w as i32;
            let dy = arena.pad_h as i32;

            let mut items0 = vec![base_val; analyzed_se.left_front.element_offsets.len()];
            let mut items1 = vec![base_val; analyzed_se.left_front.element_offsets.len()];
            let mut items2 = vec![base_val; analyzed_se.left_front.element_offsets.len()];
            let mut items3 = vec![base_val; analyzed_se.left_front.element_offsets.len()];
            let mut items4 = vec![base_val; analyzed_se.left_front.element_offsets.len()];
            let mut items5 = vec![base_val; analyzed_se.left_front.element_offsets.len()];

            for x in 0..width {
                let filter_bounds = &analyzed_se.left_front.filter_bounds;

                let mut index_iter = 0usize;

                for &filter in filter_bounds.iter() {
                    let filter_start_x = (filter.x + x as i32 + dx) as usize;
                    let filter_start_y = (filter.y + y as i32 + dy) as usize;

                    let filter_size = filter.size as usize;

                    let py0 = filter_start_y * arena.width;
                    let py1 = (filter_start_y + 1) * arena.width;
                    let py2 = (filter_start_y + 2) * arena.width;
                    let py3 = (filter_start_y + 3) * arena.width;
                    let py4 = (filter_start_y + 4) * arena.width;
                    let py5 = (filter_start_y + 5) * arena.width;

                    let mut current_x = 0usize;

                    while current_x + 16 < filter_size {
                        let px = filter_start_x + current_x;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let base_offset_4 = py4 + px;
                        let base_offset_5 = py5 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = _mm_loadu_si128(current0.as_ptr() as *const __m128i);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let new_value1 = _mm_loadu_si128(current1.as_ptr() as *const __m128i);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let new_value2 = _mm_loadu_si128(current2.as_ptr() as *const __m128i);
                        let current3 = src.get_unchecked(base_offset_3..);
                        let new_value3 = _mm_loadu_si128(current3.as_ptr() as *const __m128i);
                        let current4 = src.get_unchecked(base_offset_4..);
                        let new_value4 = _mm_loadu_si128(current4.as_ptr() as *const __m128i);
                        let current5 = src.get_unchecked(base_offset_5..);
                        let new_value5 = _mm_loadu_si128(current5.as_ptr() as *const __m128i);

                        *items0.get_unchecked_mut(index_iter) = across_vec_decision(new_value0);
                        *items1.get_unchecked_mut(index_iter) = across_vec_decision(new_value1);
                        *items2.get_unchecked_mut(index_iter) = across_vec_decision(new_value2);
                        *items3.get_unchecked_mut(index_iter) = across_vec_decision(new_value3);
                        *items4.get_unchecked_mut(index_iter) = across_vec_decision(new_value4);
                        *items5.get_unchecked_mut(index_iter) = across_vec_decision(new_value5);

                        index_iter += 1;

                        current_x += 16;
                    }

                    while current_x + 8 < filter_size {
                        let px = filter_start_x + current_x;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let base_offset_4 = py4 + px;
                        let base_offset_5 = py5 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = _mm_or_si128(_mm_loadu_si64(current0.as_ptr()), upper_fix);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let new_value1 = _mm_or_si128(_mm_loadu_si64(current1.as_ptr()), upper_fix);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let new_value2 = _mm_or_si128(_mm_loadu_si64(current2.as_ptr()), upper_fix);
                        let current3 = src.get_unchecked(base_offset_3..);
                        let new_value3 = _mm_or_si128(_mm_loadu_si64(current3.as_ptr()), upper_fix);
                        let current4 = src.get_unchecked(base_offset_4..);
                        let new_value4 = _mm_or_si128(_mm_loadu_si64(current4.as_ptr()), upper_fix);
                        let current5 = src.get_unchecked(base_offset_5..);
                        let new_value5 = _mm_or_si128(_mm_loadu_si64(current5.as_ptr()), upper_fix);

                        *items0.get_unchecked_mut(index_iter) = across_vec_decision(new_value0);
                        *items1.get_unchecked_mut(index_iter) = across_vec_decision(new_value1);
                        *items2.get_unchecked_mut(index_iter) = across_vec_decision(new_value2);
                        *items3.get_unchecked_mut(index_iter) = across_vec_decision(new_value3);
                        *items4.get_unchecked_mut(index_iter) = across_vec_decision(new_value4);
                        *items5.get_unchecked_mut(index_iter) = across_vec_decision(new_value5);

                        index_iter += 1;

                        current_x += 8;
                    }

                    while current_x + 4 < filter_size {
                        let px = filter_start_x + current_x;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let base_offset_4 = py4 + px;
                        let base_offset_5 = py5 + px;

                        let current0 = src.get_unchecked(base_offset_0..);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let current3 = src.get_unchecked(base_offset_3..);
                        let current4 = src.get_unchecked(base_offset_4..);
                        let current5 = src.get_unchecked(base_offset_5..);
                        let new_value0 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current0.as_ptr() as *const i32).read_unaligned(),
                        );
                        let new_value1 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current1.as_ptr() as *const i32).read_unaligned(),
                        );
                        let new_value2 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current2.as_ptr() as *const i32).read_unaligned(),
                        );
                        let new_value3 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current3.as_ptr() as *const i32).read_unaligned(),
                        );
                        let new_value4 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current4.as_ptr() as *const i32).read_unaligned(),
                        );
                        let new_value5 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current5.as_ptr() as *const i32).read_unaligned(),
                        );

                        *items0.get_unchecked_mut(index_iter) = across_vec_decision(new_value0);
                        *items1.get_unchecked_mut(index_iter) = across_vec_decision(new_value1);
                        *items2.get_unchecked_mut(index_iter) = across_vec_decision(new_value2);
                        *items3.get_unchecked_mut(index_iter) = across_vec_decision(new_value3);
                        *items4.get_unchecked_mut(index_iter) = across_vec_decision(new_value4);
                        *items5.get_unchecked_mut(index_iter) = across_vec_decision(new_value5);

                        index_iter += 1;

                        current_x += 4;
                    }

                    while current_x < filter_size {
                        let px = filter_start_x + current_x;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let base_offset_4 = py4 + px;
                        let base_offset_5 = py5 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let current3 = src.get_unchecked(base_offset_3..);
                        let current4 = src.get_unchecked(base_offset_4..);
                        let current5 = src.get_unchecked(base_offset_5..);
                        *items0.get_unchecked_mut(index_iter) = *current0.get_unchecked(0);
                        *items1.get_unchecked_mut(index_iter) = *current1.get_unchecked(0);
                        *items2.get_unchecked_mut(index_iter) = *current2.get_unchecked(0);
                        *items3.get_unchecked_mut(index_iter) = *current3.get_unchecked(0);
                        *items4.get_unchecked_mut(index_iter) = *current4.get_unchecked(0);
                        *items5.get_unchecked_mut(index_iter) = *current5.get_unchecked(0);

                        index_iter += 1;

                        current_x += 1;
                    }
                }

                let ptr0 = (dst.slice.as_ptr() as *mut u8).add(y * stride + x);
                ptr0.write_unaligned(morph_vec_op(items0.get_unchecked(..index_iter)));
                let ptr1 = (dst.slice.as_ptr() as *mut u8).add((y + 1) * stride + x);
                ptr1.write_unaligned(morph_vec_op(items1.get_unchecked(..index_iter)));
                let ptr2 = (dst.slice.as_ptr() as *mut u8).add((y + 2) * stride + x);
                ptr2.write_unaligned(morph_vec_op(items2.get_unchecked(..index_iter)));
                let ptr3 = (dst.slice.as_ptr() as *mut u8).add((y + 3) * stride + x);
                ptr3.write_unaligned(morph_vec_op(items3.get_unchecked(..index_iter)));
                let ptr4 = (dst.slice.as_ptr() as *mut u8).add((y + 4) * stride + x);
                ptr4.write_unaligned(morph_vec_op(items4.get_unchecked(..index_iter)));
                let ptr5 = (dst.slice.as_ptr() as *mut u8).add((y + 5) * stride + x);
                ptr5.write_unaligned(morph_vec_op(items5.get_unchecked(..index_iter)));
            }
        } else {
            let max_width = width as i32 - 1;
            let max_height = height as i32 - 1;

            for x in 0..width {
                let filter_bounds = &analyzed_se.left_front.filter_bounds;

                let mut value0 = base_val;
                let mut value1 = base_val;
                let mut value2 = base_val;
                let mut value3 = base_val;
                let mut value4 = base_val;
                let mut value5 = base_val;

                for &filter in filter_bounds.iter() {
                    let filter_start_x = filter.x + x as i32;
                    let filter_start_y = filter.y + y as i32;
                    if (filter_start_y + 5) < 0 {
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
                    let py1 = (py + 1).min(max_height).max(0) as usize * stride;
                    let py2 = (py + 2).min(max_height).max(0) as usize * stride;
                    let py3 = (py + 3).min(max_height).max(0) as usize * stride;
                    let py4 = (py + 4).min(max_height).max(0) as usize * stride;
                    let py5 = (py + 5).min(max_height).max(0) as usize * stride;

                    let mut current_x = 0usize;

                    let mut values_0 = _mm_set1_epi8(value0 as i8);
                    let mut values_1 = _mm_set1_epi8(value1 as i8);
                    let mut values_2 = _mm_set1_epi8(value2 as i8);
                    let mut values_3 = _mm_set1_epi8(value3 as i8);
                    let mut values_4 = _mm_set1_epi8(value4 as i8);
                    let mut values_5 = _mm_set1_epi8(value4 as i8);

                    while current_x + 16 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let base_offset_4 = py4 + px;
                        let base_offset_5 = py5 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = _mm_loadu_si128(current0.as_ptr() as *const __m128i);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let new_value1 = _mm_loadu_si128(current1.as_ptr() as *const __m128i);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let new_value2 = _mm_loadu_si128(current2.as_ptr() as *const __m128i);
                        let current3 = src.get_unchecked(base_offset_3..);
                        let new_value3 = _mm_loadu_si128(current3.as_ptr() as *const __m128i);
                        let current4 = src.get_unchecked(base_offset_4..);
                        let new_value4 = _mm_loadu_si128(current4.as_ptr() as *const __m128i);
                        let current5 = src.get_unchecked(base_offset_5..);
                        let new_value5 = _mm_loadu_si128(current5.as_ptr() as *const __m128i);

                        values_0 = decision(new_value0, values_0);
                        values_1 = decision(new_value1, values_1);
                        values_2 = decision(new_value2, values_2);
                        values_3 = decision(new_value3, values_3);
                        values_4 = decision(new_value4, values_4);
                        values_5 = decision(new_value5, values_5);

                        current_x += 16;
                    }

                    while current_x + 8 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let base_offset_4 = py4 + px;
                        let base_offset_5 = py5 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = _mm_or_si128(_mm_loadu_si64(current0.as_ptr()), upper_fix);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let new_value1 = _mm_or_si128(_mm_loadu_si64(current1.as_ptr()), upper_fix);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let new_value2 = _mm_or_si128(_mm_loadu_si64(current2.as_ptr()), upper_fix);
                        let current3 = src.get_unchecked(base_offset_3..);
                        let new_value3 = _mm_or_si128(_mm_loadu_si64(current3.as_ptr()), upper_fix);
                        let current4 = src.get_unchecked(base_offset_4..);
                        let new_value4 = _mm_or_si128(_mm_loadu_si64(current4.as_ptr()), upper_fix);
                        let current5 = src.get_unchecked(base_offset_5..);
                        let new_value5 = _mm_or_si128(_mm_loadu_si64(current5.as_ptr()), upper_fix);

                        values_0 = decision(new_value0, values_0);
                        values_1 = decision(new_value1, values_1);
                        values_2 = decision(new_value2, values_2);
                        values_3 = decision(new_value3, values_3);
                        values_4 = decision(new_value4, values_4);
                        values_5 = decision(new_value5, values_5);

                        current_x += 8;
                    }

                    while current_x + 4 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let base_offset_4 = py4 + px;
                        let base_offset_5 = py5 + px;

                        let current0 = src.get_unchecked(base_offset_0..);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let current3 = src.get_unchecked(base_offset_3..);
                        let current4 = src.get_unchecked(base_offset_4..);
                        let current5 = src.get_unchecked(base_offset_5..);
                        let new_value0 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current0.as_ptr() as *const i32).read_unaligned(),
                        );
                        let new_value1 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current1.as_ptr() as *const i32).read_unaligned(),
                        );
                        let new_value2 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current2.as_ptr() as *const i32).read_unaligned(),
                        );
                        let new_value3 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current3.as_ptr() as *const i32).read_unaligned(),
                        );
                        let new_value4 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current4.as_ptr() as *const i32).read_unaligned(),
                        );
                        let new_value5 = _mm_insert_epi32::<0>(
                            _mm_set1_epi8(base_val as i8),
                            (current5.as_ptr() as *const i32).read_unaligned(),
                        );

                        values_0 = decision(new_value0, values_0);
                        values_1 = decision(new_value1, values_1);
                        values_2 = decision(new_value2, values_2);
                        values_3 = decision(new_value3, values_3);
                        values_4 = decision(new_value4, values_4);
                        values_5 = decision(new_value5, values_5);

                        current_x += 4;
                    }

                    value0 = across_vec_decision(values_0);
                    value1 = across_vec_decision(values_1);
                    value2 = across_vec_decision(values_2);
                    value3 = across_vec_decision(values_3);
                    value4 = across_vec_decision(values_4);
                    value5 = across_vec_decision(values_5);

                    while current_x < filter_size {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let base_offset_4 = py4 + px;
                        let base_offset_5 = py5 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let current3 = src.get_unchecked(base_offset_3..);
                        let current4 = src.get_unchecked(base_offset_4..);
                        let current5 = src.get_unchecked(base_offset_5..);
                        value0 = match op_type {
                            MorphOp::Dilate => value0.max(*current0.get_unchecked(0)),
                            MorphOp::Erode => value0.min(*current0.get_unchecked(0)),
                        };
                        value1 = match op_type {
                            MorphOp::Dilate => value1.max(*current1.get_unchecked(0)),
                            MorphOp::Erode => value1.min(*current1.get_unchecked(0)),
                        };
                        value2 = match op_type {
                            MorphOp::Dilate => value2.max(*current2.get_unchecked(0)),
                            MorphOp::Erode => value2.min(*current2.get_unchecked(0)),
                        };
                        value3 = match op_type {
                            MorphOp::Dilate => value3.max(*current3.get_unchecked(0)),
                            MorphOp::Erode => value3.min(*current3.get_unchecked(0)),
                        };
                        value4 = match op_type {
                            MorphOp::Dilate => value4.max(*current4.get_unchecked(0)),
                            MorphOp::Erode => value4.min(*current4.get_unchecked(0)),
                        };
                        value5 = match op_type {
                            MorphOp::Dilate => value5.max(*current5.get_unchecked(0)),
                            MorphOp::Erode => value5.min(*current5.get_unchecked(0)),
                        };

                        current_x += 1;
                    }
                }

                let ptr0 = (dst.slice.as_ptr() as *mut u8).add(y * stride + x);
                ptr0.write_unaligned(value0);
                let ptr1 = (dst.slice.as_ptr() as *mut u8).add((y + 1) * stride + x);
                ptr1.write_unaligned(value1);
                let ptr2 = (dst.slice.as_ptr() as *mut u8).add((y + 2) * stride + x);
                ptr2.write_unaligned(value2);
                let ptr3 = (dst.slice.as_ptr() as *mut u8).add((y + 3) * stride + x);
                ptr3.write_unaligned(value3);
                let ptr4 = (dst.slice.as_ptr() as *mut u8).add((y + 4) * stride + x);
                ptr4.write_unaligned(value4);
                let ptr5 = (dst.slice.as_ptr() as *mut u8).add((y + 5) * stride + x);
                ptr5.write_unaligned(value5);
            }
        }
    }
}
