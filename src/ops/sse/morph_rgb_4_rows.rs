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
use crate::ops::smart_allocator::SmartAllocator;
use crate::ops::sse::hminmax::{_mm_hmax_epu8, _mm_hmin_epu8};
use crate::ops::sse::op::make_morph_op_3d_sse;
use crate::ops::sse::v_load::{
    _mm_load_deinterleave_half_rgb, _mm_load_deinterleave_quart_rgb, _mm_load_deinterleave_rgb,
};
use crate::ops::sse::{_mm_load_pack_x1_5, _mm_load_pack_x3, _mm_store_pack_x1_5, _mm_store_pack_x3};
use crate::ops::utils::write_rgb_to_slice;
use crate::se_scan::ScanPoint;
use crate::unsafe_slice::UnsafeSlice;
use crate::ImageSize;
use colorutils_rs::Rgb;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[derive(Clone)]
pub struct MorphOpFilterRgbSse2D4Rows<const OP_TYPE: u8> {}

impl<const OP_TYPE: u8> Default for MorphOpFilterRgbSse2D4Rows<OP_TYPE> {
    fn default() -> Self {
        MorphOpFilterRgbSse2D4Rows {}
    }
}

impl<const OP_TYPE: u8> MorthOpFilterFlat2DRow for MorphOpFilterRgbSse2D4Rows<OP_TYPE> {
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
        let stride = width * 3;

        let base_val = match op_type {
            MorphOp::Dilate => u8::MIN,
            MorphOp::Erode => u8::MAX,
        };

        let decision = match op_type {
            MorphOp::Dilate => _mm_max_epu8,
            MorphOp::Erode => _mm_min_epu8,
        };

        let det_op = match op_type {
            MorphOp::Dilate => _mm_hmax_epu8,
            MorphOp::Erode => _mm_hmin_epu8,
        };

