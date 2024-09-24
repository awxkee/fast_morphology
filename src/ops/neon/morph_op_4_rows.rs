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
use crate::ops::neon::fast_morph_op_1d_neon;
use crate::se_scan::ScanPoint;
use crate::unsafe_slice::UnsafeSlice;
use crate::ImageSize;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;
#[cfg(target_arch = "arm")]
use std::arch::arm::*;

#[derive(Clone)]
pub struct MorphOpFilterNeon2D4Rows<const OP_TYPE: u8> {}

impl<const OP_TYPE: u8> Default for MorphOpFilterNeon2D4Rows<OP_TYPE> {
    fn default() -> Self {
        MorphOpFilterNeon2D4Rows {}
    }
}

impl<const OP_TYPE: u8> MorthOpFilterFlat2DRow for MorphOpFilterNeon2D4Rows<OP_TYPE> {
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

        let decision_16 = match op_type {
            MorphOp::Dilate => vmaxq_u8,
            MorphOp::Erode => vminq_u8,
        };

        let decision_8 = match op_type {
            MorphOp::Dilate => vmax_u8,
            MorphOp::Erode => vmin_u8,
        };

        let across_vec_decision = match op_type {
            MorphOp::Dilate => vmaxv_u8,
            MorphOp::Erode => vminv_u8,
        };

        let across_vec_decision_f = match op_type {
            MorphOp::Dilate => vmaxvq_u8,
            MorphOp::Erode => vminvq_u8,
        };

        let base_val = match op_type {
            MorphOp::Dilate => u8::MIN,
            MorphOp::Erode => u8::MAX,
        };