        if let Some(arena) = arena {
            let window_size = analyzed_se.left_front.element_offsets.len();
            let mut allocated_window_0 = SmartAllocator::new(Rgb::dup(base_val), window_size);
            let mut allocated_window_1 = SmartAllocator::new(Rgb::dup(base_val), window_size);
            let mut allocated_window_2 = SmartAllocator::new(Rgb::dup(base_val), window_size);
            let mut allocated_window_3 = SmartAllocator::new(Rgb::dup(base_val), window_size);

            let items0 = allocated_window_0.as_mut_slice();
            let items1 = allocated_window_1.as_mut_slice();
            let items2 = allocated_window_2.as_mut_slice();
            let items3 = allocated_window_3.as_mut_slice();

            let minmax_resolver = make_morph_op_3d_sse::<OP_TYPE>();

            let arena_stride = arena.width * 3;

            let src = &arena.arena;

            let dx = arena.pad_w as i32;
            let dy = arena.pad_h as i32;

            let d_size = ScanPoint::new(dx, dy);
            let prepared_kernel = analyzed_se
                .left_front
                .filter_bounds
                .iter()
                .map(|&x| x + d_size)
                .collect::<Vec<_>>();

            for x in 0..width {

                let mut iter_index = 0usize;

                for filter in prepared_kernel.iter() {
                    let filter_start_x = filter.x + x as i32;
                    let filter_start_y = filter.y + y as i32;

                    let filter_size = filter.size as usize;

                    let py = filter_start_y as usize;
                    let py0 = py * arena_stride;
                    let py1 = (py + 1) * arena_stride;
                    let py2 = (py + 2) * arena_stride;
                    let py3 = (py + 3) * arena_stride;

                    let mut current_x = 0usize;

                    while current_x + 16 < filter_size {
                        let px = (filter_start_x + current_x as i32) as usize * 3;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let current3 = src.get_unchecked(base_offset_3..);

                        let new_value0 = _mm_load_pack_x3(current0.as_ptr());
                        let new_value1 = _mm_load_pack_x3(current1.as_ptr());
                        let new_value2 = _mm_load_pack_x3(current2.as_ptr());
                        let new_value3 = _mm_load_pack_x3(current3.as_ptr());

                        _mm_store_pack_x3(
                            items0.get_unchecked_mut(iter_index..).as_mut_ptr() as *mut u8,
                            new_value0,
                        );
                        _mm_store_pack_x3(
                            items1.get_unchecked_mut(iter_index..).as_mut_ptr() as *mut u8,
                            new_value1,
                        );
                        _mm_store_pack_x3(
                            items2.get_unchecked_mut(iter_index..).as_mut_ptr() as *mut u8,
                            new_value2,
                        );
                        _mm_store_pack_x3(
                            items3.get_unchecked_mut(iter_index..).as_mut_ptr() as *mut u8,
                            new_value3,
                        );

                        iter_index += 16;

                        current_x += 16;
                    }

                    while current_x + 8 < filter_size {
                        let px = (filter_start_x + current_x as i32) as usize * 3;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let current3 = src.get_unchecked(base_offset_3..);

                        let new_value0 = _mm_load_pack_x1_5(current0.as_ptr());
                        let new_value1 = _mm_load_pack_x1_5(current1.as_ptr());
                        let new_value2 = _mm_load_pack_x1_5(current2.as_ptr());
                        let new_value3 = _mm_load_pack_x1_5(current3.as_ptr());

                        _mm_store_pack_x1_5(
                            items0.get_unchecked_mut(iter_index..).as_mut_ptr() as *mut u8,
                            new_value0,
                        );
                        _mm_store_pack_x1_5(
                            items1.get_unchecked_mut(iter_index..).as_mut_ptr() as *mut u8,
                            new_value1,
                        );
                        _mm_store_pack_x1_5(
                            items2.get_unchecked_mut(iter_index..).as_mut_ptr() as *mut u8,
                            new_value2,
                        );
                        _mm_store_pack_x1_5(
                            items3.get_unchecked_mut(iter_index..).as_mut_ptr() as *mut u8,
                            new_value3,
                        );
                        iter_index += 8;

                        current_x += 8;
                    }

                    while current_x < filter_size {
                        let px = (filter_start_x + current_x as i32) as usize * 3;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let current3 = src.get_unchecked(base_offset_3..);

                        *items0.get_unchecked_mut(iter_index) =
                            Rgb::new(current0[0], current0[1], current0[2]);
                        *items1.get_unchecked_mut(iter_index) =
                            Rgb::new(current1[0], current1[1], current1[2]);
                        *items2.get_unchecked_mut(iter_index) =
                            Rgb::new(current2[0], current2[1], current2[2]);
                        *items3.get_unchecked_mut(iter_index) =
                            Rgb::new(current3[0], current3[1], current3[2]);

                        iter_index += 1;

                        current_x += 1;
                    }
                }

                let px = x * 3;

                let rgb0 = minmax_resolver(items0.get_unchecked(..iter_index));
                let rgb1 = minmax_resolver(items1.get_unchecked(..iter_index));
                let rgb2 = minmax_resolver(items2.get_unchecked(..iter_index));
                let rgb3 = minmax_resolver(items3.get_unchecked(..iter_index));

                write_rgb_to_slice(dst, y * stride + px, rgb0);
                write_rgb_to_slice(dst, (y + 1) * stride + px, rgb1);
                write_rgb_to_slice(dst, (y + 2) * stride + px, rgb2);
                write_rgb_to_slice(dst, (y + 3) * stride + px, rgb3);
            }
        } else {
            let max_width = width as i32 - 1;
            let max_height = height as i32 - 1;

            for x in 0..width {
                let filter_bounds = &analyzed_se.left_front.filter_bounds;

                let mut rgb0 = Rgb::dup(base_val);
                let mut rgb1 = Rgb::dup(base_val);
                let mut rgb2 = Rgb::dup(base_val);
                let mut rgb3 = Rgb::dup(base_val);

                for filter in filter_bounds.iter() {
                    let filter_start_x = filter.x + x as i32;
                    let filter_start_y = filter.y + y as i32;
                    if (filter_start_y + 3) < 0 {
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

                    let mut current_x = 0usize;

                    let (mut values_r_0, mut values_r_1, mut values_r_2, mut values_r_3) = (
                        _mm_set1_epi8(rgb0.r as i8),
                        _mm_set1_epi8(rgb1.r as i8),
                        _mm_set1_epi8(rgb2.r as i8),
                        _mm_set1_epi8(rgb3.r as i8),
                    );
                    let (mut values_g_0, mut values_g_1, mut values_g_2, mut values_g_3) = (
                        _mm_set1_epi8(rgb0.g as i8),
                        _mm_set1_epi8(rgb1.g as i8),
                        _mm_set1_epi8(rgb2.g as i8),
                        _mm_set1_epi8(rgb3.g as i8),
                    );
                    let (mut values_b_0, mut values_b_1, mut values_b_2, mut values_b_3) = (
                        _mm_set1_epi8(rgb0.b as i8),
                        _mm_set1_epi8(rgb1.b as i8),
                        _mm_set1_epi8(rgb2.b as i8),
                        _mm_set1_epi8(rgb3.b as i8),
                    );

                    while current_x + 16 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px =
                            (filter_start_x + current_x as i32).min(max_width).max(0) as usize * 3;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let current3 = src.get_unchecked(base_offset_3..);
                        let new_value0 = _mm_load_deinterleave_rgb(current0.as_ptr());
                        let new_value1 = _mm_load_deinterleave_rgb(current1.as_ptr());
                        let new_value2 = _mm_load_deinterleave_rgb(current2.as_ptr());
                        let new_value3 = _mm_load_deinterleave_rgb(current3.as_ptr());

                        values_r_0 = decision(values_r_0, new_value0.0);
                        values_r_1 = decision(values_r_1, new_value1.0);
                        values_r_2 = decision(values_r_2, new_value2.0);
                        values_r_3 = decision(values_r_3, new_value3.0);

                        values_g_0 = decision(values_g_0, new_value0.1);
                        values_g_1 = decision(values_g_1, new_value1.1);
                        values_g_2 = decision(values_g_2, new_value2.1);
                        values_g_3 = decision(values_g_3, new_value3.1);

                        values_b_0 = decision(values_b_0, new_value0.2);
                        values_b_1 = decision(values_b_1, new_value1.2);
                        values_b_2 = decision(values_b_2, new_value2.2);
                        values_b_3 = decision(values_b_3, new_value3.2);

                        current_x += 16;
                    }

                    let (mut values_r8_0, mut values_r8_1, mut values_r8_2, mut values_r8_3) = (
                        _mm_set1_epi8(det_op(values_r_0) as i8),
                        _mm_set1_epi8(det_op(values_r_1) as i8),
                        _mm_set1_epi8(det_op(values_r_2) as i8),
                        _mm_set1_epi8(det_op(values_r_3) as i8),
                    );
                    let (mut values_g8_0, mut values_g8_1, mut values_g8_2, mut values_g8_3) = (
                        _mm_set1_epi8(det_op(values_g_0) as i8),
                        _mm_set1_epi8(det_op(values_g_1) as i8),
                        _mm_set1_epi8(det_op(values_g_2) as i8),
                        _mm_set1_epi8(det_op(values_g_3) as i8),
                    );
                    let (mut values_b8_0, mut values_b8_1, mut values_b8_2, mut values_b8_3) = (
                        _mm_set1_epi8(det_op(values_b_0) as i8),
                        _mm_set1_epi8(det_op(values_b_1) as i8),
                        _mm_set1_epi8(det_op(values_b_2) as i8),
                        _mm_set1_epi8(det_op(values_b_3) as i8),
                    );

                    while current_x + 8 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px =
                            (filter_start_x + current_x as i32).min(max_width).max(0) as usize * 3;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let current3 = src.get_unchecked(base_offset_3..);
                        let new_value0 =
                            _mm_load_deinterleave_half_rgb(current0.as_ptr(), base_val);
                        let new_value1 =
                            _mm_load_deinterleave_half_rgb(current1.as_ptr(), base_val);
                        let new_value2 =
                            _mm_load_deinterleave_half_rgb(current2.as_ptr(), base_val);
                        let new_value3 =
                            _mm_load_deinterleave_half_rgb(current3.as_ptr(), base_val);

                        values_r8_0 = decision(values_r8_0, new_value0.0);
                        values_r8_1 = decision(values_r8_1, new_value1.0);
                        values_r8_2 = decision(values_r8_2, new_value2.0);
                        values_r8_3 = decision(values_r8_3, new_value3.0);

                        values_g8_0 = decision(values_g8_0, new_value0.1);
                        values_g8_1 = decision(values_g8_1, new_value1.1);
                        values_g8_2 = decision(values_g8_2, new_value2.1);
                        values_g8_3 = decision(values_g8_3, new_value3.1);

                        values_b8_0 = decision(values_b8_0, new_value0.2);
                        values_b8_1 = decision(values_b8_1, new_value1.2);
                        values_b8_2 = decision(values_b8_2, new_value2.2);
                        values_b8_3 = decision(values_b8_3, new_value3.2);

                        current_x += 8;
                    }

                    let (mut values_r8_0, mut values_r8_1, mut values_r8_2, mut values_r8_3) = (
                        _mm_set1_epi8(det_op(values_r8_0) as i8),
                        _mm_set1_epi8(det_op(values_r8_1) as i8),
                        _mm_set1_epi8(det_op(values_r8_2) as i8),
                        _mm_set1_epi8(det_op(values_r8_3) as i8),
                    );
                    let (mut values_g8_0, mut values_g8_1, mut values_g8_2, mut values_g8_3) = (
                        _mm_set1_epi8(det_op(values_g8_0) as i8),
                        _mm_set1_epi8(det_op(values_g8_1) as i8),
                        _mm_set1_epi8(det_op(values_g8_2) as i8),
                        _mm_set1_epi8(det_op(values_g8_3) as i8),
                    );
                    let (mut values_b8_0, mut values_b8_1, mut values_b8_2, mut values_b8_3) = (
                        _mm_set1_epi8(det_op(values_b8_0) as i8),
                        _mm_set1_epi8(det_op(values_b8_1) as i8),
                        _mm_set1_epi8(det_op(values_b8_2) as i8),
                        _mm_set1_epi8(det_op(values_b8_3) as i8),
                    );

                    while current_x + 4 < filter_size && filter_start_x + current_x as i32 > 0 {
                        let px =
                            (filter_start_x + current_x as i32).min(max_width).max(0) as usize * 3;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let current3 = src.get_unchecked(base_offset_3..);

                        let new_value0 =
                            _mm_load_deinterleave_quart_rgb(current0.as_ptr(), base_val);
                        let new_value1 =
                            _mm_load_deinterleave_quart_rgb(current1.as_ptr(), base_val);
                        let new_value2 =
                            _mm_load_deinterleave_quart_rgb(current2.as_ptr(), base_val);
                        let new_value3 =
                            _mm_load_deinterleave_quart_rgb(current3.as_ptr(), base_val);

                        values_r8_0 = decision(values_r8_0, new_value0.0);
                        values_r8_1 = decision(values_r8_1, new_value1.0);
                        values_r8_2 = decision(values_r8_2, new_value2.0);
                        values_r8_3 = decision(values_r8_3, new_value3.0);

                        values_g8_0 = decision(values_g8_0, new_value0.1);
                        values_g8_1 = decision(values_g8_1, new_value1.1);
                        values_g8_2 = decision(values_g8_2, new_value2.1);
                        values_g8_3 = decision(values_g8_3, new_value3.1);

                        values_b8_0 = decision(values_b8_0, new_value0.2);
                        values_b8_1 = decision(values_b8_1, new_value1.2);
                        values_b8_2 = decision(values_b8_2, new_value2.2);
                        values_b8_3 = decision(values_b8_3, new_value3.2);

                        current_x += 4;
                    }

                    let mut values0 = _mm_setr_epi8(
                        det_op(values_r8_0) as i8,
                        det_op(values_g8_0) as i8,
                        det_op(values_b8_0) as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                    );
                    let mut values1 = _mm_setr_epi8(
                        det_op(values_r8_1) as i8,
                        det_op(values_g8_1) as i8,
                        det_op(values_b8_1) as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                    );
                    let mut values2 = _mm_setr_epi8(
                        det_op(values_r8_2) as i8,
                        det_op(values_g8_2) as i8,
                        det_op(values_b8_2) as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                    );
                    let mut values3 = _mm_setr_epi8(
                        det_op(values_r8_3) as i8,
                        det_op(values_g8_3) as i8,
                        det_op(values_b8_3) as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                        base_val as i8,
                    );

                    while current_x < filter_size {
                        let px =
                            (filter_start_x + current_x as i32).min(max_width).max(0) as usize * 3;
                        let base_offset_0 = py0 + px;
                        let base_offset_1 = py1 + px;
                        let base_offset_2 = py2 + px;
                        let base_offset_3 = py3 + px;
                        let current0 = src.get_unchecked(base_offset_0..);
                        let current1 = src.get_unchecked(base_offset_1..);
                        let current2 = src.get_unchecked(base_offset_2..);
                        let current3 = src.get_unchecked(base_offset_3..);

                        let new_value0 = _mm_setr_epi8(
                            current0[0] as i8,
                            current0[1] as i8,
                            current0[2] as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                        );
                        let new_value1 = _mm_setr_epi8(
                            current1[0] as i8,
                            current1[1] as i8,
                            current1[2] as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                        );
                        let new_value2 = _mm_setr_epi8(
                            current2[0] as i8,
                            current2[1] as i8,
                            current2[2] as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                        );
                        let new_value3 = _mm_setr_epi8(
                            current3[0] as i8,
                            current3[1] as i8,
                            current3[2] as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                            base_val as i8,
                        );

                        values0 = decision(values0, new_value0);
                        values1 = decision(values1, new_value1);
                        values2 = decision(values2, new_value2);
                        values3 = decision(values3, new_value3);

                        current_x += 1;
                    }

                    let values0 = _mm_extract_epi32::<0>(values0).to_le_bytes();
                    let values1 = _mm_extract_epi32::<0>(values1).to_le_bytes();
                    let values2 = _mm_extract_epi32::<0>(values2).to_le_bytes();
                    let values3 = _mm_extract_epi32::<0>(values3).to_le_bytes();

                    rgb0 = Rgb::new(values0[0], values0[1], values0[2]);
                    rgb1 = Rgb::new(values1[0], values1[1], values1[2]);
                    rgb2 = Rgb::new(values2[0], values2[1], values2[2]);
                    rgb3 = Rgb::new(values3[0], values3[1], values3[2]);
                }

                let px = x * 3;

                write_rgb_to_slice(dst, y * stride + px, rgb0);
                write_rgb_to_slice(dst, (y + 1) * stride + px, rgb1);
                write_rgb_to_slice(dst, (y + 2) * stride + px, rgb2);
                write_rgb_to_slice(dst, (y + 3) * stride + px, rgb3);
            }
        }
    }
}