        if let Some(arena) = arena {
            let src = &arena.arena;

            let dx = arena.pad_w as i32;
            let dy = arena.pad_h as i32;

            let mut items0 = vec![base_val; analyzed_se.left_front.element_offsets.len()];
            let mut items1 = vec![base_val; analyzed_se.left_front.element_offsets.len()];
            let mut items2 = vec![base_val; analyzed_se.left_front.element_offsets.len()];
            let mut items3 = vec![base_val; analyzed_se.left_front.element_offsets.len()];
            let mut items4 = vec![base_val; analyzed_se.left_front.element_offsets.len()];
            let mut items5 = vec![base_val; analyzed_se.left_front.element_offsets.len()];

            let d_size = ScanPoint::new(dx, dy);
            let filter_bounds = analyzed_se
                .left_front
                .filter_bounds
                .iter()
                .map(|&x| x + d_size)
                .collect::<Vec<_>>();

            for x in 0..width {
                let mut index_iter = 0usize;

                for &filter in filter_bounds.iter() {
                    let filter_start_x = (filter.x + x as i32) as usize;
                    let filter_start_y = (filter.y + y as i32) as usize;

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
                        let new_value0 = vld1q_u8(current0.as_ptr());
                        let current1 = src.get_unchecked(base_offset_1..);
                        let new_value1 = vld1q_u8(current1.as_ptr());
                        let current2 = src.get_unchecked(base_offset_2..);
                        let new_value2 = vld1q_u8(current2.as_ptr());
                        let current3 = src.get_unchecked(base_offset_3..);
                        let new_value3 = vld1q_u8(current3.as_ptr());
                        let current4 = src.get_unchecked(base_offset_4..);
                        let new_value4 = vld1q_u8(current4.as_ptr());
                        let current5 = src.get_unchecked(base_offset_5..);
                        let new_value5 = vld1q_u8(current5.as_ptr());

                        *items0.get_unchecked_mut(index_iter) = across_vec_decision_f(new_value0);
                        *items1.get_unchecked_mut(index_iter) = across_vec_decision_f(new_value1);
                        *items2.get_unchecked_mut(index_iter) = across_vec_decision_f(new_value2);
                        *items3.get_unchecked_mut(index_iter) = across_vec_decision_f(new_value3);
                        *items4.get_unchecked_mut(index_iter) = across_vec_decision_f(new_value4);
                        *items5.get_unchecked_mut(index_iter) = across_vec_decision_f(new_value5);

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
                        let new_value0 = vld1_u8(current0.as_ptr());
                        let current1 = src.get_unchecked(base_offset_1..);
                        let new_value1 = vld1_u8(current1.as_ptr());
                        let current2 = src.get_unchecked(base_offset_2..);
                        let new_value2 = vld1_u8(current2.as_ptr());
                        let current3 = src.get_unchecked(base_offset_3..);
                        let new_value3 = vld1_u8(current3.as_ptr());
                        let current4 = src.get_unchecked(base_offset_4..);
                        let new_value4 = vld1_u8(current4.as_ptr());
                        let current5 = src.get_unchecked(base_offset_5..);
                        let new_value5 = vld1_u8(current5.as_ptr());

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
                        let new_value0 = vreinterpret_u8_u32(vset_lane_u32::<0>(
                            (current0.as_ptr() as *const u32).read_unaligned(),
                            vreinterpret_u32_u8(vdup_n_u8(base_val)),
                        ));
                        let new_value1 = vreinterpret_u8_u32(vset_lane_u32::<0>(
                            (current1.as_ptr() as *const u32).read_unaligned(),
                            vreinterpret_u32_u8(vdup_n_u8(base_val)),
                        ));
                        let new_value2 = vreinterpret_u8_u32(vset_lane_u32::<0>(
                            (current2.as_ptr() as *const u32).read_unaligned(),
                            vreinterpret_u32_u8(vdup_n_u8(base_val)),
                        ));
                        let new_value3 = vreinterpret_u8_u32(vset_lane_u32::<0>(
                            (current3.as_ptr() as *const u32).read_unaligned(),
                            vreinterpret_u32_u8(vdup_n_u8(base_val)),
                        ));
                        let new_value4 = vreinterpret_u8_u32(vset_lane_u32::<0>(
                            (current4.as_ptr() as *const u32).read_unaligned(),
                            vreinterpret_u32_u8(vdup_n_u8(base_val)),
                        ));
                        let new_value5 = vreinterpret_u8_u32(vset_lane_u32::<0>(
                            (current5.as_ptr() as *const u32).read_unaligned(),
                            vreinterpret_u32_u8(vdup_n_u8(base_val)),
                        ));

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
                ptr0.write_unaligned(fast_morph_op_1d_neon::<OP_TYPE>(
                    items0.get_unchecked(..index_iter),
                ));
                let ptr1 = (dst.slice.as_ptr() as *mut u8).add((y + 1) * stride + x);
                ptr1.write_unaligned(fast_morph_op_1d_neon::<OP_TYPE>(
                    items1.get_unchecked(..index_iter),
                ));
                let ptr2 = (dst.slice.as_ptr() as *mut u8).add((y + 2) * stride + x);
                ptr2.write_unaligned(fast_morph_op_1d_neon::<OP_TYPE>(
                    items2.get_unchecked(..index_iter),
                ));
                let ptr3 = (dst.slice.as_ptr() as *mut u8).add((y + 3) * stride + x);
                ptr3.write_unaligned(fast_morph_op_1d_neon::<OP_TYPE>(
                    items3.get_unchecked(..index_iter),
                ));
                let ptr4 = (dst.slice.as_ptr() as *mut u8).add((y + 4) * stride + x);
                ptr4.write_unaligned(fast_morph_op_1d_neon::<OP_TYPE>(
                    items4.get_unchecked(..index_iter),
                ));
                let ptr5 = (dst.slice.as_ptr() as *mut u8).add((y + 5) * stride + x);
                ptr5.write_unaligned(fast_morph_op_1d_neon::<OP_TYPE>(
                    items5.get_unchecked(..index_iter),
                ));
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

                    let mut values_16_0 = vdupq_n_u8(value0);
                    let mut values_16_1 = vdupq_n_u8(value1);
                    let mut values_16_2 = vdupq_n_u8(value2);
                    let mut values_16_3 = vdupq_n_u8(value3);
                    let mut values_16_4 = vdupq_n_u8(value4);
                    let mut values_16_5 = vdupq_n_u8(value4);

                    while current_x + 16 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let base_offset_4 = py4 + px;
                        let base_offset_5 = py5 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = vld1q_u8(current0.as_ptr());
                        let current1 = src.get_unchecked(base_offset_1..);
                        let new_value1 = vld1q_u8(current1.as_ptr());
                        let current2 = src.get_unchecked(base_offset_2..);
                        let new_value2 = vld1q_u8(current2.as_ptr());
                        let current3 = src.get_unchecked(base_offset_3..);
                        let new_value3 = vld1q_u8(current3.as_ptr());
                        let current4 = src.get_unchecked(base_offset_4..);
                        let new_value4 = vld1q_u8(current4.as_ptr());
                        let current5 = src.get_unchecked(base_offset_5..);
                        let new_value5 = vld1q_u8(current5.as_ptr());

                        values_16_0 = decision_16(new_value0, values_16_0);
                        values_16_1 = decision_16(new_value1, values_16_1);
                        values_16_2 = decision_16(new_value2, values_16_2);
                        values_16_3 = decision_16(new_value3, values_16_3);
                        values_16_4 = decision_16(new_value4, values_16_4);
                        values_16_5 = decision_16(new_value5, values_16_5);

                        current_x += 16;
                    }

                    let mut values0 =
                        decision_8(vget_low_u8(values_16_0), vget_high_u8(values_16_0));
                    let mut values1 =
                        decision_8(vget_low_u8(values_16_1), vget_high_u8(values_16_1));
                    let mut values2 =
                        decision_8(vget_low_u8(values_16_2), vget_high_u8(values_16_2));
                    let mut values3 =
                        decision_8(vget_low_u8(values_16_3), vget_high_u8(values_16_3));
                    let mut values4 =
                        decision_8(vget_low_u8(values_16_4), vget_high_u8(values_16_4));
                    let mut values5 =
                        decision_8(vget_low_u8(values_16_5), vget_high_u8(values_16_5));

                    while current_x + 8 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px = (filter_start_x + current_x as i32).min(max_width).max(0) as usize;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let base_offset_4 = py4 + px;
                        let base_offset_5 = py5 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let new_value0 = vld1_u8(current0.as_ptr());
                        let current1 = src.get_unchecked(base_offset_1..);
                        let new_value1 = vld1_u8(current1.as_ptr());
                        let current2 = src.get_unchecked(base_offset_2..);
                        let new_value2 = vld1_u8(current2.as_ptr());
                        let current3 = src.get_unchecked(base_offset_3..);
                        let new_value3 = vld1_u8(current3.as_ptr());
                        let current4 = src.get_unchecked(base_offset_4..);
                        let new_value4 = vld1_u8(current4.as_ptr());
                        let current5 = src.get_unchecked(base_offset_5..);
                        let new_value5 = vld1_u8(current5.as_ptr());

                        values0 = decision_8(new_value0, values0);
                        values1 = decision_8(new_value1, values1);
                        values2 = decision_8(new_value2, values2);
                        values3 = decision_8(new_value3, values3);
                        values4 = decision_8(new_value4, values4);
                        values5 = decision_8(new_value5, values5);

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
                        let new_value0 = vreinterpret_u8_u32(vset_lane_u32::<0>(
                            (current0.as_ptr() as *const u32).read_unaligned(),
                            vreinterpret_u32_u8(vdup_n_u8(base_val)),
                        ));
                        let new_value1 = vreinterpret_u8_u32(vset_lane_u32::<0>(
                            (current1.as_ptr() as *const u32).read_unaligned(),
                            vreinterpret_u32_u8(vdup_n_u8(base_val)),
                        ));
                        let new_value2 = vreinterpret_u8_u32(vset_lane_u32::<0>(
                            (current2.as_ptr() as *const u32).read_unaligned(),
                            vreinterpret_u32_u8(vdup_n_u8(base_val)),
                        ));
                        let new_value3 = vreinterpret_u8_u32(vset_lane_u32::<0>(
                            (current3.as_ptr() as *const u32).read_unaligned(),
                            vreinterpret_u32_u8(vdup_n_u8(base_val)),
                        ));
                        let new_value4 = vreinterpret_u8_u32(vset_lane_u32::<0>(
                            (current4.as_ptr() as *const u32).read_unaligned(),
                            vreinterpret_u32_u8(vdup_n_u8(base_val)),
                        ));
                        let new_value5 = vreinterpret_u8_u32(vset_lane_u32::<0>(
                            (current5.as_ptr() as *const u32).read_unaligned(),
                            vreinterpret_u32_u8(vdup_n_u8(base_val)),
                        ));

                        values0 = decision_8(new_value0, values0);
                        values1 = decision_8(new_value1, values1);
                        values2 = decision_8(new_value2, values2);
                        values3 = decision_8(new_value3, values3);
                        values4 = decision_8(new_value4, values4);
                        values5 = decision_8(new_value5, values5);

                        current_x += 4;
                    }

                    value0 = across_vec_decision(values0);
                    value1 = across_vec_decision(values1);
                    value2 = across_vec_decision(values2);
                    value3 = across_vec_decision(values3);
                    value4 = across_vec_decision(values4);
                    value5 = across_vec_decision(values5);

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
